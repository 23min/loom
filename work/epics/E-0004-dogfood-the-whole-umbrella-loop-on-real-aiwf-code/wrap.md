# Epic wrap — E-0004

**Date:** 2026-07-02
**Closed by:** human/peter
**Integration target:** main
**Epic branch:** epic/E-0004-dogfood-the-whole-umbrella-loop-on-real-aiwf-code
**Merge commit:** `a900638`

## Milestones delivered

Every milestone in the epic spec's *Milestones* section reached `done`:

- `M-0014` — Turn the umbrella loop on the status-transition FSM (merged `4362914`)
- `M-0015` — Turn the loop on real string-based canonicalization (merged `93bffe9`)

## Summary

E-0004 dogfooded loom's whole umbrella loop — a non-formal author writing prose + examples, blind
subagents authoring the formal umbrella, a Dafny verifier + gap report closing the loop — on
**real** aiwf code, testing the direction the E-0002/E-0003 pivot pointed to (the **visible-gap
value** differentiator, not the dead gaming one). Two loops: the decidable status-transition FSM
(`M-0014`) and string-based id-canonicalization (`M-0015`, laddered flat → recursive). The loop
turned end-to-end both times with the human entirely at the prose / gap-report layer (**zero**
Dafny/Go read), and delivered genuine value in both directions the bidirectional discipline
predicts — a **code** gap filed upstream (`M-0014`) and **intent** errors the operator accepted
(`M-0015`, the emit-wide/accept-narrow conflation). `M-0015` also mapped the tractability boundary
precisely: on strings, modeling + concrete-checking are tractable, but blind universal-property
discharge degrades and a `(B)`-failure stops self-diagnosing. Terminal decision `D-0006`: **qualified
proceed** to build the thin tool, scoped first to where the loop is push-button.

## ADRs ratified

- none — the epic's reasoning is captured in [`docs/loom-loop-poc.md`](../../../docs/loom-loop-poc.md)
  (the whole-loop design + the five-register umbrella convention) and decision `D-0006` (the terminal
  go/no-go). No separate ADR was minted, to avoid duplication.

## Decisions captured

- `D-0006` — loom-loop: qualified PROCEED to build the thin tool (accepted)

## Follow-ups carried forward

- None as **loom** gaps. `M-0014` surfaced a candidate **aiwf** gap (a milestone can be promoted to
  `in_progress` with no acceptance criteria), filed upstream against aiwf — not a loom gap entity.
  The string universal-proof frontier and the human-factors loop (`docs/loom-loop-poc.md` §8–9) are
  candidate successor-epic scope, not open gaps.

## Handoff

The thin loom-light tool is greenlit (qualified). Scope it first to decidable / structured invariants
(where the loop is push-button + self-diagnosing), honest about the string edge, with
universal-string-proof automation as the known frontier for extending reach. The reusable foundation:
the whole-loop mechanics, the five-register umbrella convention, blind-subagent authorship (which
enforces the loom blinding at zero metered-API cost), and the G1-reproducible verify-and-gap-report
artifacts. Deliberately still open: real-code tractability at scale, and the human-factors loop.
