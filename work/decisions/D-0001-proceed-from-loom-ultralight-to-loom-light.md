---
id: D-0001
title: Proceed from loom-ultralight to loom-light?
status: proposed
relates_to:
    - E-0001
    - M-0002
---
## Question

Does the loom-ultralight result (M-0002) justify building loom-light — the Rust engine,
the claims surface, and the verifier integration?

This gate is **qualitative**, framed by the pre-registered outcomes in
`docs/loom-ultralight.md` §5, read across the multi-model sweep:

- **Gap positive and the mutation check catches it** → endogenous weakening is real and
  mechanically detectable → the differentiator holds → **proceed to loom-light**.
- **Gap ≈ 0** → the incentive does not induce weakening on this task → the differentiator
  is weaker than hoped → **reconsider** before building.
- **Gap positive but mutation misses it** (weak specs still kill the mutants) → the effect
  is real but the *check* is insufficient → **the engine needs more than mutation** before
  it is worth building.
- **Calibration failure** (gold does not kill 8/8, or too many inconclusives) → fix the
  spec / mutants / Z3 limits and **re-run** before deciding.

Cross-model lens: a gap that **persists in the strongest model (Opus)** is the most
consequential signal — it is not a weak-model artifact. A gap that appears only in Haiku
is weaker evidence for the general thesis.

## Decision

_Pending M-0002._ To be recorded once the results artifact exists, citing the per-model
table and the §5 branch the result lands in.

## Reasoning

_Pending M-0002._
