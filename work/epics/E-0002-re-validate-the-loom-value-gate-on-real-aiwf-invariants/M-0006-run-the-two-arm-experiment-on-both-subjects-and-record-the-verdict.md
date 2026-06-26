---
id: M-0006
title: Run the two-arm experiment on both subjects and record the verdict
status: in_progress
parent: E-0002
depends_on:
    - M-0007
tdd: advisory
acs:
    - id: AC-1
      title: Both arms run on both subjects, scored by kill-rate and structural strength
      status: open
    - id: AC-2
      title: Prereg and combination-rule SHAs verified as git ancestors of the run commit
      status: open
    - id: AC-3
      title: verdict() and combine() yield a recorded go/no-go Decision
      status: open
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

The three ACs are tracked in frontmatter `acs[]`; each criterion and its evidence is
detailed under its `### AC-N` section below.

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

### AC-1 — Both arms run on both subjects, scored by kill-rate and structural strength

The two-arm (disinterested / incentivized) experiment runs on **both** subjects (FSM,
prosey) through the subject-parameterized harness, and each `(subject, arm)` result is
recorded against **both** measures: the **mutation kill-rate** (against the subject's
committed mutant bank) and the **generalized structural-strength** measure (M-0003's
gate over the subject's obligation set). Inconclusive (Z3-timeout) probes are reported
as their own category and **never** folded into "survived" / "not-entailed" (G1). The
per-arm **validity (over-claim) rate** — the fraction of specs the reference impl
actually verifies against — is reported alongside, so D-0001's two-gate requirement (a
weak spec can pass by over-claiming) is surfaced, not silently dropped.

**Evidence:** committed result artifacts under `experiments/loom-ultralight/results/`
for both subjects; calibration green dry before the paid run.

### AC-2 — Prereg and combination-rule SHAs verified as git ancestors of the run commit

Each subject's recorded run result **names its pre-registration commit SHA**
(`prereg-fsm.md` / `prereg-prosey.md`), and a mechanical check verifies that SHA is a
**git ancestor** of the run commit; the **M-0007 combination-rule prereg**
(`prereg-combination.md`) SHA is **likewise verified** as an ancestor. This is the
pre-registration-precedes-run guard — covering both the per-subject and the
cross-subject pre-registrations — so no result can have been read before its prediction
was committed (the M-0002 integrity lesson, enforced from git, not asserted in prose).

**Evidence:** the recorded result names the three SHAs; a `git merge-base
--is-ancestor` check (committed in the harness or a script) passes for each.

### AC-3 — verdict() and combine() yield a recorded go/no-go Decision

A mechanical **`verdict()`** maps each subject's recorded measures to its
pre-registered edge — **reproduced / not-reproduced / inconclusive** — as a **total
function** of the observation, matching that subject's §6 verdict map exactly
(thresholds V, Δ⁺, Δ⁰, I), pinned by an oracle test the way `combine()` is (not a hand
computation after the run). The two verdicts feed **`combine()`** (M-0007), yielding a
single epic-level **PROCEED / NO-GO / RERUN-OR-EXPAND**, recorded as a **`Decision`**
entity via `aiwfx-record-decision`, relating to E-0002 and discharging D-0001.

**Evidence:** `verdict()` + its oracle test green; the recorded `Decision` entity names
the two per-subject verdicts and the combined decision.

