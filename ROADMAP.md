# Roadmap

## E-0001 — Validate the loom differentiator (loom-ultralight) (done)

### Goal

Cheaply test the load-bearing hypothesis behind loom **before** building loom-light:
that an LLM authoring a formal spec writes a *weaker* spec when it is also graded on
making its own implementation pass (incentivized) than when it only specifies
(disinterested) — **and** that a mutation / kill-rate check catches the difference.

If the gap appears and the check catches it, loom's differentiator is real and we
proceed to loom-light. If not, we learned it cheaply, before committing to a Rust
engine, a claims surface, or a verifier integration. The full design is in
`docs/loom-ultralight.md`.

| Milestone | Title | Status |
|---|---|---|
| M-0001 | Materialize the loom-ultralight experiment into runnable files | done |
| M-0002 | Run the loom-ultralight experiment and record the kill-rate gap | done |

## E-0002 — Re-validate the loom value-gate on real aiwf invariants (active)

### Goal

Discharge D-0001's binding re-validation duty: reproduce the endogenous
claim-weakening effect on **two fresh aiwf invariants whose obligation textures
differ from id-canonicalization's**, with the discriminating mechanism
**pre-registered after the M-0002 correction** — producing a clean, falsifiable
go/no-go on whether to build the full loom-light pipeline.

| Milestone | Title | Status |
|---|---|---|
| M-0003 | Generalize the structural strength gate to a per-subject obligation list | done |
| M-0004 | Author and pre-register the FSM-transition subject | done |
| M-0005 | Author and pre-register the prosey-title subject | done |
| M-0006 | Run the two-arm experiment on both subjects and record the verdict | draft |
| M-0007 | Pre-register the subject-combination rule and go/no-go procedure | done |

