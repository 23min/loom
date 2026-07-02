---
id: M-0018
title: 'Force the second substrate: add a model-checker backend through the seam'
status: in_progress
parent: E-0005
depends_on:
    - M-0016
tdd: required
acs:
    - id: AC-1
      title: Model-checker substrate routes through the frozen runner/backend seam
      status: met
      tdd_phase: done
    - id: AC-2
      title: A TLA+ property verifies proved end to end via loom verify
      status: met
      tdd_phase: done
    - id: AC-3
      title: A TLC counterexample surfaces as a category-(B) gap
      status: met
      tdd_phase: done
    - id: AC-4
      title: Gap-report schema carries the second substrate, version-gated v1 to v2
      status: met
      tdd_phase: done
    - id: AC-5
      title: The model checker's nondeterminism is isolated and surfaced
      status: met
      tdd_phase: done
---
## Goal

Add a second, architecturally-different verification substrate — a **model checker** (TLA+/TLC) — behind the frozen runner/backend seam (Contract 5), proving the seam and the other four contracts genuinely absorb a non-Dafny backend **without a rewrite**. Where M-0017 stressed the contracts from the *inside* (more Dafny), M-0018 stresses them from the *outside*: a backend of a different kind (explicit-state model checking vs deductive SMT), a different artifact (`.tla` / `.cfg`), a different verdict vocabulary (a counterexample trace, not an unproven obligation), and a different failure mode (state-space exhaustion). This is the epic's anti-rewrite bet cashed out — the seam is only proven swappable once a genuinely different substrate has flowed through it.

## Context

M-0016 froze the five contracts and stood up the Dafny backend; M-0017 self-hosted three loom properties as more Dafny, confirming the contracts hold from the inside with zero change. M-0018 is the outside test: a substrate with a different computational model. TLA+/TLC (explicit-state temporal model checking) is chosen over deductive verifiers precisely because it is *unlike* Dafny — it exercises the seam where a near-clone (e.g. a symbolic TLA+ engine) would not.

**Schema decision — path A (closed enum + version bump).** Adding a `Substrate` enum variant changes the generated JSON Schema (the enum's value space grows), which is a Contract-2 change. This milestone takes the deliberate, contract-honoring path the M-0016 design lens blessed: bump `SCHEMA_VERSION` 1→2, check in `gap-report.v2.schema.json`, and have readers version-gate. The alternative (an open `substrate` string, no bump) was rejected — it would weaken validate-on-read on that field; the whole point is to prove the *version-gated growth* path works. So M-0018 is the first real schema-version transition, and AC-4 pins it.

**Toolchain.** TLC needs a JRE + `tla2tools.jar`, provisioned into `.devcontainer` so rebuilds and CI stay reproducible — the same way `dafny` is. The TLC-backed ACs (AC-2/AC-3) skip with a notice when TLC is absent, mirroring the Dafny-backed tests, so the suite stays portable; the output→verdict mapping is unit-tested with canned TLC output and always runs.

## Acceptance criteria

### AC-1 — Model-checker substrate routes through the frozen runner/backend seam

A new `Substrate::Tla` variant and a `dispatch` arm route it to a new TLC backend; the totality tripwires still hold (`dispatch` exhaustive over `Substrate`, `Substrate::ALL` complete, `from_token` knows `"tla"`), and the umbrella-format and runner-interface contracts are unchanged. The overlay stays opt-in and off the default graph. **Test:** the exhaustiveness/`ALL`/`from_token` tests cover the new variant; an umbrella declaring `substrate: tla` routes to the TLC backend, none silently unverified.

### AC-2 — A TLA+ property verifies proved end to end via loom verify

An umbrella (`substrate: tla`) + a TLA+ module/config models a real property (an aiwf FSM safety/invariant property) and the TLC backend runs it to a graded `proved` verdict via `loom verify`. The TLC output→verdict mapping is **total**, unit-tested with canned TLC output (no TLC needed for that test — mirrors the Dafny summary-line parser). **Test:** an end-to-end `proved` under real TLC (skipped without it) + a total canned-output classifier test that always runs.

### AC-3 — A TLC counterexample surfaces as a category-(B) gap

When the property fails, TLC's counterexample trace becomes the gap `detail` — the at-risk demonstration for the new backend, reproducible (mirrors M-0016/AC-6). **Test:** an end-to-end refuted case whose gap carries the trace; the canned-output classifier maps a violation to a `code:"B"` gap with the counterexample.

### AC-4 — Gap-report schema carries the second substrate, version-gated v1 to v2

Adding the substrate bumps `SCHEMA_VERSION` 1→2 with a checked-in `gap-report.v2.schema.json`; the freeze test and the writer↔reader equivalence hold at v2; a v1 reader refuses a v2 report (the version gate is real, not decorative). **Test:** the freeze test regenerates v2; equivalence passes at v2; a v1-only reader rejects a v2 `schema_version`.

### AC-5 — The model checker's nondeterminism is isolated and surfaced

TLC runs under a pinned, bounded config; a state-space or time exhaustion is an `error` verdict, never a false `proved` — mirroring the Dafny backend's "gave up, not a proof" discipline (G1). **Test:** a canned TLC "too many states" / timeout output maps to `error`, not `proved`; the bound is pinned in the config, not wall-clock-only.

## Constraints

- **Opt-in, contained** — the second substrate adds no default-graph dependency; the overlay stays removable-without-trace.
- **Frozen seam holds** — the backend is added *behind* Contract 5; `dispatch` stays total and exhaustive, no catch-all. Any contract change is deliberate and version-gated (AC-4), never an ad-hoc edit.
- **Isolated nondeterminism (G1)** — TLC's search is bounded by a pinned config (a deterministic budget), not a wall-clock limit alone; "gave up" is surfaced as `error`.
- **`tdd: required`** — each AC lands red→green with a test that fails if the contract/claim breaks.

## Design notes

- The TLC backend mirrors the Dafny backend's shape: a pure output→verdict classifier (unit-tested with canned output, no toolchain needed) behind a `run` that shells out to `tlc` with the property dir as cwd (relative locations, reproducible). The classifier is total — `proved` only on a clean "no error" result, a violation → `refuted` + `(B)` gap with the trace, exhaustion/timeout → `error`.
- The property artifact is `<name>.tla` + `<name>.cfg` (TLC's model config), attached like `model.dfy` is for Dafny — the umbrella stays the substrate-agnostic source of truth.
- AC-4 is the first exercise of the version-gate the M-0016 reader contract described (a consumer dispatches on `schema_version` and refuses versions it does not know). The v1→v2 transition is additive (only the `substrate` enum grows), so a v2 reader still reads v1 reports; a v1 reader refuses v2.
- The seed TLA+ property should be a small aiwf FSM safety/invariant (e.g. an invariant TLC can check by exhaustive state exploration) — chosen so a model checker is the natural tool, distinct from the Dafny lemmas.

## Out of scope

- A third substrate, tooled authoring/recognition, and any `.lm` DSL surface — later milestones.
- Verifying loom's own Rust via TLC — no extraction (ADR-0017); TLC checks a modeled property, as Dafny does.
- Migrating the existing Dafny properties or reports to v2 semantics beyond the additive enum growth.
- A general backend-plugin system — YAGNI; the second substrate is added directly through the exhaustive seam, the third is the trigger to abstract if ever.

## Dependencies

- **M-0016** — the five frozen contracts, the runner, and the backend seam the TLC backend plugs into. (`depends_on` recorded.)
- M-0017 (done) is not a hard dependency but precedes this; the seam and schema it left unchanged are what M-0018 now grows.

## Work log

The TLC backend, the schema v1→v2 migration, the TLA+ overlay, and the `.devcontainer`
provisioning landed as one `feat` implementation commit (`3696583`); the wrap review added one
corrective refactor (folded into the same commit — see Reviewer notes). Per-AC TDD phase timelines
are in `aiwf history M-0018/AC-<N>`.

- **AC-1 — substrate routes through the seam** (met): `Substrate::Tla` + a `dispatch` arm to the
  new `Backend::Tlc`; the totality tripwires (`ALL`, `from_token`, the exhaustive match) extended
  to the new variant. `3696583`.
- **AC-2 — a TLA+ property verifies proved** (met): `examples/tlc-downstream/loom/fsm-terminality`
  (the Milestone FSM's terminal-absorbing invariant) model-checks `proved` end to end via the TLC
  backend; substrate `tla`, audit inputs `[model.tla, model.cfg]`. `3696583`.
- **AC-3 — a counterexample surfaces as a (B) gap** (met): the at-risk `cancel-reachability`
  property is `refuted`; TLC's counterexample trace (reaching `cancelled`) becomes the `code:"B"`
  gap detail. `3696583`.
- **AC-4 — schema carries the substrate, version-gated v1→v2** (met): `SCHEMA_VERSION` bumped 1→2
  with a checked-in `gap-report.v2.schema.json` (v1 retained as the historical contract); a new
  version-gated `GapReport::from_json` reader refuses a version it does not know. `3696583`.
- **AC-5 — nondeterminism isolated** (met): a total TLC output→verdict classifier — `proved` only
  on the completion sentinel, a violation → `refuted`+`(B)`, exhaustion/timeout/exception →
  `error`, never a false proof. `3696583`.

## Decisions made during implementation

No new decision entities were opened. The milestone realizes decisions already on record: ADR-0002
(Dafny backend — here generalized as the seam absorbs a second backend), the M-0016 frozen
contracts, and this spec's Context scope-narrowing (self-host the modelable subset stays M-0017;
this adds the outside substrate). Three implementation choices, within the ACs' latitude, recorded
here rather than as separate entities:

- **Schema evolution = path A** (closed enum + version bump + a version-gated reader), per the
  decision locked before detailing. The alternative (open `substrate` string, no bump) was rejected
  to keep validate-on-read strong.
- **TLC backend mirrors the Dafny backend** — a pure output→verdict classifier (canned-output
  tested, no toolchain) behind a `run` that shells out under a shared wall-clock ceiling. The
  spawn-under-timeout skeleton is factored into one `run_under_timeout` helper (see Reviewer notes).
- **The TLA+ overlay lives in a dedicated `examples/tlc-downstream/` fixture**, so M-0016's frozen
  seed is untouched; TLC is provisioned in `.devcontainer` (pinned `TLA_TOOLS_VERSION=1.7.4` + a
  JRE) for reproducibility.

## Validation

- `cargo fmt --check` clean; `cargo clippy --all-targets -- -D warnings` clean; `cargo build` green.
- `cargo test`: **78 tests pass** (M-0016/M-0017's 62 + 16 for M-0018: 3 `tlc_substrate` e2e, the
  TLC classifier + version-gate + timeout-isolation unit tests, and the tripwire extensions),
  incl. the three TLC-backed `tlc_substrate` tests (tla2tools 1.7.4 / TLC 2.19 on PATH).
- The v1→v2 migration is non-destructive: `gap-report.v1.schema.json` is unchanged, v2 grows the
  `substrate` enum additively, and M-0016's `verify_seed` reproducibility + M-0017's `self_host`
  tests pass unchanged now that they validate at v2.
- Both TLA+ models were vacuity-checked at review (falsifying each invariant flips the verdict).
- **Determinism note (AC-5):** boundedness of the TLC checks comes from the models being *finite by
  construction* (a four-value status FSM, full space explored) plus `-workers 1`, not a `.cfg`
  `CONSTRAINT`; the wall-clock `VERIFY_TIMEOUT` is the isolation backstop for a genuine hang, now
  exercised by `a_run_that_exceeds_the_timeout_is_an_error_not_a_hang`.

## Deferrals

None new. The `run_under_timeout` extraction the design review recommended was done inline, not
deferred. The reconciliation of ADR-0002/0003's aspirational `Verifier` trait with the as-built
exhaustive-`match` seam — now sharpened with a concrete "introduce the trait when a third backend
arrives **and** a stable shared interface emerges, or when dynamic backend selection is needed"
trigger — belongs to the existing **G-0011** (reconciling the ADRs' planned layout with the
as-built seam), not a new gap.

## Reviewer notes

Two-lens wrap review ran over the full (uncommitted) change-set from fresh-context reviewers.

- **Code lens → APPROVE.** The headline risk — a vacuous model that "checks" but proves nothing —
  was measured false for both TLA+ properties (falsifying each invariant flips PROVED↔REFUTED). The
  classifier cannot emit a false `proved` (it requires the exact completion sentinel); the v1→v2
  migration is correct and non-destructive; `run_tlc` is reproducible (unique metadir, cleaned up,
  `-workers 1`) and does not pollute the property dir.
- **Design lens → KEEP** the multi-backend seam, the schema-evolution design, and the
  `Substrate`/`Backend` enum split (a wire-vs-engine boundary, not a parallel-enum smell). The
  exhaustive `match` is deliberately kept over a `Verifier` trait: it buys compile-time routing
  totality a runtime registry would forfeit (YAGNI — abstract on the third backend, not the second).
- **Corrective fix folded in (design Q2):** the spawn-under-timeout protocol, previously duplicated
  between `run_dafny` and `run_tlc`, is extracted into one `run_under_timeout` helper — the single
  home of the G1 timeout-isolation invariant. This removed ~25 duplicated lines and made the
  timeout branch reachable by a test (now covered). The now-generic ceiling constant was renamed
  `DAFNY_TIMEOUT` → `VERIFY_TIMEOUT`.
- **Documented, not changed:** the two remaining subprocess-failure branches (spawn error, wait
  error) stay untested — they require a fake-binary seam, and this is consistent with M-0016's
  equally-untested `run_dafny` precedent. The single-integer schema version couples all consumers
  to any substrate addition (a deliberate KISS trade-off over per-field versioning).
