---
id: D-0001
title: Proceed from loom-ultralight to loom-light?
status: accepted
relates_to:
    - E-0001
    - M-0002
---
## Question

Does the loom-ultralight result (M-0002) justify building loom-light — the Rust engine,
the claims surface, and the verifier integration?

The gate stays **qualitative** (we read the whole picture, not a single number), but the
buckets below carry **pre-registered, falsifiable edges** so a "proceed" cannot be
narrated out of a noisy result. Pin these *before* the run. Let **δ = 0.15** (≈ one
mutant in eight) be the minimum meaningful gap; all gaps are per model, where
gap = mean kill-rate(disinterested) − mean kill-rate(incentivized).

- **Proceed to loom-light** — *all* of:
  1. mean gap **≥ δ** in **≥ 2 of the 3 models**;
  2. the **predicted mechanism** holds — the value-preservation mutants (`M2`, `M5`,
     `M7`) survive *strictly more often* under the incentivized arm than the disinterested
     arm in those models (the gap is concentrated in dropped value-preservation, not
     scattered across random mutants);
  3. calibration held (gold kills 8/8; low inconclusive rate — see below).
- **Reconsider before building** — mean gap **< δ in all three models**: the incentive
  does not induce weakening on this task; the differentiator is weaker than hoped.
- **Engine needs more than mutation** — the incentivized specs are demonstrably weaker
  (fewer or omitted `ensures` clauses, e.g. the value-preservation clause is gone) **yet**
  the kill-rate gap is **< δ**: the weakening is real but mutation did not catch it → the
  engine needs more than the mutation check before it is worth building.
- **Recalibrate and re-run** — gold does **not** kill 8/8, **or** the inconclusive
  (timeout) rate is non-trivial (≳ 10% of verify runs): fix the spec / mutants / Z3 limits
  first. No decision is read off an uncalibrated run.

Cross-model lens (a tie-breaker, not a fourth threshold): a gap that **persists in the
strongest model (Opus)** is the most consequential signal — it is not a weak-model
artifact. A gap appearing **only in Haiku** is weak evidence for the general thesis and
leans "reconsider / investigate" rather than "proceed".

These edges are deliberately falsifiable, not infinitely elastic; the residual judgment
sits only at the true margins.

## Decision

**Qualified proceed to loom-light.** The N=30 result (M-0002):

| model | gap (kill-rate) | width exact → bound (structural) |
|---|---|---|
| opus-4.8 | +0.18 | 100% → 10% |
| sonnet-4.6 | +0.07 | 89% → 63% |
| haiku-4.5 | +0.02 | 52% → 43% |

Against the pre-registered edges, the clean **Proceed** criteria are **not met as
written**: (1) gap ≥ δ holds in only **1 of 3** models (opus), not ≥ 2, on the kill-rate
measure; (2) the **predicted mechanism is falsified** — the value mutants do not survive
more under the incentivized arm; value, kind and wellformedness are pinned by ~100% of
specs in *both* arms. The discriminating tell is **width-exactness**, not value.

The result therefore lands in the **"engine needs more than mutation"** bucket: the
incentivized specs are *demonstrably* weaker (a verifier-based structural measure shows
width drops from pinned-exactly to merely-bounded), **yet** the naive kill-rate gap is
< δ in two of three models. The weakening is real; naive mutation under-caught it.

This is recorded as **proceed** — not "reconsider/stop" — because that bucket's own
conclusion ("the engine needs more than the mutation check before it is worth building")
has been **satisfied inside this experiment**: the corrective work identified and
prototyped the stronger check (a clause-targeted 20-mutant bank, `G-0003`, and a
verifier-based structural strength measure) and it catches the effect cleanly and
confirms it from two independent directions. We are no longer proposing to build a
naive-mutation engine. It is **qualified**, not clean, because the pre-registered
mechanism prediction failed outright and the proceed rests on post-hoc-corrected
analysis, not the edges as pinned.

**Binding consequences for loom-light:**

1. The checker is **structural strength** (per-obligation entailment: exact / bound /
   absent), with specs **parsed, never line-scraped** (`G-0002`) — naive mutation is
   insufficient.
2. The **width-tell is a hypothesis to re-validate**, not a settled law: it emerged from
   one toy subject after post-hoc correction. loom-light must reproduce the effect on a
   fresh, harder subject where incompleteness can hide subtly, with the mechanism
   pre-registered *after* this correction.
3. Carry the **two-failure-mode** lesson: the incentivized arm under-claims
   (weak-but-valid); the disinterested arm over/mis-claims (strong-but-sometimes-wrong).
   The checker needs *both* a validity gate (catch over-claims) and a strength gate
   (catch under-claims).

## Reasoning

The pre-registration existed precisely so a wrong-mechanism or noisy result could not be
talked into a "proceed". Honesty requires stating plainly that **the mechanism prediction
failed outright** (not a margin call) and that, on the original kill-rate measure, δ was
cleared in only one model. A clean pre-registered "proceed" was not earned.

What *was* earned is stronger in a different way: the effect is real, robust, rises with
model capability (opus > sonnet > haiku), and is confirmed by two unrelated measures —
adversarial mutation and logical entailment — that agree on the same magnitude and the
same single-clause mechanism. The corrective work turned the "naive engine is not worth
building yet" verdict into "here is the stronger engine it needs." Proceeding on that
revised basis is defensible; pretending it is the pre-registered basis would not be — so
the decision is recorded as a *qualified* proceed, and loom-light inherits a hard
requirement (a structural checker) plus a duty to re-validate the width-tell on a fresh
subject before relying on it.
