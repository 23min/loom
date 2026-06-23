---
id: E-0001
title: Validate the loom differentiator (loom-ultralight)
status: done
---
## Goal

Cheaply test the load-bearing hypothesis behind loom **before** building loom-light:
that an LLM authoring a formal spec writes a *weaker* spec when it is also graded on
making its own implementation pass (incentivized) than when it only specifies
(disinterested) — **and** that a mutation / kill-rate check catches the difference.

If the gap appears and the check catches it, loom's differentiator is real and we
proceed to loom-light. If not, we learned it cheaply, before committing to a Rust
engine, a claims surface, or a verifier integration. The full design is in
`docs/loom-ultralight.md`.

## Scope

- A single-subject proof-of-concept on a **real aiwf invariant**: entity-id
  canonicalization (`internal/entity/canonicalize.go` + aiwf's `ADR-0008`),
  transcribed into self-contained Dafny — no runtime dependency on aiwf.
- Two authoring conditions (disinterested vs incentivized), one strong contract (the
  "gold spec"), and an 8-mutant bank as the calibration target.
- A **multi-model sweep** (Opus 4.8, Sonnet 4.6, Haiku 4.5) so the result speaks to
  whether endogenous weakening is a weak-model artifact or persists with capability.
- Kill-rate as the detector; the mean kill-rate gap as the headline measure.
- A go/no-go gate (D-0001) framed by the pre-registered outcomes in
  `docs/loom-ultralight.md` §5.

The burden split is the thesis in miniature: the assistant authors every artifact; a
small, human-auditable gold spec is the trust root; the human only installs the
toolchain (now in the devcontainer), sets the API key, and runs it.

## Out of scope

- Building loom-light (the Rust engine, the claims/`.lm` surface, the verifier
  integration) — gated on this epic's outcome.
- Multi-subject generalization (more aiwf invariants: the allocator, the lifecycle
  FSM) — future work if the effect holds.
- Target-language codegen, effects/capabilities, and the form of the `does` register
  — all deferred (ADR-0017).
