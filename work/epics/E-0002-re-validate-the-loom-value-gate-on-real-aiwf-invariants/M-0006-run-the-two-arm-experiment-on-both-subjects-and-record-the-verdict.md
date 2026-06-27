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
      status: met
    - id: AC-2
      title: Prereg and combination-rule SHAs verified as git ancestors of the run commit
      status: met
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

The milestone subject-parameterized the loom-ultralight harness (a production `Subject`
registry; `LOOM_SUBJECT` selects canonicalize / fsm / prosey), added a mechanical
`verdict()` (the §6 map) + `combine()` wiring, an `--decide` step, and the
`--check-prereg-ancestry` guard — then ran the paid two-arm experiment (opus-4.8, N=30)
on both subjects. The M-0002 canonicalize subject is preserved byte-for-byte (the M-0003
golden strength regression passes unchanged).

### AC-1 — both arms run on both subjects, scored by kill-rate and structural strength
Run recorded under `experiments/loom-ultralight/results/E-0002/{fsm,prosey}/`
(`results.json` kill-rate + per-arm validity, `strength.json` structural, `verdict.json`
the §6 verdict). Inconclusives (Z3 timeouts) are their own category, never folded in.
Production scorer pinned by `production_scorer_calibrates_every_subject`.

### AC-2 — prereg + combination SHAs verified as git ancestors of the run commit
`--check-prereg-ancestry` passes for `prereg-fsm.md` (`22cd65e`), `prereg-prosey.md`
(`91faa23`), and `prereg-combination.md` (`3e58ca1`) against the run commit; hermetic
test `ancestry_guard_identifies_prereg_precedence`.

### AC-3 — verdict() and combine() yield a recorded go/no-go Decision
`verdict()` (oracle test `verdict_matches_preregistered_map`) → both subjects
not-reproduced; `combine()` (oracle test `combine_matches_preregistered_truth_table`) →
**NO-GO**, recorded as **D-0002** (accepted), discharging D-0001's re-validation duty.

## Decisions made during implementation

- **Subject selection via `LOOM_SUBJECT` (not a CLI positional).** Keeps the M-0002 CLI
  and golden-reproduce commands byte-identical (default canonicalize), like the existing
  `LOOM_TRIALS` seam.
- **FSM `opaque_decls` duplication bug — found by the smoke run, fixed.** `FSM_SUBJECT`
  originally re-declared `Kind`/`Status` that the fsm.dfy preamble also defines, so every
  fsm strength probe was a resolution error in production (the unit tests had masked it by
  probing with an empty preamble). Fixed to declare only the opaque `IsLegal` (datatypes
  from the preamble, matching canonicalize); the fsm/prosey probe tests now run against
  the real preamble, so the production path is under test. This is exactly why the smoke
  run ran first.
- **Mechanical `verdict()` + oracle test (not prose).** The §6 map is a total function
  pinned against an independent boundary-covering oracle — AC-3's verdict is not a hand
  computation after the run, the same integrity bar `combine()` has.
- **opus-only run (`LOOM_MODELS=opus-4.8`).** The verdict is pre-registered on the primary
  model; opus-only is the faithful, cheapest path. Added a default-all model filter.
- **Result: D-0002 NO-GO.** The over-claim signal (fsm incentivized 50% invalid) is a real
  but un-pre-registered failure mode — recorded qualitatively, not scored (relabeling it a
  reproduction post-hoc is the move pre-registration forbids).

## Validation

- `cargo test --release` → **25 passed, 0 failed, 4 ignored**; the ignored slow guards run
  green on demand: `golden_canonicalize_n30_strength_is_reproduced` (M-0003 backward-compat,
  byte-identical), `production_scorer_calibrates_every_subject`, the two isolation sweeps.
- `cargo build --release` → no warnings; `cargo clippy --release -- -D warnings` → clean;
  `cargo fmt --check` → clean (the milestone also absorbed the 3 long-standing pre-existing
  fmt drifts, having rewritten 6 of the 9 drifted spots).
- Dry calibration: canonicalize 20/20, fsm 11/11, prosey 6/6. `--check-prereg-ancestry` →
  all 3 PASS. The recorded NO-GO reproduces: `--decide results/E-0002/fsm results/E-0002/prosey`.
- Paid run: opus-4.8, N=30/arm, 0 API errors, 0 extraction failures.

## Deferrals

- **G-0004** — unify the harness model-filtering across the kill-rate and strength outputs;
  make `verdict.json` self-contained (carry the per-arm validity rate).
- **G-0005** — gate the strength entailment population to valid specs (close the dormant
  ex-falso confound) and make `probe_spec`'s timeout routing unit-testable. Both feed the
  E-0003 successor study.

## Reviewer notes

Independent two-lens review before wrap, over the full M-0006 change-set:

- **Code-quality (`reviewer`) → APPROVE.** Every load-bearing claim verified by measuring:
  the M-0003 golden strength regression passes byte-for-byte (backward-compat proven), the
  fsm fix + calibration, the verdict / combine / ancestry oracles, and all gates. No
  blocking defect; the NO-GO reproduces from committed artifacts.
- **Design (`wf-rethink`) → SOUND.** No integrity gap lets post-hoc latitude into the
  NO-GO; the over-claim-recorded-not-scored choice was endorsed as "the model answer for a
  falsification"; the float-boundary comparison was confirmed deterministic (not a G1 risk).

Findings applied in place: corrected the `active_models()` doc comment (it wrongly claimed
scoring iterates all models — `results.json` is opus-only while `strength.json` carries all
three) and the stale module header. Findings deferred to **G-0004** / **G-0005**. The
ex-falso confound was dormant this run (the fsm over-claims surfaced as probe errors, so the
strength population equalled the valid population) and is a pre-registration note for E-0003.

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

**Evidence:** committed artifacts under `experiments/loom-ultralight/results/E-0002/`
for both subjects — `results.json` carries the per-arm validity rate (surfacing the FSM
incentivized over-claim, 15/30 valid vs 29/30 disinterested), `strength.json` the
per-obligation entailment with inconclusives as their own column, `verdict.json` the
inputs + §6 verdict. Production scorer pinned by `production_scorer_calibrates_every_subject`;
dry calibration green before the paid run.

### AC-2 — Prereg and combination-rule SHAs verified as git ancestors of the run commit

Each subject's recorded run result **names its pre-registration commit SHA**
(`prereg-fsm.md` / `prereg-prosey.md`), and a mechanical check verifies that SHA is a
**git ancestor** of the run commit; the **M-0007 combination-rule prereg**
(`prereg-combination.md`) SHA is **likewise verified** as an ancestor. This is the
pre-registration-precedes-run guard — covering both the per-subject and the
cross-subject pre-registrations — so no result can have been read before its prediction
was committed (the M-0002 integrity lesson, enforced from git, not asserted in prose).

**Evidence:** `loom-ultralight --check-prereg-ancestry` resolves each prereg's commit
(`22cd65e` / `91faa23` / `3e58ca1`) and verifies it is a `git merge-base --is-ancestor`
of the run commit — all three PASS; hermetic regression `ancestry_guard_identifies_prereg_precedence`.

### AC-3 — verdict() and combine() yield a recorded go/no-go Decision

A mechanical **`verdict()`** maps each subject's recorded measures to its
pre-registered edge — **reproduced / not-reproduced / inconclusive** — as a **total
function** of the observation, matching that subject's §6 verdict map exactly
(thresholds V, Δ⁺, Δ⁰, I), pinned by an oracle test the way `combine()` is (not a hand
computation after the run). The two verdicts feed **`combine()`** (M-0007), yielding a
single epic-level **PROCEED / NO-GO / RERUN-OR-EXPAND**, recorded as a **`Decision`**
entity via `aiwfx-record-decision`, relating to E-0002 and discharging D-0001.

**Evidence:** `verdict()` pinned by `verdict_matches_preregistered_map` and `combine()`
by `combine_matches_preregistered_truth_table` (both independent oracles); applied to the
run, both subjects → not-reproduced, `combine` → **NO-GO**, recorded as **D-0002**
(accepted, relates to E-0002 / M-0006 / D-0001). `--decide results/E-0002/fsm
results/E-0002/prosey` reproduces the decision from the committed artifacts.

