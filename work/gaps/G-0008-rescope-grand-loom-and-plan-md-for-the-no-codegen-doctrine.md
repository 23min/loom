---
id: G-0008
title: Rescope grand-loom and PLAN.md for the no-codegen doctrine
status: open
discovered_in: E-0005
---
## What

`PLAN.md` and the grand-loom vision still describe loom as a code-generating compiler (a Python execution backend, a target language, an actor runtime). ADR-0017 (accepted) retired that: loom generates no target code at any stage — the LLM does codegen. This gap tracks reconciling the older docs with the accepted doctrine.

## Scope

- Rescope `PLAN.md`'s grand-loom destination around "the LLM does codegen; loom verifies; `.lm` is the only future loom-emitted artifact."
- Review **ADR-0004** (no actor runtime in v0) — likely moot now (an execution/codegen concern) but confirm and reconcile.
- Confirm **ADR-0002** (Dafny backend) reads as the *first* substrate, per the multi-substrate direction in `docs/research/loom-reach-ambition-and-scope.md` and E-0005's frozen substrate seam.

## Why deferred

Not on E-0005's critical path — the loom-light build proceeds under the accepted ADR-0001/ADR-0017 doctrine. This is a deliberate documentation/architecture-hygiene pass to do when convenient.
