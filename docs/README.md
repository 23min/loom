# loom docs — index

loom is at the planning / PoC stage: the architecture is specified but the
code does not yet exist. The **active** work is the staged plan below; the
rest is reference, foundations, and decisions.

## Start here — the active plan

- [`loom-ultralight.md`](loom-ultralight.md) — the PoC experiment that gates
  everything else: does an incentivized LLM weaken its own spec, and does a
  mutation check catch it? Run on a model of a real aiwf invariant.
- [`loom-light.md`](loom-light.md) — the thin, verification-first stage toward
  the umbrella architecture (the near-term destination, gated on the PoC).

The far destination (grand-loom) is the root [`../PLAN.md`](../PLAN.md).

## `reference/` — grand-loom spec & internals

- [`reference/language-reference.md`](reference/language-reference.md) — Loom surface syntax.
- [`reference/claims-reference.md`](reference/claims-reference.md) — the claim forms, per register.
- [`reference/verification-internals.md`](reference/verification-internals.md) — how umbrellas translate to Dafny.
- [`reference/bidirectional-refinement.md`](reference/bidirectional-refinement.md) — the gap-report discipline.
- [`reference/compositional-correctness.md`](reference/compositional-correctness.md) — correctness across the umbrella tree.
- [`reference/spec-quality.md`](reference/spec-quality.md) — `specq` and weak-claim detection.
- [`reference/llm-operations.md`](reference/llm-operations.md) — distill / generate / summarize.

## `engineering/` — how we build loom

- [`engineering/principles.md`](engineering/principles.md) — principles for a
  healthy codebase; the code-quality bar for loom's own code (wired into
  [`../CLAUDE.md`](../CLAUDE.md)).
- [`engineering/rethink-stopgap.md`](engineering/rethink-stopgap.md) — interim
  value-gate practice for building with an LLM agent *before* loom exists.

## `research/` — foundations

- [`research/verifiable-umbrella-paper-v2.md`](research/verifiable-umbrella-paper-v2.md) — the architecture paper.
- [`research/process-gates-and-value-gates.md`](research/process-gates-and-value-gates.md) — why a *value* gate is the thing worth approximating.
- [`research/spec-quality-under-llm-authorship.md`](research/spec-quality-under-llm-authorship.md) — threat model + techniques for LLM-authored specs.
- [`research/containment-not-solution.md`](research/containment-not-solution.md) — the reliability frame: contain, don't solve.

## `adr/` — decisions (aiwf-managed)

Architecture Decision Records, managed as aiwf entities (`docs/adr/ADR-NNNN-*.md`):
ADR-0001 (Rust), ADR-0002 (Dafny), ADR-0003 (Python target — deferred),
ADR-0004 (no actors in v0), ADR-0017 (loom-light: no codegen; `does` deferred),
ADR-0018 (spec↔implementation binding). `aiwf history ADR-NNNN` reconstructs each.

## Planning & provenance

loom dogfoods [aiwf](https://github.com/23min/aiwf) for its own planning;
entities live under `work/`, the roadmap is `aiwf render roadmap`, and every
state change is provenance-tracked in git trailers.
