---
id: M-0008
title: Harden the loom-ultralight harness
status: in_progress
parent: E-0003
tdd: required
acs:
    - id: AC-1
      title: Strength entailment population is the valid population
      status: open
      tdd_phase: red
    - id: AC-2
      title: probe_spec outcome routing is unit-testable without Dafny
      status: open
      tdd_phase: red
    - id: AC-3
      title: Kill-rate and strength outputs agree on model row membership
      status: open
      tdd_phase: red
    - id: AC-4
      title: verdict.json is self-contained with per-arm valid, extracted, trials
      status: open
      tdd_phase: red
    - id: AC-5
      title: Canonicalize golden re-baselined with opus-4.8 verdict unchanged
      status: open
      tdd_phase: red
---

## Goal

Close `G-0004` and `G-0005` so the loom-ultralight harness is correct and legible
before E-0003's two-dimension study runs on it: gate the strength measure to the
valid population, make the probe routing testable without Dafny, unify
model-filtering across the two output files, and make the over-claim rate legible
from `verdict.json` alone — re-baselining the canonicalize golden as the one
deliberate, verdict-invariant consequence.

## Context

E-0002 surfaced two harness gaps and one latent confound the next study makes
live. The strength gate (`probe_spec`, `experiments/loom-ultralight/src/main.rs`)
measures every spec that *resolves*, not every spec that is *valid* — so a
resolving-but-contradictory over-claim entails every obligation ex falso and
inflates the rates toward the null. This was dormant in E-0002 (the FSM
over-claims surfaced as probe errors), but it is **not** dormant on the
canonicalize corpus: `valid` (kill-gate) and `specs` (strength resolve-gate)
already diverge in two rows (`sonnet · disinterested` 24 vs 28; `haiku ·
incentivized` 25 vs 30). E-0003's subject is chosen for over-claiming, so the
confound goes from dormant to load-bearing. This milestone closes it, plus the
model-filtering and self-containment gaps, on the existing harness — the
two-dimension *scored* verdict is a later milestone, on this hardened base.

## Acceptance criteria

### AC-1 — Strength entailment population is the valid population

The validity gate (today the first half of `score_spec` — "reference impl
verifies against the spec", `main.rs:237-251`) is extracted into a single
predicate called by **both** `score_spec` and `probe_spec`. `probe_spec` excludes
any spec the reference implementation fails, so a resolving-but-invalid over-claim
never enters the strength tally — the entailment-rate denominator *is* the valid
(over-claim-gate-passing) population by construction (`C1` single source of
truth).

**Evidence (mechanical).** A unit test feeds a spec the reference impl fails (an
over-claim that still type-checks): it is counted invalid and excluded, leaving
`specs`/`counts` unchanged; and a check that the shared predicate and
`score_spec`'s validity verdict agree for the same `(spec, subject)`. The test
fails if `probe_spec` reverts to the resolve-only guard.

### AC-2 — probe_spec outcome routing is unit-testable without Dafny

`probe_spec` takes an injectable outcome closure (mirroring `classify_ladder`'s
`probe` parameter, `main.rs:1127`), so the §5 trichotomy — `Verified` → `counts`
+ `definite`; `Failed` → `definite` only; `Timeout` → `obligation_timeouts`,
dropped from the denominator — is pinned deterministically with no `dafny verify`
call and no wall-clock dependency.

**Evidence (mechanical).** A unit test drives all three outcomes through the
injected closure and asserts every tally field (`counts`, `definite`,
`obligation_probes`, `obligation_timeouts`) and that a `Timeout` is dropped from
`mean_entailment_rate`'s denominator. Closes the branch-coverage gap on a
load-bearing measure (`D1`).

### AC-3 — Kill-rate and strength outputs agree on model row membership

The active-model list is resolved once in `main` and threaded into the strength
path (`compute_strength` / `strength_rows_json` / `print_strength_table`) and the
kill-rate gap table (`print_results`), removing the `LOOM_MODELS` read buried in
`score_trials`. A single-model run produces `results.json` and `strength.json`
with **identical** model-row membership — no zero-rows for filtered-out models.
The verdict's primary model is resolved from **one** source, not the three
`"opus-4.8"` string literals (`build_observation`, `emit_verdict`, the
`read_valid_counts` call).

**Evidence (mechanical).** A test asserts that under a model filter the strength
serializer emits only the active model's rows, matching `results.json`'s
membership; and that with no filter (the golden path, all of `MODELS`) the output
is unchanged.

### AC-4 — verdict.json is self-contained with per-arm valid, extracted, trials

`emit_verdict` writes, per arm, `valid`, `extracted` (specs that parsed), and
`trials`, so the over-claim rate (`1 − valid / extracted`) is computable from the
`verdict.json` artifact **alone**, without cross-referencing `results.json`. The
kill-rate path records `extracted` per row so the rate's denominator is the
parsed-spec count, not raw trials (clean under extraction noise). `B2` boundary
schema, extended additively.

**Evidence (mechanical).** A test asserts `emit_verdict`'s JSON carries
`valid`/`extracted`/`trials` per arm and that `results.json` rows carry
`extracted`. (E-0002's committed `verdict.json` predate this format and stay as
historical records — not recomputed.)

### AC-5 — Canonicalize golden re-baselined with opus-4.8 verdict unchanged

After AC-1's gating, the canonicalize strength over the corpus changes exactly the
two non-primary rows above (`sonnet · disinterested` 28 → 24; `haiku ·
incentivized` 30 → 25); `results/strength-n30.json` is re-baselined to match. The
**opus-4.8 rows and the canonicalize §6 verdict are verified unchanged** — run,
not asserted — so M-0002's recorded finding stands. The re-baseline is its own
commit, recording the population correction.

**Evidence (mechanical).** `golden_canonicalize_n30_strength_is_reproduced` passes
against the re-baselined golden; a recorded check confirms the opus-4.8 inputs
(28/28, 30/30 → tell/easy rates) and the `reproduced` verdict are byte-identical
to the pre-gating values.

## Constraints

- **No frozen-result regression beyond the one recorded re-baseline.**
  `verdict_matches_preregistered_map` and `combine_matches_preregistered_truth_table`
  stay green; E-0002's §6 map is untouched (the two-dimension scored verdict is a
  later milestone). The canonicalize golden re-baseline (AC-5) is the *only*
  deliberate change to a committed result, and it is verdict-invariant.
- **TDD required** — every AC red → green → refactor, with the branch-coverage
  audit on the diff before any AC flips to `met`.
- **Zero warnings** — `cargo clippy -- -D warnings` clean, `cargo fmt --check`
  clean (the cross-language floor).

## Design notes

- AC-1 **extracts** the existing validity gate; it does not introduce a second
  definition of "valid" (`C1`). The two paths converge on one predicate.
- AC-2 makes the existing trichotomy injectable while the real-Dafny path stays
  the default — behaviour-preserving refactor plus a new deterministic test.
- AC-3 threads the existing `active_models()` (`main.rs:67`) rather than reading
  the env deep inside `score_trials`; the default (env unset) remains all of
  `MODELS`, so the golden corpus is unaffected.
- `verdict.json` / `results.json` are the `B2` boundary schemas; AC-4 extends them
  additively (new fields, no field removed or repurposed).

## Out of scope

- The two-dimension **scored** verdict (under-specification *and* over-claiming as
  joint §6 dimensions) — a later E-0003 milestone, under its own pre-registration.
  This milestone makes the over-claim rate legible (the *data*); it does not score
  it.
- Any new subject, pre-registration, or experiment run.
- Editing E-0002's frozen §6 verdict map or re-running its subjects.

## Dependencies

- Closes `G-0004` and `G-0005`.
- Foundational — first milestone in E-0003; no milestone dependencies.
- Builds on the E-0002 harness (the `LOOM_SUBJECT` registry and the structural
  strength gate).
