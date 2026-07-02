---
id: ADR-0017
title: loom-light generates no target code; the form of does is deferred
status: proposed
---
# ADR-0017 — loom generates no target code; the LLM does; the form of `does` is deferred

> **Date:** 2026-06-19 · revised 2026-07-02 · **Deciders:** project initial author; ratified during E-0005 planning.
> **Related:** ADR-0001 (Rust impl) · ADR-0002 (Dafny verifier) · ADR-0003 (Python codegen — rejected) · ADR-0018 (spec↔implementation binding) · [`docs/research/loom-reach-ambition-and-scope.md`](../research/loom-reach-ambition-and-scope.md) · [`docs/loom-loop-poc.md`](../loom-loop-poc.md).

---

## Context

The original plan tangled two decisions: (1) *"loom generates code"* — the codegen commitment; and (2) *"the umbrella carries a `does` register"* — how the umbrella relates to its implementation. They are separate. This ADR settles the first — **permanently** — and leaves the second open.

Originally, codegen was deferred out of loom-light but expected to "return at grand-loom." That is now superseded: **code generation is the LLM's role.** An LLM writes target code more flexibly than any single-target backend loom could ship, so loom never needs one — at any stage.

## Decision

**loom generates no target/executable code — not Python, Go, Rust, or any language — at any stage (loom-light or grand-loom).** Its value is verification: lower the claims to a verifier (ADR-0002), run it, produce the gap report and findings. Code generation, where wanted, is done by the LLM, not by loom.

- The sole artifact loom may itself emit in the future is **`.lm`** (its own claims language) — a specification surface, not target code.
- Consequently **ADR-0003 (Python as codegen target) is rejected**, not merely deferred.
- **grand-loom** remains the follow-up to loom-light, but is rescoped around "the LLM does codegen; loom verifies" — a separate, deliberate rescoping of `PLAN.md`, tracked as a follow-up.

## Deferred (explicitly NOT decided here)

**The form and role of `does`.** `does` need not be inline code. Candidate forms — none chosen — include: a **reference** to a verified Dafny sibling (a proof link); a **reference** to host-language code (an evidence link); **prose** design-intent (a review anchor); an inline **verified Dafny body**; or **omitted** (claims-only). It may be polymorphic over these. Decided *during loom-light, from implementation evidence* — especially what the verifier needs and how a real consumer (aiwf) can actually consume loom. Because the spec↔implementation binding (ADR-0018) and the form of `does` are the same question from two sides, they are deferred **together**.

## Invariants (hold regardless of how `does` later resolves)

1. loom reads the umbrella and the implementation and checks the relationship; it does **not** generate the implementation.
2. The portable claims surface carries no consumer-specific path or binding; dependency direction is consumer → loom, never the reverse.
3. The verified link is a **proof**; the host link is **evidence**. One must not masquerade as the other.
4. A missing or unresolvable implementation **degrades to a clear finding**, never a silent skip.

## Consequences

- loom stays small, host-language-agnostic, and reachable from any consumer; the differentiator (claims + verification + gap report) is the whole of what it ships.
- No codegen target means no audience excluded by language choice — the liability the original codegen commitment carried.
- `PLAN.md` / grand-loom need a rescoping pass to remove the codegen destination; tracked separately, off E-0005's critical path.

## References

- ADR-0001 (Rust impl) · ADR-0002 (Dafny verifier) · ADR-0003 (Python codegen — rejected) · ADR-0018 (binding) · [`docs/research/loom-reach-ambition-and-scope.md`](../research/loom-reach-ambition-and-scope.md) (the graded-verifier direction this reflects).
