---
id: E-0005
title: 'Build loom-light: the opt-in, contained verification overlay and runner'
status: active
---
## Goal

Turn the E-0004 qualified proceed (`D-0006`) into a real, growable tool: an **opt-in verification overlay** a downstream repo adds under one removable directory, plus an external **runner** (living in this loom repo) that verifies umbrellas and emits schema-stable gap reports — architected **contracts-first** so the path from PoC to product is *additive, never a rewrite*.

## Context

E-0004 dogfooded the whole umbrella loop on real aiwf code and returned a qualified proceed (`D-0006`): the loop delivers real value on real code, but its push-button differentiator holds on decidable/structured properties and degrades on strings. Two research artifacts frame the build: [`docs/research/loom-reach-ambition-and-scope.md`](../../../docs/research/loom-reach-ambition-and-scope.md) (the property-shape catalogue and the graded multi-substrate ambition) and [`docs/loom-loop-poc.md`](../../../docs/loom-loop-poc.md) (the whole-loop mechanics + the five-register umbrella convention). A recognition probe on real aiwf source (four blind agents + the catalogue) surfaced a strong candidate set of loomable invariants — including the E-0004 properties found cold (recall) plus a novel authorization/mutual-exclusion harvest — and flagged which are only partially guarded. This epic builds the tool those candidates will run through.

The load-bearing design decision, established with the operator: **loom's containment and its organic growth are the same requirement.** The boundary that isolates loom's data from the downstream repo is the same boundary that makes eventual extraction a move rather than a rewrite.

## Scope

**In:**

- The **overlay pattern** — every loom artifact in a downstream repo lives under one removable directory (`loom/`); it references host source read-only and never intermingles with it. Delete the directory and the host repo is untouched.
- The **runner** — a real CLI (`loom verify <overlay>`), invoked from the downstream repo via one opt-in `make loom` target that is never part of the default pipeline. Advisory-by-default: gap reports are emitted; nothing fails the build unless a property is explicitly promoted to gating.
- The **five frozen contracts** (the anti-rewrite investment): (1) the overlay boundary; (2) the gap-report schema; (3) a substrate-agnostic umbrella format (markdown five-register source; the formal lowering is a per-substrate attached artifact); (4) the runner interface; (5) property independence (one property = one self-contained overlay subdir).
- The **three-property Dafny seed** on aiwf: FSM terminality (the E-0004 recall property), cancel-target edge-legality (the *at-risk* property — proves value by surfacing the real gap), and the archive-location ⇔ FSM-terminality biconditional.
- **Self-hosting the seed core** — loom verifies its own trust-critical plumbing (the gap-report writer↔reader schema equivalence, atomic writes, umbrella-parser totality, substrate-dispatch totality, reproducibility), which are exactly the `CLAUDE.md` load-bearing principles.
- The gap-report writer↔reader **equivalence test** (B2/D2) and **atomic writes** (C3).

**Out:**

- The full grand-loom, codegen, multi-user, and property composition.
- A polished `.lm` DSL — the umbrella's claims surface stays realized as prose + examples + LLM-authored formal for now (deferred per `loom-loop-poc.md` §8).
- Verifying loom's *probabilistic* parts (the recognition/authoring LLM steps are *used*, not themselves proven — only their output's well-formedness is checkable).
- Extraction to a standalone, separately-published binary — a later/successor concern; the engine stays in this repo, invoked against the overlay.

## Constraints

- **Opt-in, always.** loom never runs in the downstream repo's default build or CI graph. It is a separate, parameterized target.
- **Contained.** Every downstream-side artifact lives under the single overlay directory; the engine never enters the downstream repo. Host source is referenced read-only and version-pinned (a symbol + version, not a bare line number that silently drifts).
- **Contracts-first.** The five frozen contracts are established in the seed milestone and do not move; everything behind them stays swappable. New backends, tooling, and properties land *additively*.
- **The `CLAUDE.md` load-bearing principles are the build bar *and* the self-host target.** B2/D2 schemas + equivalence at the seam, C3 atomic writes, G1 reproducible, E3 audit trail — loom must honor them, and the self-host milestone proves it does, using loom.

## Success criteria

Observable at epic close (not tests):

- The overlay exists in aiwf under one directory and can be removed without trace, leaving aiwf's normal pipeline byte-identical.
- `make loom` runs opt-in, off the default pipeline, and emits gap reports that validate against the frozen schema.
- Every property in the seed milestone's acceptance set verifies, with the at-risk property surfacing its real gap in the report.
- loom verifies its own trust-critical seed core — the gap-report seam, atomic writes, and the parser/dispatch totality properties — using loom itself.
- Each of the five frozen contracts is documented and has a test pinning it, and the gap-report writer↔reader equivalence is tested against shared scenarios.
- Adding a property beyond the seed is demonstrably an additive increment — no contract changes required.

## Open questions

- **Runner home / structure.** The engine lives in this repo, implemented in **Rust** (per ADR-0001 — loom's own correctness stance, host-agnostic; reusing the E-0004 ultralight harness). loom generates no target code; code generation is the LLM's role (ADR-0017). The crate/module structure is decided at the seed milestone.
- **Model↔source binding.** How the umbrella pins the host symbol + version and how the runner offers a "re-check against current source" mode — resolved as part of the overlay-format contract in the seed milestone.
- **Advisory→gating policy shape.** How a property is promoted from advisory to build-gating, recorded per-property in the overlay — resolved when the first property is a gating candidate.

## Risks

- **Contract drift.** If any of the five contracts is set wrong at the seed, later milestones inherit a rewrite. Mitigation: the self-host milestone (pulled early) stress-tests the contracts from the inside before the tree grows.
- **Tractability regression on the harder properties.** The authorization-reachability and mutual-exclusion candidates may force a model checker sooner than planned (per the E-0004 string-frontier finding). Mitigation: the runner's backend seam is a frozen contract, so a second substrate lands additively.

## Milestones

Candidates; only the first is detailed just-in-time (via `aiwfx-plan-milestones` / `aiwfx-start-milestone`). Later ones are refined when reached.

1. **`M-0016` — Stand up the loom-light overlay, runner, and frozen contracts.** The overlay pattern, the `make loom` opt-in runner, and the five frozen contracts, on the three-property aiwf Dafny seed (FSM terminality, cancel-edge-legality [at-risk], archive⇔terminality). *(detailed; status `draft`)*
2. **Self-host the seed core.** Turn loom on loom: hand-author umbrellas for loom's trust-critical plumbing (gap-report schema seam, atomic writes, parser/dispatch totality, reproducibility) and verify them with the seed runner — the dogfood, and the from-the-inside validation of the frozen contracts. *(pulled early)*
3. **Force the second substrate.** Add a model-checker backend through the seam, on the repo-lock mutual-exclusion property and an authorization-reachability property — proving the anti-rewrite seam under real load. *(candidate)*
4. **Tool the authoring loop.** Turn the E-0004 blind-subagent authoring pattern into a `loom author` command. *(candidate)*
5. **Tool recognition.** Turn the recognition probe into a `loom suggest` command that emits umbrella stubs in the frozen format. *(candidate)*
6. **Terminal decision.** Productize / extract to a standalone tool, or iterate — recorded as a decision over the accumulated observations. *(candidate)*

## Dependencies

- Builds on E-0004 (`D-0006`, the qualified proceed) and its whole-loop mechanics + umbrella convention.
- References [`docs/research/loom-reach-ambition-and-scope.md`](../../../docs/research/loom-reach-ambition-and-scope.md) and [`docs/loom-loop-poc.md`](../../../docs/loom-loop-poc.md).
