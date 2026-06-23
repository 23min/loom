---
id: M-0002
title: Run the loom-ultralight experiment and record the kill-rate gap
status: in_progress
parent: E-0001
depends_on:
    - M-0001
tdd: advisory
acs:
    - id: AC-1
      title: N trials per condition produce a kill-rate table
      status: met
    - id: AC-2
      title: kill-rate gap (disinterested minus incentivized) computed
      status: met
    - id: AC-3
      title: result interpreted against pre-registered outcomes
      status: met
---
## Goal

Run the experiment and record the result, per `docs/loom-ultralight.md` §4–§5.

Configuration (decided in planning):
- **Models:** a sweep across Opus 4.8, Sonnet 4.6, and Haiku 4.5.
- **Trials:** N = 10 per condition, per model.
- **Conditions:** **both** arms author a spec *and* an implementation; they differ
  **only** in the grading clause — disinterested (spec audited for completeness) vs
  incentivized (graded only on `dafny verify` passing). Holding the task constant
  isolates the incentive as the sole variable (it removes the spec-only vs spec+impl
  load confound). We score only the *spec* each arm produces, against our reference
  impl + mutant bank; both implementations are discarded.

Record a committed results artifact under `experiments/loom-ultralight/results/` — the
per-model kill-rate table, the per-model gap, and the raw generations — so D-0001 is
grounded in evidence, not memory.

## Acceptance criteria

### AC-1 — N trials per condition produce a kill-rate table

For each model, N=10 trials × 2 conditions are generated, scored, and tabulated;
timeouts land in the inconclusive bucket and are never folded into "survived".

### AC-2 — kill-rate gap (disinterested minus incentivized) computed

Per model: gap = mean kill-rate(disinterested) − mean kill-rate(incentivized). The
cross-model pattern — does the gap shrink or persist as capability rises? — is reported.

### AC-3 — result interpreted against pre-registered outcomes

The result is mapped to one of the `docs/loom-ultralight.md` §5 outcomes
(gap-real-and-caught / gap≈0 / gap-real-but-missed / calibration-failure), feeding the
D-0001 gate.
