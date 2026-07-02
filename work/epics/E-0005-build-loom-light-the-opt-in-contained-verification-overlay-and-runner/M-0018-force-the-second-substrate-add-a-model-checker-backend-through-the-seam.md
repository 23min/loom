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
      status: open
      tdd_phase: green
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

_(filled during implementation)_

## Decisions made during implementation

_(filled during implementation)_

## Validation

_(filled at wrap)_

## Deferrals

_(filled at wrap)_

## Reviewer notes

_(filled at wrap)_
