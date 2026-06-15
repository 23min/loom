# Claims reference

> **Status:** draft
> **Audience:** umbrella authors. Documents the *meaning* of each register and the catalog of claim forms.
> **Companion:** [`docs/language-reference.md`](language-reference.md) (syntax).

The five registers — `knows`, `relates`, `shows`, `does`, `proves` — are the umbrella's structural skeleton. Each register has a specific role and a small catalog of claim forms. The cross-register coverage rules link them: every type used must appear in `knows`; every operation in `relates` should have at least one example in `shows` and at least one property in `proves`; every operation in `relates` must have an implementation in `does`.

This document describes each register in turn, the claim forms it admits, and the patterns that recur in umbrella authoring.

---

## 1. Why five registers

The five registers exist to make different parts of an umbrella's content *visible as different parts*. Without the structure, claims about types, operations, examples, implementations, and properties bleed into each other. With the structure, a reviewer can ask focused questions: are the types right? are the operations specified completely? are the examples well-chosen? does the implementation track the specification? are there enough properties to constrain the implementation?

The registers are not mutually exclusive in content (a claim about an operation may mention types defined in `knows`), but they are mutually exclusive in *role*. Each register answers one question:

| Register | Question |
|---|---|
| `knows` | What does the umbrella have words for? |
| `relates` | What operations exist, and what do they promise? |
| `shows` | What does correct behavior look like in concrete cases? |
| `does` | How are the operations implemented? |
| `proves` | What is true for all cases, not just the examples? |

A reviewer can ask any of these in isolation. The author can write or revise any in isolation. The LLM can be prompted register-by-register. The cross-register coverage rules ensure that addressing one register without the others leaves a visible gap.

---

## 2. `knows` — the vocabulary

### 2.1 Role

`knows` declares everything the rest of the umbrella will refer to: types, predicates, and constants. Nothing in `relates`, `shows`, `does`, or `proves` may reference a name that is not in `knows` (or imported from another umbrella).

`knows` is the umbrella's *vocabulary*. It is the smallest register and often the slowest to evolve. Type definitions in `knows` are the ground truth for what kinds of things the umbrella reasons about.

### 2.2 Forms

**Type definitions.**
```loom
knows {
  Money :: {x: int | x >= 0}
  AccountId :: {s: string | s.length > 0 and s.length <= 64}
  Account :: {id: AccountId, balance: Money, open: bool}
}
```

A type definition has the form `Name :: type-expression`. The type expression may be a refinement of a built-in type, a record, a sum type (variants), or a parameterized type like `List<T>`.

**Predicate definitions.**
```loom
knows {
  pred is_solvent(a: Account) = a.balance > 0
  pred can_receive(a: Account) = a.open
  pred valid_transfer(t: Transfer, ledger: Map<AccountId, Account>) =
    ledger.contains(t.from) and
    ledger.contains(t.to) and
    is_solvent(ledger[t.from]) and
    can_receive(ledger[t.to]) and
    ledger[t.from].balance >= t.amount
}
```

Predicates are named boolean functions over typed inputs. They factor common boolean expressions out of `relates` clauses and `proves` bodies.

**Constants.**
```loom
knows {
  const MAX_TRANSFER: Money = 1_000_000
  const SUPPORTED_CURRENCIES: Set<string> = {"USD", "EUR", "GBP"}
}
```

Constants are named, typed values that appear in claims. They make magic numbers explicit and reviewable.

### 2.3 Patterns

**Refinement types over base types.** Most domain types are refinements of `int`, `string`, or records. `Money :: {x: int | x >= 0}` rather than just `int` makes non-negativity part of the type, not an obligation on every operation.

**Predicates that name important conditions.** When a precondition like `a.balance > 0 and a.open` appears in multiple places, name it once in `knows` and reference the name. Reviewers can then ask whether the named predicate captures the intended condition, separately from asking whether each use site is right.

**Constants for thresholds.** Any number that appears in multiple operations or claims should be a constant in `knows`. The constant's name documents what the number means.

### 2.4 Anti-patterns

**Empty refinements.** `Money :: {x: int | true}` provides no constraint beyond `int`. Either remove the refinement or add a meaningful predicate. `loom check` flags trivial refinements at warning level.

**Predicates that match everything.** `pred is_valid(a: Account) = true` is decorative. Either constrain it or remove it. `loom check` flags this.

**Untyped constants.** `const MAX = 1_000_000` without a type leaves the type to inference, which may produce surprises. Always annotate.

---

## 3. `relates` — the operations

### 3.1 Role

`relates` declares the operations the umbrella offers, with their pre- and postconditions. A `relates` entry says: *this operation exists; under this precondition, it returns a value with this postcondition*.

`relates` is the umbrella's *interface*. It tells callers what they can rely on (the postconditions) and what they must arrange before calling (the preconditions). The `does` register implements these signatures; the `proves` register makes additional guarantees about them.

### 3.2 Forms

**Operation signature.**
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

The signature includes parameter types, return type, optional precondition (`requires`), and optional postcondition (`ensures`). Multiple clauses in `requires` or `ensures` are conjoined.

**Operations with effects.**
```loom
relates {
  publish(msg: Message) -> unit @net
    requires { msg.is_well_formed }
}
```

Effect annotations (`@net`, `@db`, etc.) are syntactically part of the signature. v0 parses them but does not enforce capability discipline.

**Operations with multiple return values.**
```loom
relates {
  transfer(from: Account, to: Account, amount: PositiveAmount) -> (Account, Account)
    requires { from.open, to.open, from.balance >= amount }
    ensures {
      let (from', to') = result;
      from'.balance = from.balance - amount,
      to'.balance = to.balance + amount,
      from'.id = from.id, 
      to'.id = to.id,
      from'.open = from.open, 
      to'.open = to.open,
    }
}
```

Tuple returns destructure naturally; reference fields of each element of `result`.

### 3.3 Patterns

**Specify the postcondition over the return value, not the parameters.** `ensures result.balance = ...` is the right shape; the postcondition is about what the operation produces, not about what was passed in.

**Connect inputs to outputs explicitly.** `result.id = id` (in `open_account`) makes the relationship explicit. Without it, the postcondition could be satisfied by an operation that returns an arbitrary account; the verifier has no way to know the result's id must match the input.

**Cluster preconditions in `requires`; cluster postconditions in `ensures`.** Splitting a precondition across multiple `requires` blocks is allowed (they conjoin) but reduces readability. One `requires` per operation when feasible.

### 3.4 Anti-patterns

**Postconditions that only mention inputs.** `ensures from.open = true` (in transfer) does not constrain the result. Postconditions must mention something about the operation's output (the `result` keyword, fields thereof, or post-state expressions).

**Postconditions covering every possible outcome.** `ensures result.balance >= 0 or result.balance < 0 or result.balance = 0` is a disjunction over everything; it asserts nothing. The grammar bans this pattern; `loom check` rejects.

**Vacuous antecedents in conditional postconditions.** `ensures when (x > 0 and x < 0) then P` is unsatisfiable; the implication is vacuously true and asserts nothing.

---

## 4. `shows` — the examples

### 4.1 Role

`shows` provides concrete examples of operations. Each example is a *runnable test*: given specific inputs, the operation should produce a specific output. Examples serve two purposes: as a sanity-check against the implementation (do the examples actually pass?) and as a *concretization* of the operation for readers who want to see what it does without parsing the postconditions.

`shows` is the umbrella's *demonstration*. The examples should be small enough to grasp, varied enough to cover the operation's territory, and connected enough to the operation's claims that a reader sees the pattern.

### 4.2 Forms

**Basic example.**
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

The shape: `name: operation(args) -> expected_result`. The name is used in diagnostics ("example `transfer_succeeds` failed").

**Boundary example.**
```loom
shows {
  transfer_exact_balance:
    transfer(
      {id: "alice", balance: 50, open: true},
      {id: "bob", balance: 0, open: true},
      50
    )
    -> ({id: "alice", balance: 0, open: true},
        {id: "bob", balance: 50, open: true})
}
```

By convention, examples that engage boundaries (zero, max, just-above-min) carry descriptive names like `*_boundary` or `*_exact_*`.

**Negative-space example (post-v0).** In v0, all examples are positive: input + expected output. The post-v0 form will include `does not satisfy: operation(args)` for negative examples, documenting what the operation should *not* permit.

### 4.3 Patterns

**Cover the operation's interesting input regions.** Trivial inputs (everything zero), normal inputs (typical case), boundary inputs (max, min, just-inside-bounds). The companion paper's §5.3 (domain engagement) measures this.

**Use distinct names for distinct cases.** `transfer_succeeds` for the normal case, `transfer_exact_balance` for the boundary case, `transfer_min_amount` for the small-input case. The names are reviewer-facing.

**Keep examples self-contained.** Each example provides all the inputs it needs. Don't reference values from other examples.

### 4.4 Anti-patterns

**Examples that all engage the easy path.** Three examples all transferring between two solvent accounts with comfortable balances. The companion paper's §3.3 calls this *example narrowing*. `specq` flags low boundary coverage.

**Examples that are tautologies of the operation's signature.** If the postcondition exhaustively determines the output, the example is just a repetition of `ensures`. Examples should engage with concrete values that exercise behavior.

**Examples with no expected output.** v0 requires every example to specify an expected output; otherwise the example is not runnable.

---

## 5. `does` — the implementation

### 5.1 Role

`does` is the implementation of the operations in `relates`. The implementation must satisfy the `requires`/`ensures` contracts in `relates`; the verifier checks this. The codegen translates `does` to the target language.

`does` is the umbrella's *behavior*. It is the only register that produces runtime artifacts (target-language code). The other registers produce verification obligations, examples, types, and properties — all of which inform the implementation but do not execute.

### 5.2 Forms

**Basic implementation.**
```loom
does {
  open_account(id: AccountId, initial: Money) -> Account {
    {id: id, balance: initial, open: true}
  }
}
```

The body is an expression. The expression's value is the operation's return value.

**Implementation with intermediate bindings.**
```loom
does {
  transfer(from: Account, to: Account, amount: PositiveAmount) -> (Account, Account) {
    let from' = from with {balance: from.balance - amount};
    let to' = to with {balance: to.balance + amount};
    (from', to')
  }
}
```

`let` bindings introduce names for intermediate values. The `with` syntax produces a record-updated copy.

**Implementation with conditional.**
```loom
does {
  describe_result(r: TransferResult) -> string {
    match r with
    | Success {transfer, new_balance} => "transferred " + transfer.amount.to_string()
    | InsufficientFunds {available, ...} => "insufficient: have " + available.to_string()
    | AccountClosed {account} => "closed: " + account
  }
}
```

`match` over variants is exhaustive; the checker requires all variants to be covered.

### 5.3 Patterns

**Implementation mirrors `relates`.** When `relates` says `result.balance = initial`, the implementation literally sets `balance: initial`. The closeness makes verification easy; the SMT solver can match the implementation to the postcondition directly.

**Use named intermediates for readability.** When transforming several values, introduce `let` bindings rather than nesting. The verifier handles either form; the human reader prefers the named form.

**Avoid unnecessary structure.** A two-line implementation that satisfies the contract is preferable to a ten-line one that does the same work. The codegen target sees the implementation; the verifier reasons about it; the human reviews it. All three benefit from terseness.

### 5.4 Anti-patterns

**Implementation that does more than the contract.** The implementation `transfer` should produce the two updated accounts and nothing else. If it also writes to a log, the side effect is undeclared in `relates` and the umbrella is incomplete. Either declare the effect or remove it.

**Implementation that does less than the contract.** If `ensures result.open = from.open` is in `relates` but the implementation forgets to set `open`, the verifier catches it. The error is a verification failure, not a checker warning.

**Hidden conditionals.** Branching on values not declared in the parameter list. Either parameterize on the value, or model the branching as a sum type with explicit cases.

---

## 6. `proves` — the properties

### 6.1 Role

`proves` declares properties about operations: things that hold for *all* values, not just the specific values in `shows`. A `proves` entry is a universally-quantified statement, typically about the relationship between operations or about invariants the operations maintain.

`proves` is the umbrella's *guarantee*. The examples in `shows` demonstrate; the properties in `proves` generalize. The verifier discharges `proves` entries against the implementation in `does` and reports the result in the gap report.

### 6.2 Forms

**Universal property over operations.**
```loom
proves {
  conservation:
    for-all from: Account, to: Account, amount: PositiveAmount,
      from.open and to.open and from.balance >= amount =>
        let (from', to') = transfer(from, to, amount);
        from.balance + to.balance = from'.balance + to'.balance
}
```

The shape: `name: for-all params, condition => conclusion`. The condition guards the universal; the conclusion is what holds.

**Invariant.**
```loom
proves {
  no_overdrafts:
    for-all from: Account, to: Account, amount: PositiveAmount,
      from.open and to.open and from.balance >= amount =>
        let (from', _) = transfer(from, to, amount);
        from'.balance >= 0
}
```

An invariant says that a property of the state is preserved across operations. Often written as `for-all state, op(state).property = state.property` (preservation) or `for-all state, P(op(state))` (maintenance of P).

**Algebraic property.**
```loom
proves {
  increment_then_decrement_is_identity:
    for-all s: State when s.x > 0,
      decrement(increment(s)) = s
}
```

Properties relating compositions of operations: associativity, commutativity, idempotence, identity laws. These reveal algebraic structure in the implementation.

**Existential property (less common).**
```loom
proves {
  some_overdraftable_account_exists:
    exists a: Account, can_overdraft(a) = false
}
```

Existentials require the verifier to construct a witness. Discharge depends on the verifier; v0 handles existentials within SMT's reach.

### 6.3 Patterns

**Properties about preservation.** `for-all input, op(input).P = input.P` says `op` preserves P. Useful for invariants (account ids don't change), conservation (totals are preserved), and openness (operations don't close accounts).

**Properties about boundedness.** `for-all input, op(input).x <= max` says the result is bounded. Useful for safety properties.

**Properties about relationships between operations.** `for-all s, op_a(op_b(s)) = op_b(op_a(s))` says two operations commute. `for-all s, op_b(op_a(s)) = s` says they are inverses.

**Name properties for what they assert, not what they're for.** `conservation` says total is conserved; `no_overdrafts` says no balance goes negative. The name describes the property, not its motivation.

### 6.4 Anti-patterns

**Properties that mention only inputs.** `for-all from, from.open` is not a property of any operation; it's a property of the input type. Either move it to the type's refinement or restate it as a property of an operation's behavior.

**Properties that follow from `ensures`.** If `ensures result.balance = initial` is in `relates`, a `proves` clause `for-all id initial, open_account(id, initial).balance = initial` is redundant. The verifier discharges both, but the property adds no information. `loom check` flags this.

**Properties with vacuous antecedents.** `for-all x, (x > 0 and x < 0) => P(x)` is vacuously true. The grammar bans the syntactic form; the spec quality reporter catches semantic equivalents.

**Properties that the implementation cannot establish.** If the implementation does not maintain the property, the verifier reports failure. The right response is either to fix the implementation or to weaken (or drop) the property. Resist the temptation to add `gap` annotations as the default response.

---

## 7. `summarizes` (sketch, post-v0)

For umbrellas that depend on child umbrellas, `summarizes` expresses what the parent guarantees in terms of what the children prove:

```loom
summarizes {
  customer_funds_safe:
    requires { 
      ledger.conservation, 
      ledger.no_overdrafts, 
      audit.tamper_evident 
    }
    means {
      for-all customer: AccountId, transaction: Transfer,
        balance(customer) = sum of (all completed transfers for customer)
    }
}
```

The form: a name, a `requires` clause listing child claims the parent depends on, and a `means` clause stating what the parent guarantees given those dependencies. The verifier checks that the `means` clause follows from the conjunction of `requires` claims, treating child claims as assumed.

`summarizes` is sketched here for completeness. It is not in v0; the corresponding verifier work and the cross-umbrella gap report are post-v0.

---

## 8. Cross-register coverage rules

`loom check` enforces:

**Type coverage.** Every type defined in `knows` must appear in at least one signature in `relates`, in at least one example in `shows`, or in at least one property in `proves`. Unused types are dead vocabulary.

**Predicate coverage.** Every predicate defined in `knows` must appear in at least one `requires`, `ensures`, or `proves` body. Unused predicates are dead.

**Operation coverage.** Every operation in `relates` must have:
- An implementation in `does`,
- At least one example in `shows`,
- At least one property in `proves`.

The last two are warnings, not errors, in v0 (some operations are trivially uninteresting and don't merit a property). v0.x may raise them to errors with explicit suppression annotations for the trivial cases.

**Example coverage.** Every example in `shows` must reference an operation defined in `relates`. Orphan examples are errors.

**Property coverage.** Every property in `proves` must reference at least one operation from `relates`. Properties that mention only types from `knows` (and not operations) are warnings; they may indicate a property that should be a refinement type instead.

---

## 9. Authoring workflow

A common pattern for authoring an umbrella from scratch:

1. **Start with `knows`.** Decide on the types and predicates the umbrella will use. Keep refinements tight.
2. **Move to `relates`.** Declare the operations with full pre- and postconditions. Don't write implementations yet.
3. **Add `shows`.** Provide examples for each operation. Engage boundaries.
4. **Add `proves`.** Write the properties the operations should satisfy as a group. Use conservation, invariance, and boundedness as starting points.
5. **Implement in `does`.** Write the minimal implementation that satisfies `relates` and `proves`. Iterate until `loom verify` reports all category-(A).
6. **Run `loom specq`.** Address weak claims and unused vocabulary.
7. **Run `loom verify --with-gap-discovery`.** Check the category-(C) findings — properties the implementation establishes that the umbrella doesn't credit. Add or simplify accordingly.

The LLM operations support each step: `loom distill` (prose → initial umbrella), `loom generate` (umbrella → implementation), `loom summarize` (implementation → properties, post-v0).

---

## 10. References

- [`docs/language-reference.md`](language-reference.md) — full syntax.
- [`docs/bidirectional-refinement.md`](bidirectional-refinement.md) — the gap report.
- [`docs/spec-quality.md`](spec-quality.md) — `specq` and weak-claim detection.
- [`docs/verification-internals.md`](verification-internals.md) — how each register translates to Dafny.
