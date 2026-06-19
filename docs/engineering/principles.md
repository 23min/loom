# Principles for a healthy codebase

A field guide for engineers trying to undo "vibe coding" decay, or
trying to prevent it. The principles below characterize code that a
senior engineer can pick up and understand within hours rather than
weeks. They aren't strict rules — they're forces. When forces
conflict, judgment is required.

## How to use this

- **Review checklist** while reading code or PRs.
- **Scorecard** for an existing codebase you've inherited — score
  each principle Strong / Weak / Missing with file:line evidence.
- **Prompt list** when writing something new and trying to keep it
  clean.

The right next step is rarely "fix all of these." It's: pick the
one or two that fail hardest in the current codebase, fix those,
then re-score.

## What this is NOT

- A style guide (no formatting rules).
- A framework (no specific libraries).
- A methodology (no agile / TDD / clean-architecture dogma).

It's the small set of properties that tend to be true of code that
ages well, regardless of stack.

---

# The principles

## A. Module boundaries

### A1. High cohesion

Each module has one reason to exist. A change that touches one
concern touches one module. A change that touches another concern
touches a different module.

**Smells:**
- A single file or class that mutates 10+ kinds of state.
- "And-also" function names (`load_config_and_apply_defaults`).
- Top-of-file imports that span unrelated subsystems
  (filesystem + HTTP + DB + UI in one file).
- A function that takes 8+ unrelated parameters because it does 8+
  unrelated things.

**Moves:**
- Name the module's single concern in one sentence. If you can't,
  split it.
- Group functions that change together; split functions that don't.
- Push side concerns (logging, metrics, retries) into decorators or
  wrappers, not the core function body.

**Tradeoff:** premature splitting creates fictional boundaries that
get reabsorbed later. If three things change together every time, they
are one thing — leave them.

### A2. Low coupling

Modules talk through narrow, named interfaces. Changing one
module's internals doesn't ripple into others.

**Smells:**
- Module A reaches into module B's private state (`b._cache[...]`).
- Cyclic imports.
- "Helper" modules that everyone imports because they accumulated
  miscellaneous utilities.
- Changing one function forces edits in five unrelated files.

**Moves:**
- Define the interface (function signatures, dataclasses, protocols)
  before writing the implementation.
- Prefer data passed in arguments to data fetched from globals.
- If A and B both need the same primitive, push the primitive down to
  a shared module they both depend on — don't let A reach into B for
  it.

**Tradeoff:** zero coupling means everything is duplicated. Some
coupling is fine — even good — when the cost of indirection exceeds
the cost of the link.

### A3. Layered (no upward dependencies)

Higher-level modules depend on lower-level ones. Never the reverse.
The CLI depends on the domain; the domain doesn't import the CLI.

**Smells:**
- A "core" module that imports its UI.
- A library that knows about its callers ("if this is being called
  from the web handler, do X").
- Reverse-engineering required to find the entry point.

**Moves:**
- Draw the dependency graph. Cycles or upward arrows are bugs.
- Inject dependencies down (pass a logger, don't import one with
  caller knowledge baked in).
- Keep the "what" (domain logic) separate from the "how" (transports,
  storage drivers, UI).

**Tradeoff:** strict layering adds indirection. Two layers is usually
enough for small codebases; resist building five.

---

## B. Contracts

### B1. Typed interfaces

Inputs and outputs at module boundaries are named types — dataclasses,
structs, TypedDicts, Pydantic models. Not loose `dict`s or tuples.

**Smells:**
- Functions returning `dict[str, Any]` where the keys are "the schema."
- Tuple returns where position matters and no one remembers what.
- "Magic" string parameters (`mode="strict"`, `mode="lenient"`) with
  no enum.
- Callers reach into return values with string keys that aren't
  validated anywhere.

**Moves:**
- Promote every "shape that crosses a boundary" to a named type.
- Use enums for closed sets of options.
- Make types frozen / immutable by default; mutate via explicit
  builders.

**Tradeoff:** types have a maintenance cost. For private internal
shapes that only live within one module, loose dicts are fine.

### B2. Schemas at boundaries

Wherever data crosses a process boundary — JSON over HTTP, JSONL in a
shared file, rows in a database, messages on a queue — the shape is
declared once and validated.

**Smells:**
- Two languages (e.g., Python writer + Swift reader) each define their
  own version of the same struct. Drift is invisible until something
  breaks.
- A JSONL file's schema lives only in the writer's code.
- A "version" field exists but nothing checks it.
- The only schema documentation is "see the example file."

**Moves:**
- One declaration per schema, codegen the rest. JSON Schema, Protobuf,
  OpenAPI — pick one and use it.
- Validate at the boundary (when reading), not deep in the consumer.
- Equivalence tests between the writer and the reader (see D2).

**Tradeoff:** codegen is friction. For one-off internal files that one
team controls end-to-end, a hand-written shared model is fine.

### B3. Pre/post conditions and invariants

Functions document what they require and what they guarantee.
Invariants ("this list is always sorted," "this set never contains
duplicates") are named and tested.

**Smells:**
- Defensive code at every call site, because callers can't trust the
  function.
- "Sometimes this returns None" without docs.
- Functions whose behavior depends on hidden global state.
- A bug fix that says "we forgot it could be empty" — repeatedly.

**Moves:**
- Document inputs ("must be UTF-8, must be non-empty") and outputs
  ("returns sorted, deduplicated, non-empty").
- Add an `assert` for the invariant near the construction site, not
  at every consumer.
- Reach for type narrowing (`NonEmptyList`, `SortedList`, etc.) when
  the invariant is load-bearing.

**Tradeoff:** asserts in hot paths cost cycles. Pre/post conditions
are most valuable at module boundaries; fewer are needed inside a
module where the author controls all the call sites.

---

## C. Data discipline

### C1. Single source of truth

Each fact lives in one place. Derived facts are computed, not
duplicated.

**Smells:**
- "Why does the UI show 5 when the database says 4?"
- Two stores claiming to own the same record (one is stale).
- A cache without a documented invalidation rule.
- The same data lives in three different formats across the codebase.

**Moves:**
- Name the canonical store for each piece of state.
- Derive everything else with pure functions.
- If you must cache, document the invalidation: "this cache is rebuilt
  on X event, valid for Y duration."

**Tradeoff:** denormalization is sometimes necessary for performance.
When you denormalize, name the master, the copy, and the rule that
keeps them aligned.

### C2. Idempotence

Re-running an operation against unchanged inputs converges on the same
state. No cruft. No accumulation. No surprises on retry.

**Smells:**
- Re-running the import script creates duplicate records.
- A retry produces a different result than the first call.
- "Run this once" warnings in the README.
- Recovery from a crash requires manual cleanup.

**Moves:**
- Identify the "key" of each operation (file content hash, request
  ID, primary key) and make it the basis for "have we seen this?"
- Distinguish "create if absent" from "always create."
- Test idempotence directly: run the operation twice and assert state
  is unchanged after the second run.

**Tradeoff:** idempotence is sometimes expensive (extra reads to
check existence). For high-throughput hot paths, log-and-deduplicate
later may be faster than check-before-write.

### C3. Atomic writes

A crash mid-write leaves the persisted state either fully-old or
fully-new — never half-written. Same for multi-step state changes:
either all complete or none.

**Smells:**
- A crash leaves the file with the first half of the new content
  followed by the tail of the old.
- "Sometimes the JSONL has a partial line at the end."
- Recovery code that "tolerates" corruption by skipping it.
- Two writes that must agree (e.g., index + content file) where one
  can succeed and the other fail.

**Moves:**
- Single-file: write to a sibling temp file, fsync, then rename.
- Multi-file: write all temps first, fsync, then rename in one
  predictable order.
- For multi-store: use a transaction or a two-phase commit pattern,
  or accept the asymmetry and design recovery around it.

**Tradeoff:** atomic writes cost an extra fsync and a rename. For
short-lived caches that can be rebuilt, ordinary writes are fine.

### C4. Versioned schemas with migration paths

When the shape of stored data changes, there's a declared path
forward — not "edit the file by hand."

**Smells:**
- The README says "delete the old JSON before upgrading."
- A field rename requires a coordinated deploy across the writer and
  every reader.
- "Legacy format" handling that nobody can remove because we don't
  know who still has the old data.
- Silent data loss when a reader skips a field the writer added.

**Moves:**
- Embed a version field from day one.
- For each schema bump, ship a migration step that's idempotent and
  re-runnable (see C2).
- Plan for forward-compat (readers tolerate unknown fields) and
  backward-compat (writers can emit the old shape for one cycle).

**Tradeoff:** versioning is bureaucracy. For schemas that one process
owns end-to-end with no historical data on disk, you can skip it.

---

## D. Tests that pin behavior, not implementation

### D1. Behavior pinned, not structure

Tests assert what the code does for inputs X — not which helpers got
called, in what order, with which mocks.

**Smells:**
- Tests fail when you rename an internal function.
- Tests mock five things just to call the one under test.
- Refactoring breaks 20 tests that all should have been one
  characterization snapshot.
- "Test setup" is longer than the test body.

**Moves:**
- For legacy code: characterization snapshots. Capture the current
  output, pin it as the golden reference, refactor against the gate.
- Prefer integration over unit when the cost is similar.
- Mock at process / network / filesystem boundaries — not at internal
  function boundaries.

**Tradeoff:** integration tests are slower. For pure-function
algorithm code (parsers, scorers, validators) unit tests are right.

### D2. Equivalence tests at seams

Where two implementations claim to be interchangeable (an in-memory
fake and a real DB; a Python writer and a Swift reader), prove it.
Run both against the same scenarios; assert equivalent decisions.

**Smells:**
- "The fake works but production breaks differently."
- A reader-writer pair where one side silently went out of sync.
- Two libraries that both claim to implement the same protocol with
  no shared conformance test.

**Moves:**
- Define a contract (interface, protocol, test matrix).
- One test suite, parameterized over implementations.
- Run the matrix in CI for every change to either side.

**Tradeoff:** equivalence tests double the surface that has to stay
in sync. For implementations that genuinely differ (e.g., a SQL
backend and a NoSQL backend with different consistency models),
test the contract you actually share.

### D3. Branch coverage on touched code

A coverage floor (typically 60-80%) on the lines and branches you
change in this PR — not a retroactive bar on legacy modules.

**Smells:**
- Coverage reports celebrate 92% overall while the new code is at
  30%.
- A test "passes" because the branch under test never ran.
- No coverage report at all — "we test what matters."

**Moves:**
- Gate the merge on `coverage-on-diff` ≥ threshold, not absolute %.
- Branch coverage, not statement coverage — `if x:` with no
  `else:` test is a gap.
- Raise the floor as the codebase improves. Don't try to backfill
  legacy in one go.

**Tradeoff:** coverage isn't quality. A test can hit a branch without
asserting anything useful. Coverage is necessary, not sufficient.

### D4. Tests at the right altitude

Unit tests for pure functions; integration tests at module boundaries;
end-to-end tests for the externally-observable behavior. Don't mock
what you can integration-test cheaply.

**Smells:**
- Every test mocks the database.
- "Unit tests" that exercise three modules and two real files.
- No end-to-end test — "we tested all the units."
- The same scenario tested at four altitudes, each in a different way.

**Moves:**
- Per-scenario, pick one altitude. Document why.
- Pure functions get unit tests. CLI commands get end-to-end tests.
- Mock at process boundaries (the network call, the LLM API), not
  at internal function calls.

**Tradeoff:** integration tests are slower and harder to isolate.
For algorithmic correctness where the inputs are easy to construct,
unit tests give better feedback per second.

---

## E. Errors, logs, audit trail

### E1. Structured logs

No bare `print()` or unstructured `log.info("did the thing")`.
Events have a name and a context dict; output is JSON-renderable.

**Smells:**
- Log messages full of string interpolation.
- Logs are searched with `grep`, never queried.
- "I added logging" means new `print()` calls.
- Production debug means setting `LOG_LEVEL=DEBUG` and hoping.

**Moves:**
- One structured logger (`structlog`, `zap`, `pino`, etc.); the
  rendering format is a config flag.
- Every emit binds context: `log.info("match_succeeded", file=..., score=...)`.
- Capture log events in tests so you can assert "this event fired
  with these fields."

**Tradeoff:** structured logging is more code at the emit site. For
throwaway scripts, `print` is fine.

### E2. Designed failure modes

What happens on missing input, unparseable file, network timeout,
disk full, concurrent access — is documented and tested. Not "we'll
find out when it breaks."

**Smells:**
- Exception handlers that say `pass` or `continue`.
- "Why is the file empty?" investigations that lead to a swallowed
  exception three modules deep.
- A retry loop with no jitter, no backoff, no max attempts.
- Race conditions discovered in production.

**Moves:**
- Per module, list the failure modes. Pick one of: surface, retry,
  fallback, fail-fast. Document the choice.
- Test the failure paths. A fault-injection test (random kill, full
  disk, network partition) is worth its weight.
- Distinguish "expected" failures (no match) from "unexpected"
  (parser crash). Different handling, different logs.

**Tradeoff:** designing every failure mode upfront is over-engineering
for a prototype. Reach for it when the code matters.

### E3. Audit trail

Every significant state change leaves a record. For systems where
trust matters (financial, medical, security, legal), this is
non-negotiable.

**Smells:**
- "Who deleted that record?" — nobody knows.
- The provenance of a value is "the database has it that way" with
  no history.
- A bug surfaced because someone manually edited a file in
  production.

**Moves:**
- Append-only event log alongside the state-of-the-world store.
- Each event names the actor (human, system, which service), the
  action, the before, the after, the timestamp.
- Treat the event log as data — query it, replay it, test against
  it.

**Tradeoff:** auditing every read is overkill. Audit writes, audit
decisions; let reads be inferred from logs.

### E4. Self-explaining errors

When something fails, the error message says what was tried, what was
expected, what was found. Stack traces lead to the actual problem,
not to the place we re-raised.

**Smells:**
- `raise Exception("error")`.
- Error messages that contain the function name and nothing else.
- Re-raising in a way that loses the original cause.
- The user's question after an error is always "what does this mean?"

**Moves:**
- Errors carry context. `f"matched {a}, expected {b}, in {path}"` not
  `"mismatch"`.
- Preserve causes (`raise X from e`, error wrapping that includes the
  original).
- Make error messages a first-class output: review them like UI
  copy, not afterthought.

**Tradeoff:** rich errors take effort. For internal-only systems with
a small team, "go read the log" may be acceptable.

---

## F. Reasoning aids

### F1. Names that don't lie

A function does what its name says — no more, no less. A variable
contains what its name claims.

**Smells:**
- `get_user` that also creates a user if absent.
- `validate` that mutates.
- A boolean named `loaded` that means "loaded or failed."
- "Util" / "helper" / "manager" / "handler" — name says nothing.

**Moves:**
- Read function names aloud as sentences. If "this function `get_user`
  creates and saves a user" sounds wrong, rename.
- Names should reveal *intent*, not implementation.
- When a function does N things, either rename it `do_n_things` (be
  honest) or split it.

**Tradeoff:** renaming is invasive. Sometimes the right move is to
leave a lying name and fix it the next time you touch the function.

### F2. Comments only for non-obvious "why"

Every comment should answer a question the code can't: a hidden
constraint, a subtle invariant, a workaround for a specific bug, a
historical decision that would be reverted without context.

**Smells:**
- Comments that restate the code in English.
- "Increment counter" above `counter += 1`.
- Block comments listing parameters and types when the function
  already has type hints.
- Stale comments contradicting the code they describe.

**Moves:**
- Delete comments that restate code.
- Keep comments that explain *why this surprising choice*.
- When you find yourself writing a comment, ask "could I rename a
  variable / function instead?"

**Tradeoff:** for public APIs, docstrings are expected even when
"obvious." Internal code can be sparser.

### F3. Decision records that survive turnover

The "why" of significant design choices survives the people who knew
it. ADRs, design docs, "decisions" notes — the format matters less
than the practice.

**Smells:**
- "Why is it like this?" answered with "I don't know, ask
  [person who left]."
- A code comment that says "see Slack thread from 2022."
- Two different parts of the codebase implement the same thing
  differently with no rationale.
- A refactor reverts a load-bearing choice because nobody knew it
  was load-bearing.

**Moves:**
- One short doc per non-obvious decision. Context, options
  considered, choice, consequences. Date and author.
- Link the doc from the code where the decision is enforced.
- Update or supersede when the decision changes. Don't delete —
  history matters.

**Tradeoff:** ADRs for every choice is paralysis. Reserve them for
decisions that future you (or a new hire) would otherwise re-litigate.

---

## G. Operational properties

### G1. Reproducible

Same inputs → same outputs. No hidden time / random / environment /
network dependencies in business logic.

**Smells:**
- A test passes sometimes, fails sometimes.
- "Works on my machine."
- Output depends on what was in /tmp at 3am.
- `datetime.now()` and `random()` scattered through pure-looking
  functions.

**Moves:**
- Push non-deterministic inputs (time, randomness, env) to the
  edges. Inject them.
- Capture and replay: every "run" can be saved and replayed against
  a future code version.
- Containers, lockfiles, pinned versions.

**Tradeoff:** strict determinism removes legitimate randomness
(jitter, sampling). When randomness is real, seed it explicitly.

### G2. Reversible

Destructive operations are guarded, undoable, or both. You can
recover from a botched run without restoring from backup.

**Smells:**
- A single typo deletes production data.
- "We don't have undo because nobody asked for it."
- The only recovery is a backup that's a week old.
- Dry-run is an afterthought, separately implemented from real-run.

**Moves:**
- Soft-delete by default; hard-delete is a separate operation.
- `--dry-run` is the same code path as the real run, with the writes
  routed to a stub.
- Confirmations on destructive operations match the blast radius (a
  prompt for one file, a multi-step ceremony for the whole database).

**Tradeoff:** soft-delete costs storage. For genuinely transient
data, hard-delete is fine.

### G3. Observable in production

When the user says "this looked wrong," you can answer: here's what
happened, here's why, here's the data it saw. Logs, metrics, traces,
provenance fields on records.

**Smells:**
- "Can you reproduce it?" is the first question after every bug
  report.
- Production decisions have no recorded reasoning.
- A score is shown to the user with no way to see the inputs to that
  score.
- Operational dashboards exist for infrastructure but not for
  business logic.

**Moves:**
- Provenance fields on records: which version / method / inputs
  produced this value.
- Decision logs: every non-trivial branch records why it took the
  branch.
- Metrics: per-step counts, latencies, success/failure rates.
- Sampling for high-volume paths; full capture for slow ones.

**Tradeoff:** observability has overhead. For latency-critical
paths, sample aggressively. For correctness-critical paths, capture
everything.

---

# Meta sections

## How to score a codebase against this

1. **One reviewer per principle.** Don't try to evaluate everything
   simultaneously — one principle, end-to-end, find evidence.
2. **Concrete evidence.** Cite file:line for every Strong / Weak /
   Missing verdict. "Coupling is bad" is not a finding; "module X
   reaches into module Y's private state at A:42 and B:177" is.
3. **Strong / Weak / Missing.** Three levels is enough. Don't grade
   on a 1-10 scale; you'll spend time defending the difference
   between 6 and 7.
4. **Adversarial pass.** A second reviewer tries to refute the first's
   "Strong" verdicts. If a Strong survives a real attempt to refute,
   it's actually strong.
5. **Prioritize by leverage.** Pick the weak principle whose fix
   unlocks the most downstream work. Score informs sequencing; it
   doesn't dictate it.

## Anti-patterns: each principle taken too far

| Principle | Overdone becomes |
|---|---|
| High cohesion | A1 splitting → micro-modules that obscure the flow |
| Low coupling | Indirection everywhere; one-line wrapper functions |
| Layered | Five layers of dispatchers between input and effect |
| Typed interfaces | Every internal dict promoted to a dataclass; "type theater" |
| Schemas at boundaries | A schema registry for files only one process writes |
| Single source of truth | A central god-object that everyone depends on |
| Idempotence | Check-before-write where conflict is impossible |
| Atomic writes | Two-phase commits for cache rebuilds |
| Versioned schemas | Version field on every transient struct |
| Behavior pinning | Refactor-resistant tests that hide real regressions |
| Equivalence tests | Conformance tests for implementations that no longer share a contract |
| Branch coverage | 95% coverage of nothing meaningful |
| Structured logs | Event soup nobody queries |
| Designed failure modes | Speculative recovery code for failures that never happen |
| Audit trail | Auditing reads, writes, everything; storage explodes |
| Names that don't lie | Renaming as a hobby |
| Comments for "why" | Three-paragraph comments above five-line functions |
| Decision records | ADR for picking a CSS color |
| Reproducible | Determinism enforced where randomness was legitimate |
| Reversible | Confirmations on harmless reads |
| Observable | Logging at every line; signal lost in noise |

## Priority when you can't do everything

When inheriting a vibe-coded codebase, the principles don't all pay
back equally. Approximate order of leverage:

1. **D1 Behavior pinning** (characterization tests). Without this,
   no other refactor is safe. This is the gate.
2. **C3 Atomic writes** and **C1 Single source of truth**.
   Correctness comes before structure.
3. **E1 Structured logs**. You can't fix what you can't see.
4. **B1 Typed interfaces** and **B2 Schemas at boundaries**.
   Catches the most bugs per unit of effort.
5. **A1 High cohesion** + **A2 Low coupling**. The classic
   refactor target. Only attempt after 1-4 are in place.
6. **E3 Audit trail** + **G3 Observability**. Once the code is
   stable, make it operable.
7. Everything else — refine as the team's bandwidth allows.

The principle this list is itself an instance of: **don't try to
fix everything at once**.

## When NOT to apply this

- One-off scripts.
- Prototypes you intend to throw away (and you actually will).
- Code where the author is the only reader and the lifetime is days.

The cost of clean code is real. It pays back over time and over
multiple readers. When neither time nor readers exist, do the simple
thing.

---

*This document is meant to be edited. Replace examples with ones from
the codebase it's applied to. Add principles the team finds itself
re-deriving. Delete principles that don't fit the domain. The goal is
shared vocabulary, not orthodoxy.*
