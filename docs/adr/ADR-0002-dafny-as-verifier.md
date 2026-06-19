---
id: ADR-0002
title: Dafny as the v0 verification backend
status: proposed
---

# ADR-0002 — Dafny as the v0 verification backend

> **Date:** 2026-05-22 · **Deciders:** project initial author; open for review.
> **Related:** `PLAN.md` §2.6, §4.3; `docs/verification-internals.md`.

---

## Context

Loom v0's architectural commitment (`PLAN.md` §2.6) is to compose with existing verifiers rather than invent verification semantics. This requires choosing a specific verifier as the v0 backend. The choice is high-stakes: the verifier's semantics define what Loom's claims mean operationally, the verifier's tooling sets the pace of Loom's development, and the verifier's user base affects who can engage with the project's results.

The decision is also reversible in principle (a `Verifier` trait abstracts the backend, per `docs/verification-internals.md` §8) and irreversible in practice (the trait's interface will reflect the v0 backend's idioms, and reimplementing those idioms for a second backend is non-trivial work).

The candidates considered are Dafny, F*, direct Z3 (with a custom refinement-type layer), Lean 4, Liquid Haskell, and Why3.

Requirements:

- Refinement types or an equivalent for predicate-restricted base types.
- Pre- and postconditions on operations.
- Universally-quantified properties (the `proves` register).
- Automatic SMT-backed discharge for most claims (interactive proof should not be the default mode).
- Stable, maintained, with multi-platform binary distribution.
- Translation from Loom's surface to the verifier's input must be reasonable to implement and to debug.
- Counterexamples on failed proofs, mapped back to Loom-level vocabulary.

---

## Decision

Adopt **Dafny** as the v0 verification backend.

The Loom compiler translates umbrellas to `.dfy` files (per the encoding in `docs/verification-internals.md`) and invokes `dafny verify` as a subprocess. The backend is encapsulated in `crates/loom-compile-dafny` behind the `Verifier` trait.

Dafny version is pinned (specific version per a future `ADR-0014`). Z3 version is pinned to whatever Dafny version ships with by default.

---

## Considered alternatives

### F*

**For F\*.** F* is the closest match to Loom's architectural ambition. Refinement types with effects, dependent types, and a more expressive specification language than Dafny. The F* community is research-active. F* supports capability-tracked verification (the `--profile` direction Loom anticipates in post-v0). F* is implemented in OCaml, which intersects the formal-methods community well. Microsoft Research and Inria back F*; the language is stable enough for serious use.

**Against F\*.**

- Steeper learning curve. F*'s error messages are widely regarded as opaque even by experienced users.
- The toolchain is more involved to install (OCaml ecosystem, multiple dependencies).
- The community is smaller; finding contributors who can engage with both Loom *and* F* is harder.
- F*'s release cadence is slower; pinning a version means committing to whatever state F* was in at the pin point.
- Translation from Loom to F* is more involved than to Dafny because F*'s syntax and semantics are more expressive (the translator has more choices to make, more cases to handle).

### Direct Z3 (custom refinement-type layer)

**For direct Z3.** No upstream dependency on a research verifier. Full control over refinement semantics. The smallest possible dependency footprint for end users. The most flexible substrate for novel Loom-specific encodings.

**Against direct Z3.** Inventing a refinement-type layer is a substantial original project, well outside v0's scope. Reimplements work that Dafny and F* already do well. Risks turning Loom into a verifier-implementation research project rather than a software-construction-architecture research project. Defers v0 by months or longer.

### Lean 4

**For Lean 4.** Powerful dependent type system, growing community, active development at Microsoft Research. Excellent tactic language. Increasingly used for both math and programming. Strong story for proof reuse and library composition.

**Against Lean 4.** Lean is fundamentally interactive proof; SMT-backed automatic discharge is not its default mode. Loom's claims are intended to be auto-discharged in the common case; using Lean would push the user toward writing proofs by hand, which is incompatible with the LLM-mediated workflow the architecture assumes. Lean's libraries are oriented toward mathematics and pure functional programming rather than the imperative-flavored verification Loom targets.

### Liquid Haskell

**For Liquid Haskell.** Refinement types in a production-grade language. Mature SMT integration. Real-world usage. Good Haskell community engagement.

**Against Liquid Haskell.** Tied to Haskell as both verification target and execution target. Loom's plan separates verification and execution languages (Dafny for verification, Python for execution); Liquid Haskell collapses them. Translation from Loom to Liquid Haskell would require deciding what Haskell types correspond to Loom's types, which conflates Loom's verification model with Haskell's runtime model.

### Why3

**For Why3.** Meta-verifier that targets multiple proof backends (Alt-Ergo, CVC, Z3, Coq, etc.). The multi-backend story is appealing for verifier diversity. Why3's specification language (WhyML) is reasonable.

**Against Why3.** Adds an indirection: Loom → Why3 → backend. Each layer is a place where translation can go wrong or claims can be lost in encoding. The user-facing community is smaller than Dafny's. Tooling is research-grade.

### Dafny

**For Dafny.**

- Refinement types, pre/postconditions, ghost code, automatic SMT discharge — all the primitives Loom needs are first-class.
- Excellent tooling: VS Code extension, well-maintained documentation, useful error messages.
- Active development by Amazon Web Services (the AWS Security team uses Dafny for security-critical code). This is institutional weight that suggests Dafny will not be abandoned soon.
- Stable releases with backwards compatibility commitments.
- Translation from Loom to Dafny is mechanical for the common cases (records → datatypes, refinement types → subset types, operations → function methods, properties → lemmas).
- Documentation is good enough that a contributor who has not used Dafny before can become productive in days, not months.
- Counterexample generation is mature.

**Against Dafny.**

- Limits the expressiveness of `proves` to what Dafny's specification language supports. Higher-order claims, complex ghost state, or domain-specific reasoning may not translate cleanly.
- Couples Loom's semantics to Dafny's idioms. Future divergence is a refactor (mitigated by the `Verifier` trait abstraction, but mitigated only partially because the trait's interface will reflect Dafny's vocabulary).
- Dafny is method-centric: methods with pre/post are the primary unit. Loom's register structure does not map onto methods alone; some encoding effort is required (lemmas for `proves`, function methods for `relates` + `does`, test methods for `shows`).
- Dafny is implemented in C# / .NET. Cross-platform distribution works but the dependency footprint includes a .NET runtime.
- Some claims that are natural in Loom (existentials, higher-rank quantification) are awkward or unsupported in Dafny. These are documented as limitations in `docs/verification-internals.md` §6.

---

## Consequences

### Positive

- v0 reaches a working end-to-end pipeline faster than with F* or direct-Z3.
- Contributors with no formal-methods background can engage with Dafny's well-documented surface.
- The translation effort is well-scoped and mechanical.
- Amazon's institutional usage of Dafny lowers the risk of the project being abandoned.

### Negative

- Loom's expressiveness is capped at Dafny's in v0.
- Future migration to F* (or a direct Z3 layer) is a project on its own.
- The .NET runtime dependency is non-trivial for end-user installation; mitigated by Loom's CI providing platform binaries.

### Neutral

- The `Verifier` trait makes the choice swappable in principle but not free in practice.
- The cross-pollination with the F* community is reduced; this is a real cost for the research thesis but not for v0 delivery.

---

## Migration considerations

If Loom needs to switch backends post-v0, the work involves:

1. Implementing a new crate (e.g., `crates/loom-compile-fstar`) that mirrors `loom-compile-dafny`'s structure.
2. Designing the encoding choices for the new backend (the F* counterpart of `docs/verification-internals.md`).
3. Updating examples that exercise Dafny-specific encoding limitations to use the new backend's stronger features.
4. Retargeting CI to install and pin the new verifier.

This is meaningful but bounded work, on the order of weeks for a single experienced contributor. The decision to switch should be informed by concrete evidence of Dafny's limits biting (e.g., multiple examples that cannot be expressed) rather than theoretical preference.

---

## Implementation notes

- The Dafny encoding is documented in `docs/verification-internals.md`.
- The `Verifier` trait is documented in `docs/verification-internals.md` §1.
- Dafny version pinning is the subject of a future `ADR-0014`.
- Counterexample mapping is implemented in `crates/loom-verify/src/counterexample.rs`.

---

## References

- Dafny home: https://dafny.org/
- Dafny reference manual: https://dafny.org/dafny/DafnyRef/DafnyRef
- F* home: https://www.fstar-lang.org/
- Lean 4: https://lean-lang.org/
- Liquid Haskell: https://ucsd-progsys.github.io/liquidhaskell/
- Why3: https://why3.lri.fr/
