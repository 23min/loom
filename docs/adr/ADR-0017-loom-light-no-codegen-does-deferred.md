---
id: ADR-0017
title: loom-light generates no target code; the form of does is deferred
status: proposed
---

# ADR-0017 — loom-light generates no target code; the form of `does` is deferred

> **Date:** 2026-06-19 · **Deciders:** project initial author; open for review.
> **Status note:** aiwf status `proposed` covers the no-codegen decision; the `does`-form question remains **open**, deferred to loom-light implementation evidence.
> **Relationship to ADR-0003:** defers ADR-0003 (Python as target) *out of loom-light scope*; does **not** supersede it.
> **Stage:** a **loom-light** decision (see [`docs/loom-light.md`](../loom-light.md)), not a decision about the grand-loom destination in `PLAN.md`. "loom-light" is a named stage, deliberately not "v0".
> **Related:** `PLAN.md` (the grand-loom destination) §1.2, §2.1, §4.4, §5.3, §6; ADR-0002 (Dafny verifier); ADR-0018 (spec↔implementation binding); `docs/loom-light.md`.

---

## Context

`PLAN.md` describes the **destination** ("grand-loom"): an umbrella with five registers including `does`, where `does` is implementation that a code generator emits to a target language (Python — ADR-0003), per §1.2 / §2.1 / §4.4.

**loom-light** (`docs/loom-light.md`) is a **stage on the path to that destination, not a replacement** — an *evolution*, not a fork, and possibly itself preceded by a smaller loom-ultralight PoC. This ADR scopes only what loom-light commits to; it leaves the destination intact and does **not** edit `PLAN.md` or the spec docs, which legitimately continue to describe grand-loom.

Two things had been tangled and must be separated:

1. **"Loom generates code"** — the codegen commitment.
2. **"The umbrella carries a `does` register"** — how the umbrella relates to its implementation.

These are different decisions. The first is settled here for loom-light; the second is explicitly left open.

---

## Decision (what loom-light commits to)

**loom-light generates no target/executable code.** There is no `crates/loom-compile-python` and no codegen step in the loom-light pipeline. Its value is verification — lower the claims to Dafny (ADR-0002), run it, produce the gap report — plus the vacuity check and findings.

Rationale:
- A single codegen target excludes every consumer not in that language. For a tool meant to be used *mostly by downstream consumers*, that excludes most of the audience by construction.
- Committing to codegen is an identity-level commitment, expensive to unwind — the costly-pivot pattern the project is avoiding.
- Removing it from loom-light costs nothing now (no code exists) and keeps the stage reachable from any host language.

Consequently, **ADR-0003 (Python as target) is deferred out of loom-light, not superseded.** Codegen, if built, returns at the grand-loom stage as the destination intends, and ADR-0003's analysis stands for that point.

---

## Deferred (explicitly NOT decided here)

**The form and role of `does` in loom-light.** `does` need not be inline-code-that-codegens. Candidate forms — none chosen — include:

- a **reference** to a verified Dafny sibling (a proof link);
- a **reference** to host-language code (an evidence link);
- **prose** design-intent (a review anchor / authoring input — not a proof);
- an inline **verified Dafny body** (co-located, maximal assurance, but re-bloats the umbrella);
- **omitted** (claims-only).

`does` may even be *polymorphic* over these. This is a major design decision, and per the project's anti-pivot posture it will be made **during loom-light, from implementation evidence — especially what the verifier needs and how aiwf (the first real consumer) can actually consume loom.** Iterate / decide / fork *then*, not now.

Because the spec↔implementation binding mechanism (ADR-0018) and the form of `does` are the same question seen from two sides (`does: ref …` *is* a binding), they are deferred **together**.

---

## Invariants (hold regardless of how `does` later resolves)

1. **In loom-light, loom does not generate the implementation.** It reads the umbrella and the implementation and checks the relationship.
2. **The portable claims surface carries no consumer-specific path or binding.** Dependency direction is consumer → loom, never the reverse; the surface stays usable standalone.
3. **The verified link is a proof; the host link is evidence.** One must not masquerade as the other.
4. **A missing or unresolvable implementation degrades to a clear finding,** never a silent skip.

---

## Consequences

- loom-light stays small and host-language-agnostic; the differentiator (claims + verification + vacuity check) is the whole of what it ships.
- The destination (`PLAN.md`, the spec docs) is untouched and remains the evolutionary target. The divergence is intentional: **`PLAN.md` = destination; this ADR + `docs/loom-light.md` = the loom-light stage.** A reader reconciling the two should treat loom-light as a subset stage, not a contradiction.
- This ADR is not registered in `PLAN.md` §9 (the destination's ADR backlog) by design. It belongs to `docs/loom-light.md`, the home for loom-light-stage decisions (with ADR-0018).

---

## References

- `docs/loom-light.md` — the stage this ADR scopes.
- `PLAN.md` §1.2, §2.1, §4.4, §5.3, §6 — the grand-loom destination this ADR scopes down from.
- ADR-0002 (Dafny verifier) — the verification path, unaffected.
- ADR-0003 (Python as target) — deferred out of loom-light by this ADR.
- ADR-0018 (spec↔implementation binding) — the binding mechanism, deferred jointly with the `does`-form question.
