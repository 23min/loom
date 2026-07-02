---
id: ADR-0003
title: Python as the v0 target language for code generation
status: rejected
---
# ADR-0003 — Python as the v0 target language for code generation

> **REJECTED — 2026-07-02, superseded by ADR-0017.** loom generates no target code (not Python or any language); code generation is the LLM's role. Retained for the historical record — its analysis stood for a codegen destination loom no longer has.


> **Date:** 2026-05-22 · **Deciders:** project initial author; open for review.
> **Related:** `PLAN.md` §4.4, §5.3; `docs/reference/verification-internals.md` §3.

---

## Context

Loom's three-layer architecture separates verification from execution. The verifier (Dafny, per ADR-0002) checks that the umbrella's claims are satisfied; the *target language* is what the umbrella's `does` register is compiled to for actual execution.

The target language choice affects:

- Which user community can adopt Loom (the target is the language of their existing codebases).
- The reliability of `loom generate` output (the LLM's ability to produce code in the target).
- The readability of generated code for human review.
- The performance of generated artifacts (not v0's concern but worth flagging).
- The mapping from Loom types and effects to target-language constructs.

The candidates are Python, TypeScript, Rust, JavaScript, and Go.

The plan commits to **a single target in v0** (`PLAN.md` §5.3) — multi-target compilation is out of scope. This means the target choice is consequential because v0 will not validate the cross-target portability claim.

---

## Decision

Adopt **Python** as the v0 target language.

Generated artifacts are pure Python modules in `generated/<module_name>/`, using `dataclasses` for records, tagged unions for sum types, plain functions for operations, and `pytest` test files for examples. No Loom runtime dependency: the generated code can be installed and used as a normal Python package.

---

## Considered alternatives

### TypeScript

**For TypeScript.** Strong static typing that matches Loom's types more tightly than Python's runtime annotations. Modern build tooling. Growing LLM competence. Sum types via discriminated unions are idiomatic. Excellent IDE support. The Anthropic SDK is most polished in TypeScript, which would simplify the LLM integration (but only marginally, since the LLM integration is a Rust crate per ADR-0001, not a target-language artifact).

**Against TypeScript.**

- The build pipeline (tsc, package.json, tsconfig.json, possibly bundling) is more complex than Python's "just run it."
- The Node.js runtime dependency is non-trivial for users whose existing systems are not Node-based.
- LLM TypeScript output is slightly less reliable than LLM Python output across current models. The gap is narrowing but real.
- Effect tracking (`@net`, `@db`) in TypeScript could leverage the type system more aggressively than in Python, but v0 does not enforce effects regardless.

### Rust

**For Rust.** Coherence with the compiler's implementation language (ADR-0001). Strong type system; ownership and capability tracking could map naturally to Loom's effect annotations in post-v0. No runtime dependency for generated artifacts; static binaries.

**Against Rust.**

- LLM Rust output is the least reliable of the three. Models reach for unidiomatic constructs and produce code that does not compile.
- Rust's borrow checker creates friction for the immutable-by-default style Loom assumes. The translator would need to insert clones liberally or use `Rc<T>`/`Arc<T>`, both of which produce non-idiomatic generated code.
- High activation energy for users to read generated code. Rust is more demanding to learn than Python; review burden is higher.
- Rust binaries are heavy compared to Python scripts; for v0's small examples this is overkill.

### JavaScript

**For JavaScript.** Largest LLM training corpus of any language. Universal runtime availability. Simple to deploy. No build step for plain JS.

**Against JavaScript.** Loose typing makes generated code's relationship to Loom's types implicit at best. The codegen would need to choose between adding runtime checks (defeating the type-erasure benefit) or relying on JSDoc (which most tooling ignores). TypeScript is strictly preferable for the same audience.

### Go

**For Go.** Fast compilation. Static binaries. Concurrency primitives that could map onto the deferred actor model. Reasonably-sized LLM training corpus.

**Against Go.** No algebraic data types — sum types must be encoded as interfaces with implementing types, which is verbose and removes the exhaustiveness checking Loom's `match` provides. No generics until recently and still less expressive than Rust's. Less idiomatic LLM output than Python.

### Python

**For Python.**

- LLM Python output is the most reliable across current models. `loom generate` will produce working code more often than with any alternative.
- Python is easy to read for humans inspecting generated code. Review is a regular part of the Loom workflow; readability matters.
- Python's dataclasses (with `frozen=True`) provide immutable records that match Loom's records well.
- Tagged unions via class hierarchies or `Union` types are sufficient (not as elegant as TypeScript's discriminated unions but workable).
- The Python ecosystem provides `pytest` directly, so `shows` examples become idiomatic test functions.
- Installation story is trivial: `pip install` from a directory works.
- Python's reflection capabilities make runtime assertion injection (for `--with-runtime-asserts` mode) straightforward.
- Effect annotations (`@net`, `@db`) can be encoded as Python decorators in post-v0, providing a path to capability tracking without language redesign.

**Against Python.**

- Dynamic typing means the runtime does not enforce Loom's types. Refinement-type violations become runtime exceptions (with `--with-runtime-asserts`) or silent (without). The verifier guarantees correctness in principle; runtime checks are defense-in-depth.
- Performance is acceptable for v0 examples but would not satisfy production workloads. Out of scope for v0.
- Some Loom claims (capability profiles, effect tracking) have no natural Python translation in v0; they degrade to comments or runtime annotations.
- The Python community has a complicated relationship with type annotations; the generated code uses them, which some users find foreign. Mitigated by the code being mechanically generated rather than human-written.

---

## Consequences

### Positive

- Fastest path to a working pipeline end-to-end.
- LLM operations (`loom generate`) produce working code most reliably.
- Code review burden is lowest.
- Distribution as a Python package is well-understood.

### Negative

- v0 does not validate the cross-target portability claim from the architecture paper. Multi-target support is post-v0.
- Refinement type violations are not statically caught at the target level (only at the verifier level). For production use, this would be a real gap; for v0 (research prototype) it is acceptable.
- Some Loom features (capabilities, effects) cannot be enforced at the Python level until post-v0 work designs the mapping.

### Neutral

- The codegen crate (`crates/loom-compile-python`) is independent of the verifier crate, so the choice does not constrain ADR-0002 or future verifier swaps.
- Adding a second target (e.g., TypeScript) post-v0 is a matter of writing a parallel crate; the Loom AST and the architecture do not change.

---

## Implementation notes

- Codegen rules are documented in `docs/reference/verification-internals.md` §3.
- Generated artifacts go to `generated/<module_name>/` by default; users can override with `--output`.
- The translator emits Python type annotations on all function signatures for editor and `mypy` support. Refinement type constraints are encoded as docstring comments and, optionally, as `assert` statements.
- The translator emits `pytest` test files for `shows` examples; tests run with `pytest generated/<module_name>/tests/`.
- The translator does not depend on any Loom-specific Python runtime. Generated code uses only the standard library plus `pytest` (test-only).

---

## Future considerations

Post-v0 work on multi-target compilation should consider:

- **TypeScript** as the second target, for users in Node-based ecosystems and for the static-typing benefits the Python target lacks.
- **Rust** as a third target, for the systems-programming user community and for the eventual capability-tracking work.

The current single-target choice does not bias these future decisions. The codegen architecture in v0 is structured around a single target's idioms; abstracting it for multi-target use is post-v0 design work, not v0 commitment.

---

## References

- `PLAN.md` §4.4 (target language compilation), §5.3 (multi-target deferred to post-v0).
- `docs/reference/verification-internals.md` §3 (execution-direction translation).
- Python dataclasses: https://docs.python.org/3/library/dataclasses.html
- pytest: https://docs.pytest.org/
