---
id: M-0005
title: Author and pre-register the prosey-title subject
status: in_progress
parent: E-0002
depends_on:
    - M-0003
tdd: advisory
acs:
    - id: AC-1
      title: Gold prosey spec and reference implementation verify
      status: open
    - id: AC-2
      title: Clause-isolated mutant bank that the gold spec kills
      status: open
    - id: AC-3
      title: Each gold obligation probes as an isolable goal through the gate
      status: open
---
## Goal

Author the prosey-title subject — gold spec, reference implementation, and
clause-isolated mutant bank — and commit its pre-registration, so the two-arm
experiment can run on a boolean-predicate invariant with five isolable obligations,
the subtle one being the multi-sentence-boundary rule.

## Context

E-0002's second subject, parallel to M-0004. aiwf's `IsProseyTitle`
(`internal/entity/entity.go`) is a pure `string → bool` rejecting prosey/invalid
titles via five checks: over-length, embedded newline, markdown markers, link
brackets, and a multi-sentence boundary (sentence-mark + space + capital, with an
off-by-one rune window). It is the natural fit for the strength gate's single-input
predicate probe, and the multi-sentence rule is where a weak spec hides. Depends on
M-0003's generalized gate. (aiwf source at `/tmp/aiwf-src`.)

## Acceptance criteria

<!-- Candidate ACs; formalized via `aiwf add ac` at start-milestone. -->

- A gold spec + reference implementation for prosey-title detection exist and
  `dafny verify` passes. The subject is an opaque predicate `IsProsey(t)` over a
  single string; each of the five checks is a single-input obligation.
- A clause-isolated mutant bank exists; the gold spec **kills the full bank** at
  calibration; and the bank contains a mutant **isolating each pre-registered
  predicted-tell obligation** at the granularity the strength gate distinguishes
  (the G-0003 guard), including the multi-sentence rule.
- Each gold obligation **probes as an isolable single-input goal** through the
  M-0003 gate.
- A **pre-registration artifact is committed** naming the full obligation set, the
  obligation(s) predicted to weaken (the multi-sentence rule is the candidate), the
  falsifying outcome, the strength thresholds, **and a total, falsifiable mapping
  from this subject's possible run observations into exactly one of reproduced /
  not-reproduced / inconclusive (including the inconclusive boundary)** — so no
  per-subject verdict judgment remains for after the run. Landed before the M-0006
  run.

## Constraints

- Single-input opaque predicate over a string; no collection or state.
- Clause-isolated mutants (G-0001): each breaks exactly one of the five checks.
- The pre-registration is committed before M-0006 is promoted to `in_progress`; its
  SHA will be asserted a git ancestor of the run commit in M-0006.

## Design notes

- Subject modeled from `IsProseyTitle` in `entity.go` — the five checks map to five
  obligations; the multi-sentence-boundary rule (threshold ≥ 1, off-by-one rune
  window) is the subtle obligation and the predicted tell.

## Surfaces touched

- A new subject directory under `experiments/loom-ultralight/` (gold `.dfy`,
  mutant bank, the pre-registration artifact).

## Out of scope

- Running the experiment (M-0006). The FSM subject (M-0004).

## Dependencies

- M-0003 (the generalized strength gate).

## References

- E-0002 epic spec; `/tmp/aiwf-src/internal/entity/entity.go` (`IsProseyTitle`); D-0001.

---

## Work log

## Decisions made during implementation

- (none)

## Validation

## Deferrals

- (none)

## Reviewer notes

- (none)

### AC-1 — Gold prosey spec and reference implementation verify

### AC-2 — Clause-isolated mutant bank that the gold spec kills

### AC-3 — Each gold obligation probes as an isolable goal through the gate

