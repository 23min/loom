---
id: M-0003
title: Generalize the structural strength gate to a per-subject obligation list
status: draft
parent: E-0002
tdd: advisory
acs:
    - id: AC-1
      title: Strength gate driven by per-subject obligation spec
      status: open
    - id: AC-2
      title: Canonicalize N=30 strength matches golden fixture
      status: open
---
## Goal

Generalize loom-ultralight's structural strength gate (`--strength`) from the
hardcoded id-canonicalization obligations to a **per-subject obligation list** over
an opaque function/predicate — the one new component E-0002 requires — without
changing any verdict on the existing subject.

## Context

The strength gate (`experiments/loom-ultralight/src/main.rs`, the
`strength` / `entails` / `assemble_strength` functions) currently hardcodes the
four canonicalize obligations (K/V/W with a two-rung width ladder, F). E-0002
re-validates the spec-weakening effect on two new subjects whose obligations
differ in shape, so the gate must accept an arbitrary per-subject obligation set.
This milestone is foundational: M-0004 and M-0005 author subjects against this
generalized interface and use it to confirm their obligations are isolable
single-input goals before pre-registering.

## Acceptance criteria

<!-- Candidate ACs; formalized via `aiwf add ac` at start-milestone. -->

- Given a per-subject obligation spec over a named opaque function/predicate, the
  gate emits per-obligation **exact/bound/free verdicts driven entirely by that
  spec**, with no canonicalize obligation hardcoded in the strength path —
  exercised by minimal fixtures covering the obligation **shapes the new subjects
  need**: at least an **exclusion goal** (`!P` over a ground tuple) and a **bounded
  quantifier over a finite datatype** (the FSM shapes), and a **unary opaque
  predicate over a single value** (the prosey shape), proving the interface is
  general beyond the canonicalize unary-function shape.
- Re-running the generalized gate on the cached canonicalize N=30 generations
  reproduces a **committed golden strength fixture** (the per-condition K/V/F
  entailment counts and the width exact/bound/free distribution), diffed
  mechanically; any changed verdict fails this AC.

## Constraints

- Behavior-preserving on the existing subject — the canonicalize strength verdicts
  must not change (the regression AC is the guard).
- Implementation-independent: obligations are probed against an opaque
  function/predicate (`function {:opaque} F`), never a concrete implementation.
- The killed / survived / inconclusive trichotomy and the probe-error guard carry
  over unchanged.

## Design notes

- Build on the existing `assemble_strength` / `entails` / `strength` functions;
  replace the hardcoded `STRENGTH_GOALS` constant and obligation classification
  with a per-subject obligation spec passed in.
- No arbitrary Dafny-spec parsing — the obligation list is authored per subject (an
  explicit spec object), not inferred from the candidate.

## Surfaces touched

- `experiments/loom-ultralight/src/main.rs` (the strength-gate functions).

## Out of scope

- Authoring the new subjects (M-0004, M-0005).
- Any change to the mutation kill-rate path or the API/run path.

## Dependencies

- None — foundational. Reuses the cached canonicalize N=30 run for the regression.

## References

- E-0002 epic spec; D-0001; `experiments/loom-ultralight/results/RESULTS.md`.

---

## Work log

## Decisions made during implementation

- (none)

## Validation

## Deferrals

- (none)

## Reviewer notes

- (none)

### AC-1 — Strength gate driven by per-subject obligation spec

### AC-2 — Canonicalize N=30 strength matches golden fixture

