---
id: M-0004
title: Author and pre-register the FSM-transition subject
status: draft
parent: E-0002
depends_on:
    - M-0003
tdd: advisory
acs:
    - id: AC-1
      title: Gold FSM spec + reference impl verify
      status: open
    - id: AC-2
      title: 'Mutant bank: gold kills full bank, isolating mutant per tell'
      status: open
    - id: AC-3
      title: Each obligation probes as isolable single-input goal via M-0003 gate
      status: open
    - id: AC-4
      title: Committed pre-registration with falsifiable verdict mapping
      status: open
---
## Goal

Author the FSM status-transition subject — gold spec, reference implementation, and
clause-isolated mutant bank — and commit its pre-registration, so the two-arm
experiment can run on a relational invariant whose load-bearing obligation is
**negative space** (which transitions are *illegal*).

## Context

E-0002's first subject. aiwf's `internal/entity/transition.go` defines per-kind
legal status transitions; a faithful gold spec must assert not only the legal edges
but the **illegal** ones, terminality-as-no-outgoing, and one-directionality. The
negative-space obligation is exactly where a weak (positive-only) spec hides.
Depends on M-0003's generalized gate to confirm the obligations probe as isolable
single-input goals before the pre-registration is finalized. (aiwf source cloned at
`/tmp/aiwf-src`, github.com/23min/aiwf.)

## Acceptance criteria

Tracked as `acs[]` in frontmatter (AC-1 … AC-4); the full detail lives under the
per-AC sections at the foot of this spec.

## Constraints

- Single-input opaque probe — no quantifier over an entity collection; statuses are
  a bounded finite datatype.
- Clause-isolated mutants (G-0001): each breaks exactly one obligation.
- The pre-registration is committed before M-0006 is promoted to `in_progress`; its
  SHA will be asserted a git ancestor of the run commit in M-0006.

## Design notes

- Subject modeled from the per-kind table in `transition.go`; the exact kinds/edges
  slice is fixed at authoring.
- The predicted tell is the negative-space obligation: `!IsLegal(kind, from, to)`
  for illegal edges (a positive-only spec entails no exclusion → measured weak).

## Surfaces touched

- A new subject directory under `experiments/loom-ultralight/` (gold `.dfy`,
  mutant bank, prompts or reuse, the pre-registration artifact).

## Out of scope

- Running the experiment (M-0006). The prosey subject (M-0005).

## Dependencies

- M-0003 (the generalized strength gate).

## References

- E-0002 epic spec; `/tmp/aiwf-src/internal/entity/transition.go`; D-0001.

---

## Work log

## Decisions made during implementation

- (none)

## Validation

## Deferrals

- (none)

## Reviewer notes

- (none)

### AC-1 — Gold FSM spec + reference impl verify

A gold spec + reference implementation for FSM transition legality exist and
`dafny verify` passes (gold valid against the reference impl). Statuses are a
finite Dafny datatype; legality is an opaque predicate over ground
`(kind, from, to)` tuples.

### AC-2 — Mutant bank: gold kills full bank, isolating mutant per tell

A clause-isolated mutant bank exists; the gold spec **kills the full bank** at
calibration; and the bank contains a mutant **isolating each pre-registered
predicted-tell obligation** — including the negative-space/exclusion obligation —
at the granularity the strength gate distinguishes (the G-0003 guard).

### AC-3 — Each obligation probes as isolable single-input goal via M-0003 gate

Each gold obligation **probes as an isolable single-input goal** through the M-0003
gate (a `StrengthSubject` whose obligations are exclusion goals over ground tuples
and bounded quantifiers over the finite status datatype).

### AC-4 — Committed pre-registration with falsifiable verdict mapping

A **pre-registration artifact is committed** naming the full obligation set, the
obligation(s) predicted to weaken under the incentivized arm, the outcome that
would falsify the prediction, the strength thresholds, **and a total, falsifiable
mapping from this subject's possible run observations into exactly one of
reproduced / not-reproduced / inconclusive (including the inconclusive boundary)**
— so no per-subject verdict judgment remains for after the run. Landed before the
M-0006 run.

