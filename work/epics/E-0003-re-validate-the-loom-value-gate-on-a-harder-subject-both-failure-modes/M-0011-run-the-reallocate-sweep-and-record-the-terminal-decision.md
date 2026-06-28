---
id: M-0011
title: Run the reallocate sweep and record the terminal decision
status: in_progress
parent: E-0003
depends_on:
    - M-0010
    - M-0012
    - M-0013
tdd: required
acs:
    - id: AC-1
      title: The two-arm reallocate sweep is executed and the scored artifacts are recorded
      status: met
      tdd_phase: done
    - id: AC-2
      title: The prereg-ancestry guard passes for the run commit
      status: met
      tdd_phase: done
    - id: AC-3
      title: The terminal decision is recorded as a decision entity, re-derivable offline
      status: met
      tdd_phase: done
---
## Goal

Execute the two-arm `reallocate` experiment across the pre-registered model sweep at
**N = 30** trials/arm, apply the frozen `reallocate_verdict` §6 map to the run artifacts,
and record the terminal go/no-go as a decision entity — discharging E-0003 with a fair
test of **both** pre-registered failure modes (under-specification *and* over-claiming).

## Context

`M-0008` hardened the harness; `M-0009` built and calibrated the `reallocate` instrument;
`M-0010` froze the two-dimension decision procedure and the pre-registration
(`prereg-reallocate.md`, committed at `bb1d220`). This is the **only** milestone that calls
the live Anthropic API. It wires the `M-0010` scorers (until now `#[allow(dead_code)]`) into
the production decide path and applies them to the run, then records the terminal decision
the epic's success criteria call for. Nothing downstream of the decision is in scope — a
PROCEED would justify building loom-light, a separate epic.

The first smoke on the `M-0012` sound gate surfaced `G-0007` (correct, complex disinterested
specs marked `unexecutable` by the instrument, not by the model) — a construct-validity flaw
that would have manufactured a confounded over-claim signal. The run was held while `M-0013`
certified the instrument (extraction terminator, helper capture, guarded-quantifier rewrite,
`<==>`-precedence normalization, enriched battery, freshness guard; error bound recorded in
`D-0004`). The recorded run below is on that certified gate.

## Acceptance criteria

### AC-1 — The two-arm reallocate sweep is executed and the scored artifacts are recorded

The harness runs both arms (disinterested, incentivized) for the pre-registered model sweep
(`opus-4.8`, `sonnet-4.6`, `haiku-4.5`) at the fixed **N = 30** trials/arm, producing the
raw generations and the scored artifacts: `results.json` (per model×arm `valid` /
`extracted` / `trials` census) and `strength.json` (per-obligation tell/easy entailment
rates + `inc`). Artifacts are written atomically (C3) and committed; scoring is deterministic
local Dafny (no API).

**Evidence (mechanical).** The committed run directory carries `results.json` +
`strength.json` for all three models × two arms; the census/strength shapes are validated on
read (B2), and a recorded check confirms the trial count is N = 30 per arm (no optional
stopping).

### AC-2 — The prereg-ancestry guard passes for the run commit

`loom-ultralight --check-prereg-ancestry <run-commit>` exits 0 — `prereg-reallocate.md`
(`bb1d220`) is a git-ancestor of the run commit, so "prereg precedes run" is verifiable from
git rather than asserted in prose (the `D-0001` / M-0002 integrity lesson).

**Evidence (mechanical).** The recorded guard output (exit 0) against the run commit; the
ancestry guard already pins `prereg-reallocate.md` in `PREREGS` (M-0010 / AC-4).

### AC-3 — The terminal decision is recorded as a decision entity, re-derivable offline

`reallocate_verdict` is wired into the production decide path (closing `M-0010`'s
procedure/run split — the scorers and their types lose `#[allow(dead_code)]`), applied to the
run's per-model census + strength to produce the per-model two-dimension verdicts and the
**primary-anchored** terminal decision, written to a self-contained `verdict.json` (the
over-claim rate, tell/easy gaps, and `inc` legible from the artifact — E3/G3). The terminal
go/no-go (PROCEED / NO-GO / RERUN-OR-EXPAND) is recorded as a **decision entity** via
`aiwfx-record-decision`, derived mechanically from `reallocate_verdict` with no residual
judgment after results are visible. The decision **re-derives offline** from the recorded
verdict(s) with no API call (G1).

**Evidence (mechanical).** Unit tests for the wired decide path (a fixture run directory →
`reallocate_verdict` → expected `verdict.json` and terminal decision); the recorded decision
entity; an offline re-derivation check (recorded verdict → same decision, no API). This AC
also lands the `M-0010` Reviewer-notes refinements: the B2 census-boundary validation on
`read_arm_counts` (`valid ≤ extracted ≤ trials`) and `null` `over_claim_rate` for a
zero-extracted arm.

## Constraints

- **One recorded run on `reallocate`; N = 30 fixed before the run; no optional stopping.**
  No peek-then-extend; a replacement subject is a deliberate recorded act under the identical
  prereg boundary — never an unbounded retry until one yields a reproduction (no
  subject-shopping).
- **The full pre-registered sweep is run.** The prereg committed to recording all three
  models as generalization evidence; the terminal decision only *anchors* on `opus-4.8`.
  Running fewer models would deviate from the frozen pre-registration.
- **The run commit descends from `bb1d220`** (the prereg), enforced by
  `--check-prereg-ancestry` — the pre-registration is a git-ancestor of the result.
- **API key from the gitignored `.env`** (never committed). Z3 nondeterminism is isolated and
  surfaced (G1); the killed / survived / inconclusive trichotomy never folds a timeout into a
  result.
- **loom's load-bearing principles hold**: B2 (validate the census/strength schemas on read),
  C3 (atomic artifact writes), E3/G3 (the `verdict.json` audit trail legible), G1 (same
  artifacts → same decision, offline).

## Design notes

- Run config: `LOOM_SUBJECT=reallocate`, `LOOM_MODELS=opus-4.8,sonnet-4.6,haiku-4.5`,
  `LOOM_TRIALS=30`.
- Wire `reallocate_verdict` into the `--decide` / verdict-emit path; the `M-0010` scorers and
  types (`overclaim_verdict`, `combine_dimensions`, `reallocate_verdict`, `ReallocateScore`,
  …) lose their `#[allow(dead_code)]` as they become reachable from `main`.
- **N = 30** mirrors E-0002's n30 and clears the `V = E = 10` floor with margin even under a
  ~50% incentivized validity collapse (≈15 valid). Budget: 3 × 2 × 30 = **180 API
  generations**; all scoring is local.
- The terminal decision is the primary model's (`opus-4.8`); `sonnet-4.6` / `haiku-4.5` are
  recorded as generalization evidence in `verdict.json` but do not gate.

## Out of scope

- **Building loom-light** — downstream of a PROCEED, a separate epic, not part of this
  re-validation.
- Re-running or re-scoring E-0002's subjects (`fsm`, `prosey`), or editing their frozen maps.
- Any change to the frozen `reallocate` instrument (`M-0009`) or the §6 procedure / thresholds
  / prereg (`M-0010`) beyond wiring them into the run path.

## Dependencies

- Depends on `M-0010` (the frozen two-dimension procedure + the committed, ancestry-guarded
  prereg), `M-0009` (the calibrated instrument), and `M-0008` (the self-contained census the
  scorers read). Resumed on the `M-0012` sound gate as certified by `M-0013` (the run was held
  for `G-0007` / `D-0004`).
- **Terminal milestone of E-0003** — its recorded decision entity discharges the epic.

## Work log

The wiring landed before the run; the run resumed once `M-0013` certified the instrument.
The §6 procedure / thresholds / prereg (`bb1d220`) were untouched throughout.

### AC-1 — Two-arm sweep executed, artifacts recorded

The certified-gate run produced `runs/reallocate/1782641702/` (gitignored; pinned by the run
commit and quoted in `D-0005`) with `results.json` + `strength.json` for all three models ×
two arms at **N = 30** — 30 saved generations per arm, 180 total. `results.json` records
`trials = 30` and `extracted = 30` for every row; the census is validated on read (B2,
`valid ≤ extracted ≤ trials`). · run commit `595d3dd`

### AC-2 — Prereg-ancestry guard passes for the run commit

`loom-ultralight --check-prereg-ancestry 595d3dd` → **exit 0**: all four pre-registrations
(`prereg-fsm`, `prereg-prosey`, `prereg-combination`, and `prereg-reallocate` at `bb1d220`)
are git-ancestors of the run commit. · wiring `2f00d7f`

### AC-3 — Terminal decision recorded, re-derivable offline

`reallocate_verdict` wired into the decide path (the `M-0010` scorers/types lose
`#[allow(dead_code)]`); a self-contained `verdict.json` was emitted (per-model
`over_claim_rate`, `tell_gap`, `easy_gap`, `inc`, thresholds, and the primary-anchored
`terminal`), and the terminal go/no-go was recorded as decision **`D-0005`** (accepted),
re-derivable offline from the recorded census + strength with no API call. · wiring `2f00d7f`,
decision `D-0005`

## Decisions made during implementation

- **`D-0005`** (accepted) — the terminal **NO-GO**. The frozen `reallocate_verdict` over the
  recorded N=30 × three-model × two-arm sweep returns terminal = NO-GO, anchored on the primary
  `opus-4.8`, on which **both** failure modes are not-reproduced (`tell_gap = 0.0`,
  `over_claim_gap = 0.0`, both arms 30/30 valid, 0 unexecutable, `inc = 0.0`). Mechanically
  derived; no residual judgment after results were visible; re-derivable offline.
- The construct-validity detour that preceded the run (`G-0007` → the hybrid gate `D-0003` and
  the certification `D-0004`) was carried by `M-0012` / `M-0013`, not folded into this
  milestone; `M-0011` resumed on the certified gate.

## Validation

- **Recorded run** `runs/reallocate/1782641702/` — 30 generations/arm × six arms (180), all
  `extracted = 30`. `results.json`: opus 30/30 valid both arms; sonnet 30/27; haiku 21/26 (the
  surfaced sweep-model residual). `verdict.json`: terminal **NO-GO**, `primary_model` opus-4.8.
- `loom-ultralight --check-prereg-ancestry 595d3dd` → exit 0 (all four preregs precede the run).
- `cargo test`: **69 passed; 0 failed; 4 ignored**. `clippy -D warnings` + `fmt --check` clean.
- The wired decide path and the offline re-derivation are pinned by
  `reallocate_verdict_matches_preregistered_map`, `reallocate_terminal_anchors_on_primary_model`,
  `emit_reallocate_verdict_writes_multimodel_verdict_json`,
  `combine_dimensions_matches_preregistered_truth_table`, and
  `verdict_inputs_json_is_self_contained` — deterministic (G1), no API.

## Deferrals

None. The over-claim instrument residual is accepted and surfaced per `D-0004`, not deferred;
the evidence-only sweep-model residual (haiku-4.5 disinterested `unexecutable`) is surfaced
per-arm in `results.json` (`unexecutable` / `inconclusive`), never folded into a result.

## Reviewer notes

- **The NO-GO is sound, not an instrument artifact** — this is what `M-0013` / `D-0004` bought.
  On the primary `opus-4.8` the over-claim instrument had a **0% unexecutable residual in this
  run** (30/30 valid, both arms), so `over_claim_rate = 0.0` is a genuine "the reference impl
  satisfies these specs," not the gate failing to decide. Before the certification, the opus
  disinterested arm carried a spurious ~20% over-claim from the `<==>`-precedence artifact
  (`G-0007`), which would have confounded the comparison.
- **Sweep models are evidence-only**, pre-registered as generalization, not gating; both return
  NO-GO. `sonnet-4.6` shows an incentivized `easy_rate` dip (`tell_gap 0.29`) but it is *not
  localized* on the load-bearing tell and over-claiming is not-reproduced; `haiku-4.5` carries
  the genuine-undecidable residual `D-0004` bounds (8 disinterested `unexecutable`) — a visible
  per-arm-census signal, surfaced not silently folded, and NO-GO regardless.
- **Integrity.** One recorded run; N = 30 fixed before it (no optional stopping); no
  subject-shopping; `bb1d220` is a git-ancestor of the run commit (`--check-prereg-ancestry`
  exit 0). The §6 procedure / thresholds / prereg were untouched after `bb1d220`.
