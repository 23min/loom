---
id: M-0011
title: Run the reallocate sweep and record the terminal decision
status: draft
parent: E-0003
depends_on:
    - M-0010
tdd: required
acs:
    - id: AC-1
      title: The two-arm reallocate sweep is executed and the scored artifacts are recorded
      status: open
      tdd_phase: red
    - id: AC-2
      title: The prereg-ancestry guard passes for the run commit
      status: open
      tdd_phase: red
    - id: AC-3
      title: The terminal decision is recorded as a decision entity, re-derivable offline
      status: open
      tdd_phase: red
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
  scorers read).
- **Terminal milestone of E-0003** — its recorded decision entity discharges the epic.
