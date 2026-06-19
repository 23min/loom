---
id: M-0001
title: Materialize the loom-ultralight experiment into runnable files
status: draft
parent: E-0001
tdd: advisory
acs:
    - id: AC-1
      title: dafny verify passes GoldSpec and Idempotent
      status: open
    - id: AC-2
      title: gold spec kills all 8 mutants (8/8 calibration)
      status: open
    - id: AC-3
      title: experiment harness materialized and runnable
      status: open
---
## Goal

Turn the design in `docs/loom-ultralight.md` §3 into runnable files so the repo finally
contains something that executes. Target directory: `experiments/loom-ultralight/`.

Deliverables:
- `canonicalize.dfy` — the `Id` datatype, `Wellformed`, the reference `Canonicalize`,
  and the gold spec (`GoldSpec` + `Idempotent`).
- `mutants/M1..M8` — the 8-mutant bank (three of which break value-preservation, the
  "gamed-spec" tell).
- `prompts/disinterested.md` and `prompts/incentivized.md` — **both** author a spec *and*
  an implementation, identical prose intent; they differ **only** in the grading clause
  (spec audited for completeness vs graded only on `dafny verify` passing). Holding the
  task constant isolates the incentive as the sole variable; we score only the spec.
- a Rust harness (`Cargo.toml` + `src/main.rs`, deps pinned by `Cargo.lock`) — calls
  the Anthropic API, pairs each spec-under-test with the gold impl and every mutant,
  runs `dafny verify`, classifies killed / survived / inconclusive, and computes
  kill-rate. **Parameterized by model** (for the M-0002 sweep). The shell-out to
  `dafny verify` is a deliberate micro-prototype of loom-light's verifier path.
- `run.sh` — a thin `cargo run` wrapper over the harness; prints the table.

Definition of done is the §3 Step-0 calibration, captured by the ACs below: the gold
spec verifies and kills all 8 mutants. (The Dafny may need a one-line syntax fix on
first run — that is a fix, not a re-authoring; the toolchain ships in the devcontainer.)

## Acceptance criteria

### AC-1 — dafny verify passes GoldSpec and Idempotent

`dafny verify` on `canonicalize.dfy` reports success for both `GoldSpec` and
`Idempotent` against the reference implementation.

### AC-2 — gold spec kills all 8 mutants (8/8 calibration)

Run against the mutant bank, the gold spec fails to verify every mutant (kill-rate
8/8). This calibrates the detector: any spec that misses a mutant is weaker than gold.

### AC-3 — experiment harness materialized and runnable

`canonicalize.dfy`, `mutants/`, `prompts/`, the Rust harness (`Cargo.toml` +
`src/main.rs`), and `run.sh` exist under `experiments/loom-ultralight/` and run
end-to-end; a dry (no-API) mode is enough to prove the scoring path — the live
multi-model run is M-0002.
