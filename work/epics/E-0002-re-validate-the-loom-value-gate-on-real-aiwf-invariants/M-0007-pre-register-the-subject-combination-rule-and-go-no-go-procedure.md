---
id: M-0007
title: Pre-register the subject-combination rule and go/no-go procedure
status: in_progress
parent: E-0002
depends_on:
    - M-0004
    - M-0005
tdd: advisory
acs:
    - id: AC-1
      title: Combination-rule pre-registration artifact is committed
      status: open
    - id: AC-2
      title: The rule is total and falsifiable over every verdict pair
      status: open
---
## Goal

Pre-register the cross-subject **combination rule** — how the two per-subject
verdicts (reproduced / not-reproduced / inconclusive) combine into a single
epic-level go/no-go on building the full loom-light pipeline — committed before any
run, so it cannot be tuned to results.

## Context

The terminal decision (D-0001's discharge) rests on combining two per-subject
results. Each subject's pre-registration (M-0004, M-0005) defines when *that*
subject reproduces; this milestone pre-registers the rule that maps the *pair* of
categorical verdicts to proceed/no-go. It depends on both subjects being authored
and calibrated (so the rule is informed by what they turned out to be) but lands
before M-0006's run — a hard milestone boundary, the same integrity the per-subject
pre-registrations get.

## Acceptance criteria

<!-- Candidate ACs; formalized via `aiwf add ac` at start-milestone. -->

- A committed pre-registration artifact states the **subject-combination rule**: the
  mapping from the pair of per-subject verdicts (reproduced / not-reproduced /
  inconclusive) to a single epic-level go/no-go — including how a **mixed** result
  (one reproduces, the other does not or is inconclusive) is resolved, and what
  combined outcome counts as a no-go.
- The rule is **total and falsifiable** — it yields a definite outcome (proceed /
  no-go / an explicit "inconclusive → rerun-or-expand") for **every** combination of
  the two subjects' possible verdicts, leaving no residual judgment to be exercised
  after the run.
- The artifact is committed before M-0006 is promoted to `in_progress` (enforced by
  the M-0006 → M-0007 dependency), and its commit SHA is recorded for M-0006's
  git-ancestor check.

## Constraints

- Committed before any run; no result is visible when this is authored — the
  dependency edge plus the M-0006 ancestor guard enforce the boundary.
- Total and falsifiable: every verdict pair maps to a defined outcome.

## Design notes

- The rule operates on the categorical per-subject verdicts, which are defined by
  each subject's own pre-registration (M-0004, M-0005) — so it is subject-agnostic in
  form, but committed after the subjects exist so it can reflect them.
- It records the decision *procedure*; M-0006 mechanically applies it and records the
  actual decision entity.

## Out of scope

- Running the experiment or recording the decision entity (M-0006).
- The per-subject "reproduced" criteria (set in M-0004, M-0005).

## Dependencies

- M-0004 and M-0005 (both subjects authored, calibrated, and pre-registered).

## References

- E-0002 epic spec; D-0001 (the duty this epic discharges).

---

## Work log

## Decisions made during implementation

- (none)

## Validation

## Deferrals

- (none)

## Reviewer notes

- (none)

### AC-1 — Combination-rule pre-registration artifact is committed

### AC-2 — The rule is total and falsifiable over every verdict pair

