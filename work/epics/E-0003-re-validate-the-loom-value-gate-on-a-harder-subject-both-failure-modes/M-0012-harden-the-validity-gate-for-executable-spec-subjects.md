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
      status: open
      tdd_phase: red
    - id: AC-2
      title: A committed concrete-tree battery covers the over-claim violation modes
      status: open
      tdd_phase: red
    - id: AC-3
      title: M-0009 calibration holds under the new validity gate
      status: open
      tdd_phase: red
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
