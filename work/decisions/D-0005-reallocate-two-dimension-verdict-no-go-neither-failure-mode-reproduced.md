---
id: D-0005
title: 'Reallocate two-dimension verdict: NO-GO (neither failure mode reproduced)'
status: proposed
---
## Question

Does the loom incentive distort spec quality — by **under-specification** (E-0002's predicted
tell) or **over-claiming** (E-0002's recorded-but-un-pre-registered lead) — on a genuinely harder
aiwf invariant (`reallocate`), measured by the pre-registered two-dimension §6 verdict
(`prereg-reallocate.md`, frozen at `bb1d220`)?

## Decision

**NO-GO.** The frozen `reallocate_verdict`, applied to the recorded N=30 × three-model × two-arm
sweep (run commit `595d3dd`, on the `M-0013`-certified gate), returns **terminal = NO-GO**,
anchored on the pre-registered primary `opus-4.8`. On the primary, **both** failure modes are
**not-reproduced**:

- **Under-specification — not reproduced.** Disinterested and incentivized arms both entail the
  load-bearing tell (`refs_rewritten`) at rate 1.0; `tell_gap = 0.0` (≪ the 0.20 material gap).
- **Over-claiming — not reproduced.** Both arms `over_claim_rate = 0.0` (30/30 valid);
  `over_claim_gap = 0.0` (≪ the 0.20 material rise).
- Inconclusive 0.0 (no Z3/execution timeouts).

The incentivized arm wrote specs exactly as strong and as valid as the disinterested arm. The two
sweep models (`sonnet-4.6`, `haiku-4.5`) — pre-registered as generalization evidence, not gating —
also return NO-GO.

## Reasoning

- **Mechanical, not judged.** The decision is `reallocate_verdict`'s output over the recorded
  artifacts, re-derivable offline with no API call (`--strength <runs-dir>` re-reads
  `results.json` + the saved responses and recomputes the identical `verdict.json`). No residual
  judgment was exercised after results were visible; the thresholds, combination rule, and
  predictions were all committed before the run (`bb1d220`, a git-ancestor of `595d3dd` — verified
  by `--check-prereg-ancestry`).
- **The result is sound, not an instrument artifact — this is what `M-0013` bought.** On the
  primary the over-claim instrument had a **0% unexecutable residual in this run** (30/30 valid,
  both arms), so `over_claim_rate = 0.0` is a genuine "the reference impl satisfies these specs,"
  not the gate failing to decide. Before `M-0013`/`D-0004`, the opus disinterested arm carried a
  spurious ~20% over-claim from the `<==>`-precedence artifact (`G-0007`), which would have
  manufactured a confounded signal. The certified, adversarially-reviewed, error-bounded
  instrument (`D-0004`) is what makes this NO-GO trustworthy.
- **Evidence-only residual, surfaced.** The weakest sweep model `haiku-4.5` carries a 27%
  disinterested unexecutable residual (the genuine undecidable class `D-0004` bounds) — a visible,
  per-arm-census signal, not silently folded; it does not gate the primary-anchored decision and
  is NO-GO regardless.
- **Consistency with E-0002.** `D-0002` returned NO-GO on the FSM and prosey subjects (the
  predicted claim-weakening did not reproduce). E-0003 set out to re-test that on a harder subject
  while pre-registering BOTH failure modes — and the predicted incentive-distortion again does not
  reproduce, now on a decidable-regime id-reallocation invariant with a certified two-dimension
  instrument. The loom value-gate hypothesis (an incentivized LLM writes materially weaker specs)
  is not supported on the primary model across four subjects.
- **Scope.** This discharges the §6 question on this self-contained Dafny *model* of the
  reallocation invariant; transfer to the full aiwf runtime is out of scope (a PROCEED would have
  motivated that; a NO-GO does not). Per the epic's no-subject-shopping constraint, this one
  recorded subject stands as the result.
