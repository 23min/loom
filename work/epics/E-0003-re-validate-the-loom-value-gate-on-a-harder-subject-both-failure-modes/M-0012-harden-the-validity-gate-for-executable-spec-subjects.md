---
id: M-0012
title: Harden the validity gate for executable-spec subjects
status: in_progress
parent: E-0003
depends_on:
    - M-0010
tdd: required
acs:
    - id: AC-1
      title: The validity gate falls back to concrete-tree execution for unprovable specs
      status: met
      tdd_phase: done
    - id: AC-2
      title: A committed concrete-tree battery covers the over-claim violation modes
      status: met
      tdd_phase: done
    - id: AC-3
      title: M-0009 calibration holds under the new validity gate
      status: open
      tdd_phase: green
---
## Goal

Replace the empty-body `dafny verify` validity gate with a **hybrid** that falls back to
concrete-tree execution (per `D-0003`), so correct-but-not-auto-provable specs (existentials,
iff-characterizations) count as valid and only genuine over-claims are rejected — restoring
construct validity to the `reallocate` over-claim §6 dimension before the run.

## Context

`M-0011`'s smoke test exposed `G-0006`: the empty-body `dafny verify` gate marks correct
thorough specs invalid (the rename existential `HasId(t', newId)` and iff-characterizations
are true but not auto-provable), confounding the over-claim dimension. `D-0003` (accepted)
chose the hybrid gate. This milestone implements it and re-calibrates `M-0009`, unblocking
`M-0011`'s run. It is a **pre-run instrument bug-fix**: no recorded run has happened; the §6
procedure / thresholds / predictions (`prereg-reallocate.md`, `bb1d220`) are untouched, and
the prereg's mechanism-agnostic §5 ("passed the validity gate") needs no edit — `D-0003` is
the committed record of the gate's mechanism.

## Acceptance criteria

### AC-1 — The validity gate falls back to concrete-tree execution for unprovable specs

`validate_spec` runs `dafny verify` first (the fast, sound path); when a spec is rejected, it
falls back to **executing** the candidate's `ensures` as a boolean predicate over the
committed concrete-tree battery via `dafny run --target:go`. A spec is **valid** iff it is
provable OR holds on every battery tree. A spec that cannot be executed (a ghost-only
construct, e.g. an unbounded quantifier) stays invalid and is counted in a distinct, surfaced
category — never silently.

**Evidence (mechanical).** Unit tests drive (a) an auto-provable spec → valid via the fast
path, (b) the actual `M-0011` smoke `opus-4.8` disinterested spec (existential + iffs) → valid
via the execution fallback, (c) a genuine over-claim → invalid, (d) a ghost-only spec →
invalid-uncategorized. Deterministic; no API.

### AC-2 — A committed concrete-tree battery covers the over-claim violation modes

A fixed battery of concrete `Tree` inputs (satisfying the preconditions `oldId != newId`,
`Valid(t)`, `HasId(t, oldId)`, `!HasId(t, newId)`) is committed, derived from the
{R, F, C}-violation modes the mutant bank encodes — at minimum: a cross-reference to `oldId`,
a distant reference (an entity elsewhere referencing `oldId`), and multiple entities.

**Evidence (mechanical).** A test asserts that for each `reallocate` mutant's broken clause,
some battery tree makes a spec asserting the gold clause distinguish the reference impl from
that violation — i.e. the battery exposes every over-claim mode the bank defines (bounding the
testing-incompleteness caveat `D-0003` flags).

### AC-3 — M-0009 calibration holds under the new validity gate

`M-0009`'s calibration is green under the hybrid gate: the gold spec validates, kills the full
mutant bank, and the over-claim test still rejects an over-claiming spec. The
previously-rejected correct spec (`G-0006`) now validates.

**Evidence (mechanical).** `reallocate_gold_calibrates_clean`,
`reallocate_over_claim_is_caught_by_validity_gate`, and the production-path calibration pass
unchanged; a regression test pins that the `G-0006` spec is now valid.

## Constraints

- **Pre-registration preserved.** The §6 procedure, thresholds, combination rule, and
  predictions (`bb1d220`) are untouched; `D-0003` (an ancestor of any future run) records the
  gate mechanism. No edit to `prereg-reallocate.md`.
- **Strict superset.** The hybrid gate only ADDS validity to specs the verify path rejected;
  it never removes validity from an auto-provable spec — so no E-0002 frozen result could
  change (those subjects are not re-run regardless).
- **Toolchain documented.** The Dafny Go backend (`dafny run --target:go` + `goimports`) is a
  new dependency for the validity step; document it and degrade clearly if the backend is
  absent.
- **TDD required; zero warnings** (`clippy -D warnings`, `fmt --check`); determinism (G1) — the
  battery + execution are deterministic.

## Design notes

- The execution fallback compiles per rejected spec (seconds); batch all battery trees into one
  `Main` per spec (one compile) to bound the cost. Only verify-REJECTED specs hit the fallback,
  so auto-provable specs stay fast.
- The kill-rate mechanism (`score_spec`'s mutant loop) shares the verify confound but is
  corroborating, not the §6 measure; leave it (note the known limitation) unless cheap to
  extend.
- The gate change is internal to `validate_spec`; reuse the existing atomic-write / workdir
  patterns.

## Out of scope

- The reallocate run itself and the terminal decision — `M-0011` (resumes after this).
- Re-running or re-scoring E-0002 subjects.
- Changing the §6 procedure / thresholds / prereg.

## Dependencies

- Depends on `M-0010` (the instrument + frozen §6) and the `M-0009` calibration it
  re-validates; addresses `G-0006` per `D-0003`.
- **Blocks `M-0011`'s run** — the run resumes once the gate is sound.

## Work log

Implementation landed in `feat(loom-ultralight): hybrid validity gate with execution fallback`
(commit `229750f`). The `Validity` enum + `is_valid`/`label`, `ensures_to_conjunction`, the Go
backend helpers (`go_backend_path_env` / `go_backend_available` / `run_dafny_exec`),
`run_battery` / `execute_validity`, the rewritten `validate_spec`, `ExecCase` + per-subject
`exec_battery` (`REALLOCATE_BATTERY`), and the census surfacing all land in that one commit, plus
the README toolchain note. The post-review corrective changes (fail-fast `require_exec_backend`,
the `inconclusive` census column, the empty-`ensures` guard) are folded into the same commit.

- **AC-1** — hybrid gate with four distinct classifications. `reallocate_gold_spec_is_valid…`
  (Provable), `reallocate_smoke_opus_disinterested_validates_via_execution` (ExecValid + verify
  rejects it), `reallocate_over_claim_is_caught_by_validity_gate` (ExecOverclaim),
  `reallocate_ghost_only_spec_is_unexecutable` (Unexecutable), plus
  `validity_partition_and_labels_are_total` and `ensures_to_conjunction_splits_clauses_and_scopes_comments`.
  · commit `229750f`
- **AC-2** — `REALLOCATE_BATTERY` committed; `reallocate_battery_cases_satisfy_precondition` and
  `reallocate_battery_distinguishes_every_violation` prove every {R,F,C} mutant mode is exposed.
  · commit `229750f`
- **AC-3** — calibration green under the hybrid gate; `reallocate_gold_calibrates_clean`,
  `reallocate_over_claim_is_caught_by_validity_gate`, the (ignored) `production_scorer_calibrates_every_subject`,
  and the `G-0006` regression all pass. · commit `229750f`

## Decisions made during implementation

- **`D-0003`** (accepted, pre-milestone) — the hybrid `dafny verify` + concrete-tree execution
  gate. Implemented here.
- **Post-review hardening (folded into `229750f`).** The independent two-lens review surfaced
  that an `Inconclusive` spec (Go backend absent, or a verify/exec timeout) was counted in
  `extracted − valid` (the over-claim numerator) but not surfaced — so a backend-absent run
  would silently corrupt the exact §6 number this milestone fixes. Resolved two ways, keeping
  the frozen `1 − valid/extracted` formula byte-for-byte: (1) `require_exec_backend` fails the
  run fast on a battery subject when the backend is absent ("degrade clearly", `D-0003`); (2) an
  `inconclusive` census column parallel to `unexecutable` in `results.json` / the table, so the
  residual is auditable. Also guarded the empty-`ensures` → vacuous-`true` path to
  `Unexecutable` rather than a silent valid.

## Validation

- `cargo test`: **53 passed; 0 failed; 4 ignored** (the 4 ignored are the slow full sweeps).
- Slow production-path calibration: `cargo test production_scorer_calibrates_every_subject --
  --ignored` → **1 passed** (every subject's gold validates + kills its full bank under the new
  gate).
- `cargo clippy --all-targets -- -D warnings`: clean. `cargo fmt --check`: clean. Build: green.
- Toolchain present and exercised end-to-end: dafny 4.9.0, go (`/usr/local/go/bin`), goimports
  (`$HOME/go/bin`). The four AC-1 classifications and both AC-2 tests run the real Go backend.
- `prereg-reallocate.md` is unmodified (still at freeze commit `bb1d220`); the diff touches only
  `src/main.rs` and `README.md`.

## Deferrals

None blocking. No deferred ACs; no new gaps required.

## Reviewer notes

- **Independent two-lens review (fresh-context).** Code review: **APPROVE** (5 non-blocking
  findings). Design review: **well-shaped**, one finding to fix before wrap. Both converged on
  the `Inconclusive` silent-fold, which is fixed (see Decisions). The remaining corrective
  changes are small and directly responsive to the reviewers; confirmed mechanically (tests
  green) rather than re-dispatching a full review round.
- **Notes deferred as fine-as-is (reviewer-endorsed):** the `LOOM_CASE <i>=` marker parse is a
  prefix match but safe under emission order with the 3-case battery (worst case a safe
  `Unexecutable`, never a false valid); `run_dafny_exec` mirrors `run_dafny`'s wait-then-read
  (deadlock-free while output stays under the pipe buffer — bounded for the tiny battery
  program); the clause-splitting convention is encoded in three functions
  (`extract_spec_ensures`, `ensures_to_requires`, `ensures_to_conjunction`) — a mild C1 latent
  worth unifying if a second executable-spec subject lands, left as-is for now.
- **Deliberately untested branches (environmental degradation).** The backend-absent path
  (`execute_validity` → `Inconclusive`; `require_exec_backend` → exit) and the verify/exec
  timeout → `Inconclusive` path are not unit-tested — they need the toolchain removed or a real
  hang. The testable predicate `exec_backend_missing` is pinned for the empty-battery
  short-circuit (`exec_backend_not_required_without_a_battery`).
- **Scope held.** §6 procedure / thresholds / combination rule / prereg untouched; the kill-rate
  scorer is corroborating (not the §6 over-claim measure) and is left unchanged per the design
  note.
