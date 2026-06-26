---
id: M-0006
title: Run the two-arm experiment on both subjects and record the verdict
status: draft
parent: E-0002
depends_on:
    - M-0007
tdd: advisory
---
## Goal

Run the two-arm (disinterested vs incentivized) experiment on both subjects, record
each result against both measures and its pre-registration, and apply the
pre-registered combination rule to produce the epic-level go/no-go on building the
full loom-light pipeline.

## Context

The terminal milestone. Both subjects (M-0004 FSM, M-0005 prosey) are authored,
calibrated, and pre-registered; M-0003's generalized gate measures structural
strength; and M-0007 has committed the combination rule before any run. This
milestone executes the paid runs and records the verdict that discharges D-0001's
re-validation duty and feeds any successor loom-light epic.

## Acceptance criteria

<!-- Candidate ACs; formalized via `aiwf add ac` at start-milestone. -->

- The two-arm experiment is run on both subjects and recorded against **both** the
  mutation kill-rate **and** the generalized structural strength measure;
  inconclusive (timeout) verdicts are reported separately and **never** folded into
  "survived", and the per-arm **validity (over-claim) rate** is reported alongside,
  so the over-claim failure mode (D-0001's two-gate requirement) is surfaced rather
  than silently dropped.
- Each subject's recorded run result **names its pre-registration commit SHA, and
  that SHA is verified to be a git ancestor of the run commit**; the **M-0007
  combination-rule pre-registration commit is likewise verified to be a git ancestor
  of the run commit** (the mechanical pre-registration-precedes-run guard, covering
  both the per-subject and the cross-subject pre-registrations).
- Each subject's result is mapped to its pre-registered edges (reproduced /
  not-reproduced / inconclusive), and the **M-0007 subject-combination rule** is
  applied to yield a single epic-level go/no-go, recorded as a decision.

## Constraints

- Pre-registration ordering is enforced via the git-ancestor check — no result is
  read before the pre-registration is committed.
- The killed / survived / inconclusive trichotomy is preserved (G1); inconclusives
  are surfaced, never scored as survived.
- The paid run requires explicit human go-ahead and `ANTHROPIC_API_KEY` — a hard
  stop the milestone must not auto-trigger.

## Design notes

- Reuse the harness (`run.sh`, `--run`, `--strength`) per subject; record committed
  result artifacts under `experiments/loom-ultralight/results/`.
- The go/no-go decision is recorded via `aiwfx-record-decision` as a project
  decision relating to E-0002, feeding any successor loom-light epic.

## Surfaces touched

- `experiments/loom-ultralight/` (run artifacts, `results/`); a new decision entity.

## Out of scope

- Building the loom-light pipeline (a successor epic, gated on this verdict).

## Dependencies

- M-0007 (the committed combination rule), which transitively requires M-0004 and
  M-0005 (both subjects authored, calibrated, and pre-registered).

## References

- E-0002 epic spec; D-0001 (the duty this discharges).

---

## Work log

## Decisions made during implementation

- (none)

## Validation

## Deferrals

- (none)

## Reviewer notes

- (none)
