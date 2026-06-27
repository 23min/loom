---
id: M-0010
title: Author the two-dimension pre-registration
status: in_progress
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
