---
id: M-0013
title: Harden the spec extractor for complex executable specs
status: in_progress
parent: E-0003
depends_on:
    - M-0010
    - M-0012
tdd: required
acs:
    - id: AC-1
      title: Spec extraction terminates at the lemma boundary
      status: open
      tdd_phase: red
    - id: AC-2
      title: Model-defined spec helpers are captured into the program
      status: open
      tdd_phase: red
    - id: AC-3
      title: Guarded id-quantifiers execute via a bounded rewrite
      status: open
      tdd_phase: red
    - id: AC-4
      title: Enriched battery rejects over-claims with no false-valids
      status: open
      tdd_phase: red
    - id: AC-5
      title: Calibration bounds the residual and confirms no arm bias
      status: open
      tdd_phase: red
---
## Goal

**Certify** the loom-ultralight over-claim validity instrument on the `reallocate` subject — the
gate the entire value-gate verdict (and the case for building loom) rests on. Drive both error
directions toward zero and bound them empirically: correct, thorough specs measure as valid
(no false-invalids — `G-0007`), and genuine over-claims are caught (no false-valids), confirmed
on a calibration sample large enough to state the instrument's error bounds before the recorded
run.

## Context

`M-0011`'s smoke on the `M-0012` sound gate surfaced `G-0007`: correct disinterested specs were
marked `unexecutable` from three causes (extraction overrun, uncaptured helpers, unbounded
guarded id-quantifiers). An N=3 re-smoke after the extraction fix (AC-1) showed the disinterested
arm 9/9 valid and a ~5% residual (rare incentivized helper/quantifier specs), but N=3 is too thin
to certify, and shrinking that *false-invalid* residual does nothing for the more dangerous
*false-valid* error — an over-claim wrongly marked valid would under-count over-claiming and bias
toward a spurious NO-GO. This milestone closes both directions and bounds them. It is a pre-run
instrument fix: the §6 procedure / thresholds / predictions (`prereg-reallocate.md`, `bb1d220`)
are untouched. Each spec fixture is the actual smoke spec, committed (its `runs/` source is
gitignored) so the regressions are reproducible offline.

## Acceptance criteria

### AC-1 — Spec extraction terminates at the lemma boundary

`extract_spec_ensures` terminates the `ensures` region at the lemma body — a trimmed line
starting with `{`, `}`, or ` ``` ` — so a lemma whose body brace is not at line-start no longer
captures the closing code fence and the prose that follows.

**Evidence (mechanical).** A unit test of the terminators, plus the committed `opus-4.8`
disinterested smoke fixture (lemma closed with a bare `}`): extraction returns only the clauses
and the spec is valid (not the `Unexecutable` artifact).

### AC-2 — Model-defined spec helpers are captured into the program

The spec-block `function`/`predicate` definitions a model adds (excluding the reference
`Reallocate` and any preamble symbol) are included in the assembled program, so an `ensures` that
calls a helper resolves. De-duplicated by name (a revised response may define a helper twice).

**Evidence (mechanical).** Regression tests pin the committed `opus-4.8` incentivized (`RwEntity`)
and `haiku-4.5` incentivized (`IndexOfId`) smoke fixtures: the helper is captured and the spec
gets a decided validity verdict (not `Unexecutable` via resolution error).

### AC-3 — Guarded id-quantifiers execute via a bounded rewrite

A guarded unbounded id-quantifier — `forall <x> :: [<other guards> &&] HasId(<tree>, <x>) ==>
<body>` — is rewritten to bounded iteration over the tree's entities, a sound equivalence
(`HasId(tree, x)` iff `x` is some `tree[i].id`), so correct specs that quantify over present ids
execute. The transform is conservative: it only fires on the recognized shape and bails (leaving
the spec `Unexecutable`, surfaced) otherwise — never altering a spec's meaning.

**Evidence (mechanical).** A regression test pins the committed `sonnet-4.6` disinterested smoke
fixture (clauses quantifying `forall x :: … HasId(t, x) …`): it validates via the rewrite. A unit
test pins the rewrite's soundness on the guarded shape and its bail-out on an unrecognized one.

### AC-4 — The enriched battery rejects over-claims with no false-valids

The concrete-tree battery is enriched (single-entity, empty-refs, self-reference, multiple distant
cross-references, larger trees) so that a genuine over-claim is false on some battery tree. An
adversarial suite of known over-claims — including shapes the mutant bank does not cover — is each
rejected (`ExecOverclaim` / not valid). No over-claim in the suite passes the gate.

**Evidence (mechanical).** A test runs a battery of hand-authored over-claims (wrong rename, frame
violation, partial/over rewrite, spurious-id, and at least one non-mutant shape) through the gate
and asserts every one is rejected; plus the existing mutant-distinguishing coverage on the
enriched battery.

### AC-5 — A calibration run bounds the residual and confirms no arm bias

A calibration sweep (N≈10–20 × three models) on the certified gate is recorded and hand-audited:
the per-arm `unexecutable`/`inconclusive` residual is below a small stated bound, the disinterested
and incentivized arms show no systematic validity-classification bias attributable to the
instrument, and a manual spot-audit confirms every sampled `valid` is genuinely valid and every
`invalid` genuinely over-claims. A decision (`aiwfx-record-decision`) records the instrument's
error bounds and any accepted residual class as the certified boundary.

**Evidence (mechanical + recorded).** The calibration `results.json` (committed or quoted) with the
per-arm residual counts; a decision entity stating the bounds. This is calibration, not the
recorded run — `M-0011` runs N=30 on the certified gate afterward.

## Constraints

- **Pre-registration preserved.** §6 procedure, thresholds, combination rule, predictions
  (`bb1d220`) untouched; no edit to `prereg-reallocate.md`. The fixes change how validity is
  *decided*, never the frozen `1 − valid/extracted` formula.
- **Soundness over coverage — the load-bearing constraint.** No transform, helper-capture, or
  battery change may make an over-claiming spec validate (no false-valids). The guard rewrite is an
  exact equivalence; helper-capture only adds the model's own definitions; the battery only adds
  inputs. When a spec cannot be soundly decided, leave it `Unexecutable` (surfaced) rather than
  risk a false valid.
- **TDD required; zero warnings** (`clippy -D warnings`, `fmt --check`); determinism (G1) for all
  deterministic paths (the calibration sweep's API nondeterminism is isolated to AC-5).

## Design notes

- Extraction → helper-capture → quantifier-rewrite → assembly → the M-0012 gate. Each layer is
  local and unit-tested against its committed fixture.
- Helper-capture and the guarded-quantifier rewrite both transform the candidate before
  `ensures_to_conjunction`; thread the helpers alongside the ensures (prepend to the impl slot so
  `assemble`/`run_battery` need no new structure, and prepend to each mutant body in `score_spec`).
- AC-4's enriched battery raises the over-claim-catching power; keep every case satisfying the
  reallocation precondition (the AC-2/M-0012 precondition test extends to the new cases).

## Out of scope

- The reallocate run itself + the terminal decision — `M-0011` (resumes on the certified gate).
- Re-running or re-scoring E-0002 subjects.
- Changing the §6 procedure / thresholds / prereg.

## Dependencies

- Depends on `M-0010` (the instrument + frozen §6) and `M-0012` (the hybrid gate it extends and
  certifies). Addresses `G-0007`. **Blocks `M-0011`'s recorded run** — the run resumes once the
  instrument is certified.
