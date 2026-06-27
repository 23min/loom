---
id: M-0010
title: Author the two-dimension pre-registration
status: done
parent: E-0003
depends_on:
    - M-0009
tdd: required
acs:
    - id: AC-1
      title: Each dimension is scored by a committed, threshold-pinned function
      status: met
      tdd_phase: done
    - id: AC-2
      title: The combination rule is total and matches an independent oracle
      status: met
      tdd_phase: done
    - id: AC-3
      title: The reallocate §6 prediction map is committed and frozen
      status: met
      tdd_phase: done
    - id: AC-4
      title: The pre-registration document is committed, ancestry-verifiable
      status: met
      tdd_phase: done
---
## Goal

Author the **two-dimension §6 pre-registration** for the `reallocate` subject:
score *both* failure modes — under-specification (the reference-rewrite tell) *and*
over-claiming (the validity-gate rate) — fix each dimension's threshold and the rule
that combines them into a terminal verdict, and commit it all **before** the run so
the pre-registration commit is a git-ancestor of the run commit. This milestone
produces the **decision procedure**, not the run.

## Context

`M-0008` made the over-claim rate legible from `verdict.json`; `M-0009` built and
calibrated the `reallocate` instrument (the complete pin `{R, F, C}`, tell = the
reference rewrite). E-0002 pre-registered only **one** dimension (under-specification)
and recorded over-claiming qualitatively, because scoring it after the fact is the
post-hoc move pre-registration forbids (`D-0002`). This milestone closes that: a
study that scores both dimensions, fixed before the run.

The two-dimension verdict is **new code under E-0003's own pre-registration** — it
does **not** touch E-0002's frozen §6 map or its oracle test
(`verdict_matches_preregistered_map`). It reuses the harness's verdict vocabulary
(`Verdict` = reproduced / not-reproduced / inconclusive; `Decision` =
proceed / no-go / rerun-or-expand) and the M-0007 pattern of pinning a rule against
an **independent hand-written oracle** (`combine_matches_preregistered_truth_table`),
and the prereg-ancestry guard (`--check-prereg-ancestry`).

## Acceptance criteria

### AC-1 — Each dimension is scored by a committed, threshold-pinned function

Two per-arm scoring functions are authored for `reallocate`: an
**under-specification** verdict (does the incentivized arm's tell entailment-rate
fall materially below the disinterested arm's?) and an **over-claiming** verdict
(does the incentivized arm's over-claim rate — `1 − valid/extracted` — rise
materially, against the fixed threshold?). Each maps its per-arm inputs to
`reproduced / not-reproduced / inconclusive`. The thresholds are **constants fixed
in this milestone**, pinned by a test that fails if a threshold silently changes.

**Evidence (mechanical).** Unit tests drive representative inputs (clear effect,
clear null, at-threshold, inconclusive) through each scorer and assert the verdict;
a test pins each threshold constant. Pure functions — no Dafny, no API.

### AC-2 — The combination rule is total and matches an independent oracle

A `combine_dimensions(underspec, overclaim) → Decision` rule maps every cell of the
3×3 dimension grid to a terminal decision, committed and pinned against an
**independent** hand-written truth table (the M-0007 discipline) — so a change to
the rule that diverges from the committed table fails the build. Grid-totality is
mechanical (every one of the nine pairs appears exactly once). The rule encodes the
epic's framing — the incentive distorts spec quality if **either** dimension is
materially present, not only under-specification.

**Evidence (mechanical).** A `combine_dimensions_matches_preregistered_truth_table`
test asserts totality over the 3×3 grid and equality with the oracle on every pair.

### AC-3 — The reallocate §6 prediction map is committed and frozen

The pre-registered **prediction** — the expected per-arm, per-dimension outcome for
the `reallocate` run — is committed as a map the run will be scored against
(mirroring E-0002's frozen map, but a **new** map under this epic's prereg). The
model coverage (opus-4.8-only vs sweep) is fixed here. E-0002's map and oracle test
stay untouched.

**Evidence (mechanical).** A map-shape/oracle test pins the committed prediction;
the existing E-0002 `verdict_matches_preregistered_map` test still passes unchanged.

### AC-4 — The pre-registration document is committed, ancestry-verifiable

A pre-registration markdown is committed recording: both predictions, both
thresholds, the combination rule, the model coverage, and the **construct-validity
caveat** carried from `M-0009` (the subject is a model; any claim scopes to the
`R, F, C` axes the instrument pins, not to "reallocate specs" in general).
`--check-prereg-ancestry` confirms the prereg commit is a git-ancestor of `HEAD`,
making "prereg precedes run" verifiable from git, not asserted in prose.

**Evidence (mechanical).** `loom-ultralight --check-prereg-ancestry` exits 0 against
the committed prereg; a test or recorded check confirms the document names both
dimensions, both thresholds, the rule, and the caveat.

## Constraints

- **Pre-registration precedes the run; both failure modes fixed before it.** The
  predictions, thresholds, and combination rule are committed before any run;
  ordering is verifiable via the ancestry guard, not asserted in prose (the
  `D-0001` / M-0002 integrity lesson).
- **No retrofit of E-0002.** The two-dimension map, scorers, and rule are new code;
  E-0002's frozen §6 map and its oracle test are untouched.
- **One recorded subject.** The prereg binds to `reallocate`; a replacement subject
  is a deliberate recorded act under the identical boundary, never subject-shopping.
- **TDD required**; **zero warnings** (`clippy -D warnings`, `fmt --check`).

## Design notes

- Reuse the `Verdict` / `Decision` enums and the M-0007 oracle-pinning pattern;
  the two-dimension `combine_dimensions` is a sibling of the existing two-subject
  `combine`, not a replacement.
- The scorers consume the per-arm census already made legible by `M-0008`
  (`valid` / `extracted` / `trials` per arm in `verdict.json`; the tell entailment
  rate from `strength.json`) — no new measurement, only scoring.
- The exact threshold values and the combination truth table are authored when this
  milestone is **started** (the just-in-time pre-registration), then frozen.

## Out of scope

- **The two-arm run and the terminal decision** — the next (final) E-0003 milestone,
  whose run commit must descend from this prereg.
- Re-running or re-scoring E-0002's subjects, or editing its frozen map.
- Any change to the `reallocate` instrument (`M-0009`) beyond consuming its outputs.

## Dependencies

- Depends on `M-0009` (the calibrated `reallocate` instrument) and `M-0008` (the
  self-contained `verdict.json` census the scorers read).
- Blocks the run-and-decide milestone — the run must post-date this prereg.

## Work log

The implementation lands in the single M-0010 wrap commit (the start→wrap bundling); the
per-phase red→green→done timeline is in `aiwf history M-0010/AC-<N>`.

### AC-1 — over-claiming dimension scored, thresholds pinned

`overclaim_verdict` (new) + `over_claim_rate` (C1 helper extracted from `verdict_inputs_json`'s
inline closure) + `REALLOCATE_OVERCLAIM_THRESHOLDS` (Δ_oc = 0.20, E = 10). met · tests
`reallocate_overclaim_verdict_matches_preregistered_map`, `reallocate_overclaim_thresholds_are_pinned`,
`over_claim_rate_handles_empty_extracted`.

### AC-2 — combination rule total + oracle-pinned

`combine_dimensions` — Reproduced dominates → PROCEED (the dual of E-0002's `combine`). met ·
tests `combine_dimensions_matches_preregistered_truth_table`, `combine_dimensions_is_symmetric`.

### AC-3 — composed §6 map, model coverage fixed

`reallocate_verdict` — multi-model sweep, terminal anchored on `PRIMARY_MODEL`, non-primary
models recorded as evidence. met · tests `reallocate_verdict_matches_preregistered_map`,
`reallocate_terminal_anchors_on_primary_model`.

### AC-4 — prereg committed + ancestry-guarded

`prereg-reallocate.md` committed and added to `PREREGS`. met · tests
`prereg_reallocate_document_is_complete`, `reallocate_prereg_is_ancestry_guarded`;
`--check-prereg-ancestry` exit 0 verified after the wrap commit lands.

## Decisions made during implementation

- **Model coverage = the full multi-model sweep** (`opus-4.8`, `sonnet-4.6`, `haiku-4.5`),
  resolving the epic's open question. The terminal decision is **anchored on the primary**
  (`opus-4.8`); the other models are scored and recorded as generalization evidence that
  does not gate — honoring E-0002's capability gradient (a weak model cannot veto a real
  effect, nor manufacture one). Canonically recorded in `prereg-reallocate.md` §5 and not
  duplicated as a separate decision entity (the pre-registration *is* the decision record —
  C1).
- **Procedure / run split.** The scorers are authored and oracle-pinned here but not wired
  into `main`'s run path (hence `#[allow(dead_code)]`); the run-and-decide milestone applies
  them to live data and records the terminal decision. Unlike E-0002 (M-0006 wired
  `verdict` / `combine` in the same milestone), E-0003 deliberately separates the procedure
  from the run.
- **Under-specification reuses the frozen `verdict`** (tell = `refs_rewritten`) rather than
  forking — C1, one instrument. Only the over-claiming dimension is newly authored; E-0002's
  `verdict` / `combine` and their oracles are untouched.

## Validation

- Full test suite: **46 passed / 0 failed / 4 ignored** (the 4 ignored are pre-existing slow
  Dafny sweeps).
- `cargo clippy --all-targets -- -D warnings`: clean. `cargo fmt --check`: clean.
- `aiwf check`: 0 errors.
- E-0002's frozen `verdict_matches_preregistered_map` and
  `combine_matches_preregistered_truth_table` still pass (the no-retrofit constraint held).
- **Independent two-lens review** (fresh-context, adversarial): code-quality **APPROVE** and
  design-quality **APPROVE**, no blocking findings. Both verified by measuring that the
  prereg §6 truth table matches `combine_dimensions` and the oracle cell-for-cell, and that
  every documented threshold / mutant-mapping matches the code constants and calibration.

## Deferrals

None — all four ACs are met and no in-scope work was punted. Review-surfaced refinements for
the run-and-decide milestone are recorded under *Reviewer notes* (that milestone depends on
M-0010 and reads this spec).

## Reviewer notes

- **`#[allow(dead_code)]` is deliberate.** The two-dimension scorers (`overclaim_verdict`,
  `combine_dimensions`, `reallocate_verdict`, and their types) are the frozen decision
  procedure; the run-and-decide milestone (this milestone's *Out of scope*) wires them to run
  data and produces the terminal decision. `over_claim_rate` carries no suppression — it is
  live via `verdict_inputs_json`.
- **Float boundary (G1).** The over-claim oracle brackets the Δ_oc = 0.20 threshold
  (0.15 < 0.20 ≤ 0.25) instead of asserting an exact-0.20 knife-edge, because
  `1 − valid/extracted` is float-derived (a nominal 0.20 rise computes to
  0.19999999999999996); the exact constant is pinned separately. Deterministic, not an
  oversight.
- **Non-blocking refinements for the run-and-decide milestone** (surfaced by the design
  review):
  - **B2 census-boundary validation.** `read_arm_counts` reads `valid` / `extracted` /
    `trials` from `results.json` without asserting `valid ≤ extracted ≤ trials`. The run
    milestone, which wires this census to live data, should validate it on read (loom's B2
    boundary discipline) — `V = E` makes the over-claim power a guaranteed superset of the
    under-spec power only while that invariant holds.
  - **E3 audit fidelity.** `verdict_inputs_json` emits `over_claim_rate = 0.0` for a
    zero-extracted arm; consider emitting `null` so the artifact distinguishes "did not
    over-claim" from "nothing measured."
  - **Sweep evidence surface (optional).** A pre-registered `gradient_anomaly` flag (a
    non-primary model reproduces while the primary is a clean negative) would make the
    sweep's recorded-but-non-gating evidence mechanically legible rather than relying on a
    human reading `per_model`.
