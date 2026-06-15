# Loom language reference

> **Status:** draft (v0 surface — subject to refinement as the parser is built)
> **Audience:** anyone writing or reading `.lm` files

This document is the canonical surface-syntax reference for Loom v0. It is structured for completeness rather than for tutorial. The companion document [`docs/claims-reference.md`](claims-reference.md) catalogs the claim forms with examples.

---

## 1. File structure

A Loom source file has the extension `.lm`. A file contains one or more `module` declarations. A module is the unit of umbrella+implementation that the verifier acts on.

```loom
module ledger {
  // five registers, in any order
  knows { ... }
  relates { ... }
  shows { ... }
  does { ... }
  proves { ... }
}
```

The five registers (`knows`, `relates`, `shows`, `does`, `proves`) are the umbrella's structural skeleton. The cross-register coverage rules (every type used, every operation has an example and a property) are checked by `loom check`; see [`docs/claims-reference.md`](claims-reference.md) §1.

A file may also contain `import` declarations at the top level, before any module:

```loom
import money from "../shared/money.lm"
import * from "../shared/account.lm"
```

Imports bring names into scope for downstream modules in the same file. Imports are resolved relative to the importing file's directory.

A file may have a single top-of-file header for metadata:

```loom
@loom-version 0.1
@target python
@profile minimal

module ledger { ... }
```

The header is optional; defaults are taken from the project's `loom.toml`.

---

## 2. Lexical structure

### 2.1 Identifiers

Identifiers are `[a-zA-Z_][a-zA-Z0-9_]*`. Convention:

- `snake_case` for value names and operation names.
- `PascalCase` for type names.
- `SCREAMING_SNAKE_CASE` for constants.

Reserved words: `module`, `import`, `knows`, `relates`, `shows`, `does`, `proves`, `summarizes`, `requires`, `ensures`, `when`, `gap`, `for-all`, `exists`, `true`, `false`, `let`, `if`, `then`, `else`, `match`, `with`, `before`, `after`, `result`.

### 2.2 Literals

- Integers: `42`, `-7`, `0x1A`, `0b1010`, `1_000_000`.
- Decimals: `3.14`, `-0.001`, `1.0e10`.
- Strings: `"hello"`, with `\n`, `\t`, `\\`, `\"` escapes.
- Booleans: `true`, `false`.
- Unit: `()`.

### 2.3 Comments

```loom
// single-line comment

/*
  block
  comment
*/

/// doc-comment on the next declaration
```

Doc-comments (`///`) attach to the following declaration and are surfaced in diagnostics and generated documentation.

### 2.4 Operators

Precedence, lowest to highest:

```
or              || (boolean)
and             && (boolean)
not             ! (prefix, boolean)
comparison      = ≠ < ≤ > ≥  (also != <= >= as ASCII)
additive        + -
multiplicative  * / %
unary           - (negation)
implication     ⇒ (also =>)
application     f(x, y)
field           x.y
```

The Unicode forms (`≤`, `≥`, `≠`, `⇒`, `∧`, `∨`, `¬`, `∀`, `∃`) are accepted and canonical in formatted output. ASCII equivalents are accepted in source.

---

## 3. Types

### 3.1 Built-in types

```
int       arbitrary-precision integer
nat       non-negative integer (subtype of int)
real      arbitrary-precision rational
bool      boolean
string    UTF-8 string
unit      the unit type, one value ()
```

### 3.2 Refinement types

A refinement type is a base type together with a predicate restricting its values:

```loom
{x: int | x >= 0}
{x: int | x > 0 and x < 100}
{s: string | s.length > 0}
```

The refinement predicate must be a closed expression in the surrounding scope. Predicates may reference user-defined predicates from the same module's `knows` block.

Named refinement types are introduced in `knows`:

```loom
knows {
  Money :: {x: int | x >= 0}
  PositiveAmount :: {x: int | x > 0}
  AccountId :: {s: string | s.length > 0 and s.length <= 64}
}
```

### 3.3 Record types

```loom
knows {
  Account :: {
    id: AccountId,
    balance: Money,
    open: bool,
  }
  
  Transfer :: {
    from: AccountId,
    to: AccountId,
    amount: PositiveAmount,
  }
}
```

Record fields are accessed with dot notation: `a.balance`. Record literals use the same brace syntax:

```loom
shows {
  example_account: Account = {id: "acct-1", balance: 100, open: true}
}
```

### 3.4 Sum types (variants)

```loom
knows {
  TransferResult :: 
    | Success {transfer: Transfer, new_balance: Money}
    | InsufficientFunds {account: AccountId, requested: Money, available: Money}
    | AccountClosed {account: AccountId}
}
```

Variants are pattern-matched with `match ... with`:

```loom
does {
  describe_result(r: TransferResult) -> string {
    match r with
    | Success {transfer, new_balance} => "transferred " + transfer.amount.to_string()
    | InsufficientFunds {account, requested, available} => "insufficient funds"
    | AccountClosed {account} => "account closed"
  }
}
```

### 3.5 Collection types

```
List<T>           ordered, immutable
Set<T>            unordered, immutable, unique elements
Map<K, V>         key-value, immutable
```

Collections are first-class values with operations defined in the standard prelude (`length`, `contains`, `insert`, etc.). Mutability is expressed at the operation level (operations may produce updated copies), not via type modifiers.

### 3.6 Function types

```
Account -> Money                  unary function
(Account, Transfer) -> Account    binary function
```

Functions are not first-class values in v0 (no closures, no higher-order functions in `does`). Higher-order claims in `proves` (quantification over functions) are supported syntactically but discharge depends on verifier capability.

### 3.7 Effects and capabilities

Effect annotations are part of the syntax but not enforced in v0:

```loom
relates {
  fetch_account(id: AccountId) -> Account @db @net
}
```

The annotations are parsed and recorded but do not affect verification. v0.x will enforce capability tracking; see ADR for the schedule.

---

## 4. The five registers

This section describes the *syntactic* shape of each register. Their *semantic* role and the catalog of claim forms within each register are in [`docs/claims-reference.md`](claims-reference.md).

### 4.1 `knows`

```loom
knows {
  // type definitions
  Money :: {x: int | x >= 0}
  Account :: { id: AccountId, balance: Money, open: bool }
  
  // user-defined predicates
  pred is_open(a: Account) = a.open
  pred is_solvent(a: Account) = a.balance > 0
  
  // constants
  const MAX_TRANSFER: Money = 1_000_000
  const ZERO_TRANSFER: Money = 0
}
```

Forms in `knows`: type definitions (`Name :: type`), predicate definitions (`pred name(params) = body`), constants (`const name: type = value`).

### 4.2 `relates`

Operation signatures with pre- and postconditions:

```loom
relates {
  open_account(id: AccountId, initial: Money) -> Account
    requires { initial >= 0 }
    ensures { result.id = id, result.balance = initial, result.open = true }

  transfer(from: Account, to: Account, amount: PositiveAmount) -> (Account, Account)
    requires { from.open, to.open, from.balance >= amount }
    ensures {
      let (from', to') = result;
      from'.balance = from.balance - amount,
      to'.balance = to.balance + amount,
      from'.id = from.id, to'.id = to.id,
      from'.open = from.open, to'.open = to.open,
    }
}
```

The `result` keyword in `ensures` refers to the operation's return value. For tuple returns, destructuring is supported via `let (a, b) = result;`.

A `relates` entry has the structure:
```
name(params) -> return_type
  requires { precondition }
  ensures { postcondition }
```

Both `requires` and `ensures` are optional; their absence means *no precondition* and *no postcondition* respectively.

### 4.3 `shows`

Concrete examples of operations:

```loom
shows {
  // example: open an account with initial balance
  open_initial:
    open_account("alice", 100)
    -> {id: "alice", balance: 100, open: true}

  // example: transfer between two accounts
  transfer_succeeds:
    transfer(
      {id: "alice", balance: 100, open: true},
      {id: "bob", balance: 0, open: true},
      30
    )
    -> ({id: "alice", balance: 70, open: true},
        {id: "bob", balance: 30, open: true})

  // example: transfer at boundary
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

The shape: `name: operation(args) -> expected_result`. The example name is the example's identifier and appears in diagnostics. The expected result is compared to the actual result by structural equality.

For operations with side effects (deferred to v0.x), the `shows` form will extend.

### 4.4 `does`

The implementation:

```loom
does {
  open_account(id: AccountId, initial: Money) -> Account {
    {id: id, balance: initial, open: true}
  }

  transfer(from: Account, to: Account, amount: PositiveAmount) -> (Account, Account) {
    let from' = from with {balance: from.balance - amount};
    let to' = to with {balance: to.balance + amount};
    (from', to')
  }
}
```

The `with` syntax produces a record with updated fields, leaving others unchanged. This is the only "mutation" syntax; all values are immutable.

The body of a `does` operation must satisfy the `requires` and `ensures` from `relates`. The verifier checks this; the human does not need to repeat the conditions in `does`.

### 4.5 `proves`

Universally quantified properties about operations:

```loom
proves {
  // conservation: total money is preserved by transfer
  conservation:
    for-all from: Account, to: Account, amount: PositiveAmount,
      from.open and to.open and from.balance >= amount =>
        let (from', to') = transfer(from, to, amount);
        from.balance + to.balance = from'.balance + to'.balance

  // no_overdrafts: account balance never goes negative
  no_overdrafts:
    for-all from: Account, to: Account, amount: PositiveAmount,
      from.open and to.open and from.balance >= amount =>
        let (from', _) = transfer(from, to, amount);
        from'.balance >= 0

  // open_account_initial: opening with balance B yields balance B
  open_account_initial:
    for-all id: AccountId, initial: Money,
      open_account(id, initial).balance = initial
}
```

Each `proves` entry has a name and a body. The body is a closed predicate, typically a universally-quantified statement about operations from `relates`.

Existential quantification is supported syntactically (`exists x: T, P(x)`) but discharge depends on the verifier; in v0 with Dafny, existentials within reach of SMT are decidable.

### 4.6 `summarizes` (post-v0 sketch)

When a parent umbrella collects claims from child modules, the `summarizes` register expresses the relationship:

```loom
summarizes {
  customer_funds_safe:
    requires { ledger.conservation, ledger.no_overdrafts, audit.tamper_evident }
    means { for-all customer, balance(customer) reflects all completed transfers }
}
```

`summarizes` is sketched here for completeness but is not in v0; child-umbrella composition is documented in detail when the feature lands.

---

## 5. Expressions

### 5.1 Let-bindings

```loom
let x = 42;
let (a, b) = some_tuple;
let {id, balance} = some_account;
```

Bindings are immutable. `let mut` and re-assignment are not in the language.

### 5.2 Conditional

```loom
if x > 0 then x else 0

// chained
if x > 100 then "high"
else if x > 10 then "medium"
else "low"
```

`if` is an expression; it must have an `else` branch.

### 5.3 Match

```loom
match transfer_result with
| Success {transfer, new_balance} => new_balance
| InsufficientFunds {available, ...} => available
| AccountClosed {...} => 0
```

Patterns include literals, variable bindings, record destructuring, variant destructuring, and `_` (wildcard). Patterns must be exhaustive; the checker reports missing patterns.

### 5.4 Quantification

```loom
for-all x: int, P(x)
exists x: int, P(x)

// with conditions
for-all x: int when x > 0, P(x)        // equivalent to ⇒
```

Quantified expressions appear in `proves` and in refinement-type predicates.

### 5.5 Built-in operations on records

```loom
account.balance          // field access
account with {balance: new_balance}    // record update
{id: "x", balance: 0, open: true}      // record literal
```

---

## 6. Effect annotations (parsed in v0, enforced post-v0)

```loom
relates {
  log_event(e: Event) -> unit @log
  read_config() -> Config @fs
  current_time() -> Timestamp @clock
  publish(msg: Message) -> unit @net
  query_db(q: Query) -> List<Row> @db
}
```

Effects are declared at the operation level. v0 parses and stores them. Capability inference (checking that a `does` body only uses effects declared in its `relates` signature) is post-v0.

---

## 7. Modules and imports

### 7.1 Module syntax

```loom
module ledger {
  knows { ... }
  relates { ... }
  shows { ... }
  does { ... }
  proves { ... }
}
```

Modules are the unit of compilation. A file may contain multiple modules; each is compiled and verified independently.

### 7.2 Imports

```loom
// at top of file, before any module declaration
import money from "../shared/money.lm"
import {Account, AccountId} from "./account.lm"
import * from "./prelude.lm"
```

Three forms:
- `import name from "path"` — import a single module and reference its declarations as `name.Foo`.
- `import {A, B, C} from "path"` — import specific declarations into the current scope.
- `import * from "path"` — import all public declarations into the current scope.

A declaration is *public* by default. Private declarations are prefixed with `priv`:

```loom
knows {
  priv internal_only :: int    // not exported
  Public :: string             // exported
}
```

### 7.3 Module composition (post-v0 sketch)

Cross-umbrella verification — proving that a parent's claims follow from children's claims — uses the `summarizes` register (§4.6) and is documented when implemented.

---

## 8. Header metadata

```loom
@loom-version 0.1
@target python
@profile minimal
@author "Alice <alice@example.com>"
@since 2026-05-22
```

Recognized headers:
- `@loom-version` — minimum compiler version.
- `@target` — preferred target language for codegen.
- `@profile` — verification profile (see `loom.toml`).
- `@author` — free-form author string.
- `@since` — ISO 8601 date when the umbrella was first written.

Unrecognized headers are warnings, not errors.

---

## 9. Conventions

### 9.1 Style

- Two-space indent.
- One register per line at the module level.
- Claims separated by blank lines for readability.
- Long claims may use line continuations within braces.

### 9.2 Naming

- Operations are verb phrases: `open_account`, `transfer`.
- Claims are noun phrases describing the property: `conservation`, `no_overdrafts`.
- Examples are descriptive names: `transfer_succeeds`, `boundary_exact_balance`.
- Types are PascalCase nouns: `Account`, `Money`.

### 9.3 Comments

- Doc-comments (`///`) on declarations are extracted into generated documentation.
- Comments inside register bodies explain *why*, not *what*. The umbrella's structure already explains what.

---

## 10. Reserved for future versions

Syntax positions reserved but not implemented in v0:

- Linear types (`!T`) — for resource-tracked values.
- Higher-rank refinements — refinements that quantify over functions.
- Dependent function types — return types that depend on input values.
- Phase tags within registers (`shows@boundary`, `proves@invariant`) — for categorizing claims.

These are listed so that parser implementations preserve syntactic space for them.

---

## 11. Grammar (informal BNF)

```
file        ::= header? import* module+

header      ::= ('@' identifier value)+

import      ::= 'import' import-spec 'from' string-literal
import-spec ::= identifier
              | '{' identifier (',' identifier)* '}'
              | '*'

module      ::= 'module' identifier '{' register+ '}'

register    ::= 'knows' '{' know-form* '}'
              | 'relates' '{' relate-form* '}'
              | 'shows' '{' show-form* '}'
              | 'does' '{' do-form* '}'
              | 'proves' '{' prove-form* '}'

know-form   ::= identifier '::' type
              | 'pred' identifier '(' params? ')' '=' expr
              | 'const' identifier ':' type '=' expr

relate-form ::= identifier '(' params? ')' '->' type
                ('requires' '{' expr '}')?
                ('ensures' '{' expr '}')?

show-form   ::= identifier ':' application '->' expr

do-form     ::= identifier '(' params? ')' '->' type '{' expr '}'

prove-form  ::= identifier ':' expr

type        ::= identifier
              | '{' identifier ':' type '|' expr '}'
              | '{' (field (',' field)*)? '}'
              | type '->' type
              | identifier '<' type (',' type)* '>'

expr        ::= literal | identifier | application | field-access
              | binary-op | unary-op | quantifier | let-expr | match-expr | if-expr
              | record-literal | record-update | tuple

quantifier  ::= ('for-all' | 'exists') params ',' expr
              | ('for-all' | 'exists') params 'when' expr ',' expr
```

The full grammar (with precedence, associativity, and disambiguation rules) is defined by the parser implementation in `crates/loom-syntax/`. This sketch is illustrative.

---

## 12. Worked example

A complete, small umbrella:

```loom
@loom-version 0.1
@target python

module counter {
  knows {
    State :: {x: int | x >= 0}
  }

  relates {
    initial() -> State
      ensures { result.x = 0 }

    increment(s: State) -> State
      ensures { result.x = s.x + 1 }

    decrement(s: State) -> State
      requires { s.x > 0 }
      ensures { result.x = s.x - 1 }
  }

  shows {
    starts_at_zero:
      initial() -> {x: 0}

    one_increment:
      increment({x: 5}) -> {x: 6}

    one_decrement:
      decrement({x: 5}) -> {x: 4}
  }

  does {
    initial() -> State {
      {x: 0}
    }

    increment(s: State) -> State {
      {x: s.x + 1}
    }

    decrement(s: State) -> State {
      {x: s.x - 1}
    }
  }

  proves {
    increment_then_decrement_is_identity:
      for-all s: State,
        decrement(increment(s)).x = s.x

    decrement_keeps_nonneg:
      for-all s: State when s.x > 0,
        decrement(s).x >= 0

    initial_is_zero:
      initial().x = 0
  }
}
```

Running `loom verify` on this file should produce a gap report with all five claims (`starts_at_zero`, `one_increment`, `one_decrement`, plus three `proves`) in category (A).

---

## 13. Open syntactic questions

These are tracked as open ADRs in `docs/adr/`:

- Whether `requires` and `ensures` should appear inline with `relates` or be allowed to be hoisted to a separate `requires` block at the module level. v0 inline; alternative considered for v0.x.
- Whether `does` bodies should be required to be expression-only (no statements). v0 is expression-only; statement form may be added if expression-only proves restrictive.
- Whether to support layout-sensitive parsing (significant indentation). v0 uses braces; layout-sensitive form is a possible v1 ergonomic improvement.

See `docs/adr/` for status of these and other open questions.
