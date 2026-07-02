---
id: D-0006
title: 'loom-loop: qualified PROCEED to build the thin tool'
status: proposed
---
## Question

Does the loom whole-loop dogfood (`E-0004`) — a non-formal author driving prose + examples, blind
subagents authoring the formal umbrella, a verifier + gap report closing the loop, on **real** aiwf
code — deliver enough value to justify building the thin loom-light tool?

## Decision

**Qualified PROCEED.** Across two loops on real aiwf code, the whole umbrella loop turned
end-to-end with the human entirely at the prose / examples / gap-report layer (**zero** Dafny/Go
read), and it delivered genuine, non-obvious value in both directions the bidirectional discipline
predicts:

- **`M-0014` (status-transition FSM, decidable):** push-button and self-diagnosing — every `(B)`
  failure was a real gap. It surfaced a **code** discrepancy (a milestone can be promoted to
  `in_progress` with no acceptance criteria) that the operator filed upstream against aiwf.
- **`M-0015` (id canonicalization, strings):** surfaced real **intent**-vs-code divergences the
  operator independently checked and **accepted** (the emit-wide / accept-narrow conflation), plus
  an intent-vs-intent inconsistency in the operator's own examples.

The proceed is **qualified** by the tractability boundary `M-0015` mapped precisely: on strings,
modeling and concrete-example checking remain tractable (flat *and* recursive), but **blind
universal-property discharge — loom's edge over tests — degrades, and a `(B)` failure stops being
self-diagnosing** (a real gap and a tractability limit are mechanically indistinguishable). So:
**build the thin loom-light tool, scoped first to where it is push-button (decidable / structured
invariants), honest about the string edge, and treating universal-string-proof automation as the
known frontier for extending reach.**

## Reasoning

- **The value is demonstrated, not asserted** — on real code, for a non-formal author, in both the
  code-is-wrong and the intent-is-wrong case. This is loom's actual top-line claim (README:
  "surface the parts that were not checked rather than quietly absorb them") holding up.
- **Successor to the E-0002 / E-0003 pivot.** Those killed the *endogenous-gaming* differentiator
  under pre-registration (`D-0002`, `D-0005`). `E-0004` re-based loom's case on the **visible-gap
  value** differentiator and dogfooded it on real code — and it held. This proceed rests on *that*
  evidence, not the dead gaming hypothesis.
- **The tractability limit is characterized, not fatal.** loom-with-Dafny reaches string-heavy real
  code for modeling and concrete checks; only the universal-proof step needs body-aware help — which
  bounds *where* the thin tool is strongest without blocking the build.
- **Feasibility, not confirmatory.** `E-0004` was an honest dogfood (no pre-registered threshold),
  so this is a judgment over recorded observations (two `gap-report.md`s), re-derivable offline from
  the committed, G1-reproducible `.dfy` artifacts — not a mechanical verdict.
- **Scope of "proceed."** Greenlights *building the thin tool* (the `loom-light` direction), not the
  full grand-loom. The string frontier and the human-factors question are the next things to face,
  per `docs/loom-loop-poc.md` §8–9, and remain out of scope of this dogfood.
