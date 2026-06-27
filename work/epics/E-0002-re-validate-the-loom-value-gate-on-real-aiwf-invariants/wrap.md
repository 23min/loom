# Epic wrap — E-0002

**Date:** 2026-06-27
**Closed by:** human/peter
**Integration target:** main
**Epic branch:** epic/E-0002-re-validate-the-loom-value-gate-on-real-aiwf-invariants
**Merge commit:** abaf8f9

## Milestones delivered

- M-0003 — Generalize the structural strength gate to a per-subject obligation list (merged 6d8f23b)
- M-0004 — Author and pre-register the FSM-transition subject (merged 0b51268)
- M-0005 — Author and pre-register the prosey-title subject (merged 1de1662)
- M-0007 — Pre-register the subject-combination rule and go/no-go procedure (merged 15dc5f9)
- M-0006 — Run the two-arm experiment on both subjects and record the verdict (merged c299a93)

## Summary

Discharged D-0001's binding re-validation duty. Generalized the loom-ultralight strength
gate to any registered subject, authored and pre-registered two fresh, harder aiwf
invariants whose obligation textures differ from id-canonicalization's — the status-transition
FSM (negative-space tell) and the prosey-title check (multi-sentence-rule tell) — and
pre-registered the cross-subject combination rule, all before any run. The two-arm
experiment (opus-4.8, N=30/arm) then found the endogenous claim-weakening effect did **not**
reproduce on either subject: the tell gaps (0.019 / 0.000) were an order of magnitude below
the pre-registered material-gap threshold Δ⁺=0.20, well-powered and with low inconclusive
rates. Per the combination rule, that is **NO-GO** (D-0002). The pre-registration discipline
delivered a clean falsification rather than a narrated proceed.

## ADRs ratified

- none — the outcome is a project-scoped decision (D-0002); the methodology (structural
  strength over naive mutation, pre-registration after correction, the two-gate validity +
  strength requirement) was already set by D-0001 and the per-subject pre-registrations.

## Decisions captured

- D-0002 — Build loom-light — does the gaming effect re-validate on fresh subjects? **NO-GO.**
  Discharges D-0001's re-validation duty with a negative; the qualified-proceed precondition
  for relying on the effect is, on this evidence, unmet.

## Follow-ups carried forward

- G-0004 — Unify loom-ultralight model-filtering across the kill-rate and strength outputs;
  make `verdict.json` self-contained.
- G-0005 — Gate the strength entailment population to valid specs (close a dormant ex-falso
  confound) and make `probe_spec`'s timeout routing unit-testable.

## Handoff

loom-light is **not** greenlit on this evidence. A real but un-pre-registered failure mode
did appear — the FSM incentivized arm over-claimed (15/30 valid vs 29/30 disinterested),
caught by the validity gate exactly as D-0001's two-failure-mode lesson anticipated — and is
recorded, not scored. The legitimate next step is a **successor study**: its own epic, its own
pre-registration committed before its run, on a genuinely more complex subject, pre-registering
**both** failure modes (under-specification and over-claiming), with G-0004 / G-0005 closed
first. It must not be a post-hoc expansion of E-0002 — the combination rule returned a terminal
NO-GO, not RERUN-OR-EXPAND, and iterating fresh subjects until one reproduces is the
subject-shopping this epic exists to forbid.

The loom-ultralight harness is now subject-parameterized and reusable (`LOOM_SUBJECT`,
`LOOM_MODELS`); the M-0002 canonicalize subject and its golden fixtures are preserved
byte-for-byte. The recorded NO-GO reproduces offline: `cargo run -- --decide
results/E-0002/fsm results/E-0002/prosey`.
