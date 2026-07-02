---
id: M-0016
title: Stand up the loom-light overlay, runner, and frozen contracts
status: in_progress
parent: E-0005
tdd: required
acs:
    - id: AC-1
      title: Overlay contained and removable-without-trace
      status: met
      tdd_phase: done
    - id: AC-2
      title: make loom is opt-in, off the default pipeline
      status: met
      tdd_phase: done
    - id: AC-3
      title: Gap-report schema frozen and reader-equivalent
      status: met
      tdd_phase: done
    - id: AC-4
      title: Report writes are atomic and reproducible
      status: met
      tdd_phase: done
    - id: AC-5
      title: Umbrella format agnostic; parse+dispatch total
      status: met
      tdd_phase: done
    - id: AC-6
      title: Seed properties verify; at-risk gap surfaces
      status: met
      tdd_phase: done
---
## Goal

Stand up the overlay pattern, the opt-in `make loom` runner, and the five frozen contracts — proven on the three-property aiwf Dafny seed, with the at-risk property surfacing its real gap. This is the machine the rest of E-0005 grows on.

## Context

First milestone of E-0005, building directly on E-0004 (`D-0006`, the qualified proceed) and its whole-loop mechanics + five-register umbrella convention. It establishes the five contracts every later milestone depends on, so getting them right here is the anti-rewrite investment. The three seed properties come from the E-0004 recall property plus the recognition probe on real aiwf source: FSM terminality, cancel-target edge-legality (the *at-risk* one), and the archive-location ⇔ FSM-terminality biconditional. Their aiwf source is referenced read-only at a pinned version.

## Acceptance criteria

### AC-1 — Overlay contained and removable-without-trace

The entire loom footprint in the downstream repo lives under one directory; removing it leaves the host's normal pipeline byte-identical. **Test:** stage the overlay, remove it, assert `git diff` is empty outside the overlay path and aiwf's default build is unaffected. *(Contract 1 — overlay boundary.)*

### AC-2 — make loom is opt-in, off the default pipeline

The downstream default build/CI graph never invokes loom; a separate `make loom` target runs the runner and emits gap reports; `make loom PROP=<id>` runs a single property. **Test:** assert the default target's dependency graph contains no loom invocation; assert `make loom` produces report files; assert `PROP=` scopes to one. *(Contract 4 — runner interface — + the opt-in constraint.)*

### AC-3 — Gap-report schema frozen and reader-equivalent

The gap report has a declared, versioned schema; every report the runner writes validates against it; a shared-scenario test drives the loom **writer** and the consumer **reader** over the same fixtures and asserts they agree. **Test:** schema-validation over emitted reports + a writer↔reader equivalence test on shared scenarios that fails if either side's shape drifts. *(Contract 2 · B2/D2 — the load-bearing seam.)*

### AC-4 — Report writes are atomic and reproducible

Reports are written temp-then-rename, so a crash mid-write never leaves a partial or corrupt report (fully-old or fully-new); and the same overlay + same pinned source yields byte-identical reports across runs. **Test:** an atomicity test (inject failure between temp-write and rename; assert the prior report is intact or absent, never partial) + a determinism test (two runs, byte-identical output; time/randomness at the edges). *(C3 atomic + G1 reproducible.)*

### AC-5 — Umbrella format agnostic; parse+dispatch total

The umbrella source is substrate-agnostic markdown with a declared `substrate:` field; the formal lowering (`.dfy`) is an attached artifact, not the source of truth. The parser is total — every umbrella file is parsed or explicitly rejected, none silently misparsed — and every parsed property routes to exactly one backend or errors, none silently unverified. **Test:** parser accepts a corpus of well-formed umbrellas and rejects malformed ones with a typed error (no silent drop); an exhaustiveness test over the substrate set maps each to one backend and errors on unknown. *(Contracts 3 & 5 · §4.5 totality.)*

### AC-6 — Seed properties verify; at-risk gap surfaces

Running the runner over the three-property aiwf overlay: FSM-terminality and archive⇔terminality verify clean; cancel-edge-legality's gap report contains the real `(B)` finding (the from→target edge not proven FSM-legal — the recognition probe's at-risk flag). **Test:** an end-to-end test asserting the three reports' verdicts (two clean, one with the specific expected gap), reproducible. *(Value demonstration.)*

## Constraints

- The five frozen contracts are established here and **must not move** afterward; everything behind them stays swappable.
- Opt-in and contained (per the epic constraints); host source referenced read-only and version-pinned.
- `tdd: required` — each AC lands red→green with a test that fails if the contract breaks.
- The `CLAUDE.md` load-bearing principles (B2/D2 schema at the seam, C3 atomic writes, G1 reproducible, E3 audit trail) are the bar.

## Design notes

- **Runner language: Rust** (decided per ADR-0001 — loom's implementation embodies its own correctness stance: robustness, type safety, elegance; and it stays **host-agnostic** — loom must be usable outside aiwf, so aiwf's Go is incidental, not a reason. Reuses the existing E-0004 Rust ultralight harness; the runner shells out to Dafny). loom generates **no** target code — code generation, where needed, is the LLM's role (ADR-0017).
- **Overlay layout:** `loom/<property>/{umbrella.md, <lowering>.dfy, gap-report.json, gap-report.md}` — finalized under AC-1/AC-5.
- **Gap-report schema:** JSON, versioned; the `.md` render is derived. The schema is the frozen contract (AC-3).
- The three seed properties reuse the E-0004 Dafny modeling approach (blind-authored model + claims), now driven through the runner rather than by hand.
- **E3 audit trail:** each report records what was checked, the inputs and source-version it saw, and the verdict with its reason.

## Out of scope

- The second substrate (model checker), tooled authoring/recognition, and self-host — later milestones of E-0005.
- Any `.lm` DSL surface.
- Standalone-binary extraction.

## Dependencies

- None — first milestone off `proposed` E-0005. Builds on E-0004 / `D-0006`.

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
