# Verification internals

> **Status:** draft (v0 plan; details will harden as the translator is built)
> **Audience:** contributors working on `loom-compile-dafny`, `loom-verify`, or the verifier abstraction layer.

This document describes how Loom umbrellas translate to Dafny for verification, how Dafny's results are read back into Loom's vocabulary, and what assumptions the translation makes. The companion document [`docs/bidirectional-refinement.md`](bidirectional-refinement.md) describes what the verifier's output is used for after this translation completes.

---

## 1. The translation contract

The verifier abstraction (`crate trait Verifier`) requires:

```rust
pub trait Verifier {
    fn translate(&self, umbrella: &Umbrella) -> Result<String, TranslationError>;
    fn verify(&self, translated: &str) -> Result<VerificationReport, VerifierError>;
    fn name(&self) -> &'static str;
    fn version(&self) -> &str;
}
```

Two key properties:

1. **Totality of translation.** Every well-formed umbrella (passes `loom check`) translates to syntactically valid Dafny. Translation failures are bugs in the translator, not user errors. User errors are caught earlier by the checker.

2. **Soundness of translation.** If Dafny verifies the translated program, the corresponding Loom claims hold. The translation does not weaken claims; it preserves their logical content.

The second property requires care. Some translation choices preserve claims trivially (a straight `int` becomes Dafny's `int`); others require nontrivial encoding (`relates` operations with pre- and postconditions become Dafny methods or functions with specifications). The translator's correctness depends on these encodings being correct.

---

## 2. Verification-direction translation (Loom → Dafny for verification)

### 2.1 `knows` → Dafny type declarations

**Refinement types.**
```loom
knows {
  Money :: {x: int | x >= 0}
}
```
becomes
```dafny
type Money = x: int | x >= 0 witness 0
```

Dafny requires a *witness* — a concrete value that satisfies the refinement — to ensure the type is inhabited. The translator chooses a witness:
- For integer-bounded types, the boundary (`0` for non-negative, `1` for positive, `min` for ranged).
- For predicate-restricted types, an SMT call to find a witness if a heuristic fails. (Fallback only; usually a constant works.)

If no witness is findable, the translator emits a diagnostic and aborts. This is rare and indicates an unsatisfiable refinement, which is a checker-catchable error.

**Records.**
```loom
knows {
  Account :: {id: AccountId, balance: Money, open: bool}
}
```
becomes
```dafny
datatype Account = Account(id: AccountId, balance: Money, open: bool)
```

Dafny's `datatype` provides immutable, structurally-equal records. Field access is `a.id`, `a.balance`, etc., matching Loom's syntax.

**Sum types.**
```loom
knows {
  TransferResult :: 
    | Success {transfer: Transfer, new_balance: Money}
    | InsufficientFunds {account: AccountId, requested: Money, available: Money}
    | AccountClosed {account: AccountId}
}
```
becomes
```dafny
datatype TransferResult =
  | Success(transfer: Transfer, new_balance: Money)
  | InsufficientFunds(account: AccountId, requested: Money, available: Money)
  | AccountClosed(account: AccountId)
```

Pattern matching translates one-for-one between `match ... with` and Dafny's `match`.

**Predicates.**
```loom
knows {
  pred is_solvent(a: Account) = a.balance > 0
}
```
becomes
```dafny
predicate is_solvent(a: Account) {
  a.balance > 0
}
```

**Constants.**
```loom
knows {
  const MAX_TRANSFER: Money = 1_000_000
}
```
becomes
```dafny
const MAX_TRANSFER: Money := 1000000
```

### 2.2 `relates` → Dafny function or method signatures

Each `relates` entry becomes a Dafny method (for operations that may have side effects or non-trivial computation) or a function (for pure operations). In v0, all Loom operations are pure, so they all become Dafny `function method`s.

```loom
relates {
  open_account(id: AccountId, initial: Money) -> Account
    requires { initial >= 0 }
    ensures { 
      result.id = id, 
      result.balance = initial, 
      result.open = true 
    }
}
```
becomes
```dafny
function method open_account(id: AccountId, initial: Money): Account
  requires initial >= 0
  ensures open_account(id, initial).id == id
  ensures open_account(id, initial).balance == initial
  ensures open_account(id, initial).open == true
```

Note: Loom uses `result` to refer to the operation's output. In Dafny functions, the output is not a name but the function application itself. The translator rewrites `result.field` to `funcname(args).field`.

**Tuple returns.** Loom's tuple-returning operations require encoding:

```loom
relates {
  transfer(from: Account, to: Account, amount: PositiveAmount) -> (Account, Account)
    ensures {
      let (from', to') = result;
      from'.id = from.id, to'.id = to.id
    }
}
```

Dafny does not have first-class tuples in the same way; the translator introduces a synthetic datatype for the tuple:
```dafny
datatype TransferResult2 = Pair(_0: Account, _1: Account)

function method transfer(from: Account, to: Account, amount: PositiveAmount): TransferResult2
  ensures transfer(from, to, amount)._0.id == from.id
  ensures transfer(from, to, amount)._1.id == to.id
```

The Loom-level destructuring `let (from', to') = result` becomes Dafny's `result._0` and `result._1` references in the translated ensures clauses.

### 2.3 `proves` → Dafny lemmas

Each `proves` entry becomes a Dafny lemma:

```loom
proves {
  conservation:
    for-all from: Account, to: Account, amount: PositiveAmount,
      from.open and to.open and from.balance >= amount =>
        let (from', to') = transfer(from, to, amount);
        from.balance + to.balance = from'.balance + to'.balance
}
```
becomes
```dafny
lemma conservation(from: Account, to: Account, amount: PositiveAmount)
  requires from.open && to.open && from.balance >= amount
  ensures 
    var result := transfer(from, to, amount);
    from.balance + to.balance == result._0.balance + result._1.balance
{}
```

The lemma's body is empty: Dafny's SMT-backed verification attempts to prove the postcondition without explicit proof steps. If the proof requires hints (induction, case splits), Dafny reports the proof obligation it cannot discharge; the translator passes this back as a category-(B) finding with the proof obligation as the reason.

For some properties, Dafny's auto-discharge succeeds. For others, hints are needed. v0 does not generate hints; unhinted lemmas that fail are reported as `Status: verifier limitation, may require proof hints`. Adding hints (or supporting them in Loom syntax) is post-v0 work.

### 2.4 `does` → Dafny function bodies

Each `does` entry is the body of the corresponding `relates` function:

```loom
does {
  open_account(id: AccountId, initial: Money) -> Account {
    {id: id, balance: initial, open: true}
  }
}
```
becomes
```dafny
function method open_account(id: AccountId, initial: Money): Account
  requires initial >= 0
  ensures /* ... from relates ... */
{
  Account(id, initial, true)
}
```

The `relates` declaration's spec and the `does` body merge into a single Dafny function with both spec and body.

Loom's `with`-update syntax:
```loom
does {
  transfer(from: Account, to: Account, amount: PositiveAmount) -> (Account, Account) {
    let from' = from with {balance: from.balance - amount};
    let to' = to with {balance: to.balance + amount};
    (from', to')
  }
}
```
becomes
```dafny
function method transfer(from: Account, to: Account, amount: PositiveAmount): TransferResult2
  /* ...spec... */
{
  var from' := from.(balance := from.balance - amount);
  var to' := to.(balance := to.balance + amount);
  Pair(from', to')
}
```

Dafny's `.(field := value)` is record-update syntax. The translation is mechanical.

### 2.5 `shows` → Dafny test methods

Each `shows` example becomes a test method:

```loom
shows {
  transfer_succeeds:
    transfer(
      {id: "alice", balance: 100, open: true},
      {id: "bob", balance: 0, open: true},
      30
    )
    -> ({id: "alice", balance: 70, open: true},
        {id: "bob", balance: 30, open: true})
}
```
becomes
```dafny
method {:test} transfer_succeeds() {
  var input_from := Account("alice", 100, true);
  var input_to := Account("bob", 0, true);
  var expected := Pair(Account("alice", 70, true), Account("bob", 30, true));
  expect transfer(input_from, input_to, 30) == expected;
}
```

The `expect` statement is Dafny's runtime check. Test methods are discharged by Dafny's `dafny test` command. Examples serve both as additional verification (the example's expected output must be consistent with the function's spec) and as runtime checks (the implementation actually produces the expected output).

---

## 3. Execution-direction translation (Loom → Python for execution)

The verification-direction translation is for Dafny. The execution-direction translation produces the runnable target code (Python in v0).

### 3.1 Types

Loom types map to Python type hints:

| Loom | Python |
|---|---|
| `int`, `nat` | `int` |
| `real` | `float` (or `decimal.Decimal` for precision-critical) |
| `bool` | `bool` |
| `string` | `str` |
| `unit` | `None` |
| `List<T>` | `list[T]` |
| `Set<T>` | `frozenset[T]` |
| `Map<K, V>` | `dict[K, V]` (treated immutably) |

Refinement types degrade to base types with optional runtime assertions:

```loom
Money :: {x: int | x >= 0}
```
becomes (Python):
```python
Money = int  # refinement: x >= 0
```
with an optional runtime check at construction sites if `--with-runtime-asserts` is set.

Records become dataclasses:

```loom
Account :: {id: AccountId, balance: Money, open: bool}
```
becomes:
```python
from dataclasses import dataclass

@dataclass(frozen=True)
class Account:
    id: AccountId
    balance: Money
    open: bool
```

Sum types become tagged unions:

```python
@dataclass(frozen=True)
class Success:
    transfer: Transfer
    new_balance: Money

@dataclass(frozen=True)
class InsufficientFunds:
    account: AccountId
    requested: Money
    available: Money

@dataclass(frozen=True)
class AccountClosed:
    account: AccountId

TransferResult = Success | InsufficientFunds | AccountClosed
```

### 3.2 Operations

`relates` + `does` produces a Python function:

```loom
relates { open_account(id: AccountId, initial: Money) -> Account ... }
does { open_account(id, initial) { ... } }
```
becomes:
```python
def open_account(id: AccountId, initial: Money) -> Account:
    return Account(id=id, balance=initial, open=True)
```

Pre/postconditions become runtime assertions only if `--with-runtime-asserts` is set. By default, they are documentation comments. The verifier has already established the conditions hold; runtime re-checking is for defense-in-depth in production, not v0.

### 3.3 Examples become tests

`shows` examples become pytest test functions:

```loom
shows {
  transfer_succeeds:
    transfer({id: "alice", balance: 100, open: true}, ..., 30) -> (...)
}
```
becomes:
```python
def test_transfer_succeeds():
    input_from = Account(id="alice", balance=100, open=True)
    input_to = Account(id="bob", balance=0, open=True)
    expected = (
        Account(id="alice", balance=70, open=True),
        Account(id="bob", balance=30, open=True),
    )
    assert transfer(input_from, input_to, 30) == expected
```

Tests live in a parallel `tests/` directory in the generated package and run under pytest.

### 3.4 Module structure

A Loom file `examples/ledger.lm` with module `ledger` produces:

```
generated/
└── ledger/
    ├── __init__.py
    ├── types.py        # from knows
    ├── operations.py   # from relates + does
    └── tests/
        └── test_examples.py    # from shows
```

The package is plain Python, importable from any project that depends on it. No Loom runtime is required.

---

## 4. Reading verifier output

After translating and invoking Dafny, the orchestrator (`loom-verify`) reads Dafny's output and produces a Loom-level report.

### 4.1 Dafny output forms

Dafny's CLI produces:
- Exit code 0 if all proof obligations discharge.
- Exit code 4 if any obligation fails to discharge.
- Stdout with one line per failed obligation: `Verification of 'name' failed.`
- Stderr with diagnostic details: counterexamples (when available), proof obligation that could not be discharged, source location in the translated Dafny.

### 4.2 Mapping back to Loom

The translator maintains a *source map* during translation: each Dafny construct records the originating Loom AST node. When Dafny reports a failure at, e.g., `line 47 column 3`, the source map maps this to the Loom file and line.

The source map is structured:

```rust
struct SourceMap {
    dafny_line_to_loom_span: HashMap<usize, LoomSpan>,
    dafny_function_to_loom_operation: HashMap<String, OperationId>,
    dafny_lemma_to_loom_proof: HashMap<String, ProofId>,
}
```

Each verifier-reported failure walks the source map back to the originating Loom construct. The Loom-level diagnostic includes both the original Loom span and (in verbose mode) the corresponding Dafny excerpt.

### 4.3 Counterexamples

When Dafny produces a counterexample (a concrete assignment that falsifies the obligation), the orchestrator translates it back into Loom-level values:

```
Loom-level counterexample for 'no_overdrafts':
  from = Account{id: "x", balance: 5, open: true}
  to = Account{id: "y", balance: 0, open: true}
  amount = 5
  
  After transfer:
    from'.balance = 0 (expected >= 0 ✓)
  
  But the lemma's antecedent required from.balance > amount;
  this counterexample has from.balance = amount, which is the
  edge case the lemma's antecedent does not cover.
  
  Suggestion: relax the antecedent to from.balance >= amount
  if the equal-case is intended to be covered.
```

Counterexample translation requires the source map plus a value-level decoder. This is one of the more delicate parts of the translator and the most user-facing.

---

## 5. Translation invariants

Properties the translator must maintain:

1. **Type-preserving.** A well-typed Loom expression translates to a well-typed Dafny expression.
2. **Refinement-preserving.** A Loom refinement type's predicate is exactly its Dafny refinement.
3. **Spec-preserving.** A `relates` precondition translates to a Dafny `requires`; a `relates` postcondition translates to a Dafny `ensures`. No conditions are added, removed, or weakened.
4. **Body-preserving.** A `does` body translates to a Dafny function body whose semantics match. Implementation details (variable names, internal structure) may differ; the function's input-output behavior must be identical.
5. **Diagnostic-preserving.** Every Dafny construct in the output is annotated with its source location in the Loom file. No translator-introduced construct is unattributed.

These invariants are tested via integration tests in `crates/loom-compile-dafny/tests/`. Each test takes a Loom file, translates it, runs Dafny, and checks both the result and the source-map round-trip.

---

## 6. Known translation limitations

These limitations are documented so users understand when verification may not behave as expected.

### 6.1 Higher-order functions

Loom does not permit functions as first-class values in `does` (v0). Higher-order *claims* in `proves` (e.g., `for-all f, ...`) are supported syntactically but their translation depends on Dafny's support for higher-order quantification, which is limited. Most useful higher-order properties can be restated as universal claims over concrete functions.

### 6.2 Recursion

Loom's `does` permits recursive definitions. Dafny requires termination proofs for recursive functions. The translator emits a default `decreases` clause based on a heuristic (typically the size of the first parameter). If the heuristic is wrong, verification fails with a termination error and the user must add an explicit `decreases` annotation in the Loom source (syntax to be designed for v0.x).

### 6.3 Mutable state

There is no mutable state in Loom v0. The actor model and supervised runtime, deferred per §5.1 of the plan, would require introducing state and a state-modeling encoding in Dafny. v0 sidesteps this entirely.

### 6.4 Effects

Effect annotations (`@net`, `@db`) are parsed but not enforced. They do not appear in the Dafny translation. Capability-tracked verification is post-v0.

### 6.5 Set and Map operations

Dafny's `set<T>` and `map<K, V>` cover most operations. Some Loom-level operations (e.g., `Map.filter`) may not have direct Dafny equivalents and require translator-emitted lemmas to express. These are tracked as they arise; for v0 the example corpus avoids them where possible.

---

## 7. Performance considerations

### 7.1 Translation time

Translation is O(n) in umbrella size. No optimizations are required for v0 scale (umbrellas of hundreds of claims).

### 7.2 Verification time

Verification time is dominated by SMT calls. Dafny invokes Z3 per proof obligation. Typical small umbrellas verify in seconds; complex ones can take minutes. Beyond a few minutes per claim, the verification is likely failing for a reason other than time (verifier limitation, missing hint).

### 7.3 Caching

The verifier orchestrator caches verification results by `(umbrella_hash, dafny_version, z3_version)`. Re-runs with unchanged umbrellas hit the cache. The cache is in `.loom-cache/` at the project root and is excluded from git.

---

## 8. Switching backends

The translation is encapsulated in `crates/loom-compile-dafny`. Switching to F* would require a parallel crate `crates/loom-compile-fstar` implementing the same `Verifier` trait. The Loom AST does not change; the source map structure does not change; the orchestrator does not change.

The Dafny-specific encodings (tuple workaround, `with`-update syntax, `decreases` heuristic) are unique to Dafny. F* has different equivalents. The encoding choices are confined to the per-backend crate.

Adding a second backend before v0 ships is out of scope. The trait-based abstraction is included from the start because retrofitting it later is significantly harder than designing for it from the start, even with only one implementation in v0.

---

## 9. Open questions

Tracked as ADRs:

- ADR-0014 — pinning Dafny version and SMT solver version.
- ADR-0015 (post-v0) — adding hint syntax to Loom for proofs Dafny cannot auto-discharge.
- ADR-0016 (post-v0) — encoding effects and capabilities into the verifier model.

---

## 10. References

- [`docs/language-reference.md`](language-reference.md) — Loom surface syntax.
- [`docs/claims-reference.md`](claims-reference.md) — what each register means.
- [`docs/bidirectional-refinement.md`](bidirectional-refinement.md) — what happens with the verifier's results.
- Dafny reference manual: https://dafny.org/dafny/DafnyRef/DafnyRef
- The `Verifier` trait: `crates/loom-verify/src/verifier_trait.rs`.
