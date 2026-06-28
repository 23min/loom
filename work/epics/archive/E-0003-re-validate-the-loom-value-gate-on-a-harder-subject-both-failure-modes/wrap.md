# Epic wrap — E-0003

**Date:** 2026-06-28
**Closed by:** human/peter
**Integration target:** main
**Epic branch:** epic/E-0003-re-validate-the-loom-value-gate-on-a-harder-subject-both-failure-modes
**Merge commit:** `f2fc781`

## Milestones delivered

Every milestone in the epic spec's *Milestones* section reached `done`:

- `M-0008` — Harden the loom-ultralight harness (merged `1032281`)
- `M-0009` — Design the id-reallocation subject (merged `d38da0a`)
- `M-0010` — Author the two-dimension pre-registration (merged `386d548`)
- `M-0012` — Harden the validity gate for executable-spec subjects (merged `c5d201d`)
- `M-0013` — Harden the spec extractor for complex executable specs (merged `036975d`)
- `M-0011` — Run the reallocate sweep and record the terminal decision (merged `2af4411`)

## Summary

E-0003 re-tested the loom value-gate hypothesis — that an incentivized LLM writes materially
weaker specs — on a genuinely harder, decidable aiwf invariant (id-reallocation /
reference-rewrite), pre-registering **both** failure modes (under-specification *and*
over-claiming) in a two-dimension §6 verdict frozen at `bb1d220` before the run. Two
construct-validity flaws surfaced by the first smoke (`G-0006`, `G-0007`) were closed first:
the validity gate became a hybrid that executes a candidate's `ensures` over a concrete-tree
battery when `dafny verify` rejects it (`D-0003`), and the spec instrument was certified to a
bounded, adversarially-reviewed residual with no false-valids (`D-0004`). On the certified
gate, the recorded N=30 × three-model × two-arm sweep returned a terminal **NO-GO** (`D-0005`):
on the pre-registered primary `opus-4.8` neither failure mode reproduced (both arms 30/30
valid; tell and over-claim gaps 0.0). The loom value-gate is now not-reproduced on the primary
across four subjects (fsm, prosey, and reallocate's two dimensions). The result is mechanical,
re-derivable offline, and scoped to the self-contained Dafny model of the invariant.

## ADRs ratified

- none — the epic's architectural reasoning is captured in the decisions below. `D-0003` (the
  hybrid validity-gate architecture) and `D-0004` (the certified instrument + residual bound)
  are the durable, load-bearing records; no separate ADR was minted, to avoid duplicating them.

## Decisions captured

- `D-0003` — Hybrid validity gate: `dafny verify` with concrete-tree execution fallback (accepted)
- `D-0004` — Over-claim instrument certified; residual bound accepted (accepted)
- `D-0005` — Reallocate two-dimension verdict: NO-GO, neither failure mode reproduced (accepted)

## Follow-ups carried forward

- none — `G-0004` and `G-0005` were closed by `M-0008`; `G-0006` by `M-0012`; `G-0007` by
  `M-0013`. No gap survives the epic. (`G-0004` / `G-0005` remain in the active tree pending the
  next `aiwf archive --apply` sweep — housekeeping, not open work.)

## Handoff

The loom-ultralight harness now carries a certified, two-dimension over-claim instrument and a
hybrid validity gate — the foundation a future loom-light epic would build on. The terminal
NO-GO means no such epic is greenlit on this evidence: the value-gate hypothesis is not
supported on the primary model. A PROCEED would have motivated transfer to the full aiwf
runtime; a NO-GO does not, and per the no-subject-shopping constraint this one recorded subject
stands as the result. The decidable-regime instrument and the prereg-ancestry discipline are
reusable for any future re-validation.
