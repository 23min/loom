# Epic wrap — E-0001

**Date:** 2026-06-23
**Closed by:** human/peter
**Integration target:** main
**Epic branch:** experiment/materialize-loom-ultralight
**Merge commit:** the `wrap-epic` merge of `experiment/materialize-loom-ultralight`
into `main` (recoverable via `aiwf history E-0001`)

## Milestones delivered

- M-0001 — Materialize the loom-ultralight experiment into runnable files (done `d8f216c`)
- M-0002 — Run the loom-ultralight experiment and record the kill-rate gap (done `5663aad`)

## Summary

E-0001 set out to cheaply validate loom's load-bearing hypothesis before building
loom-light: that an LLM authoring both a Dafny spec and an implementation writes a
*weaker spec* when graded only on making its implementation verify. M-0001
materialized the runnable harness (subject, gold spec, mutant bank, prompts, Rust
scorer) and calibrated it. M-0002 ran the paid sweep (N=30 × 2 conditions × 3
models) and recorded the result. The first run looked null; analysis traced that
to two harness defects (a line-scraping `ensures` extractor and a mutant bank that
pre-registered the wrong clause), which were fixed mid-flight — growing the bank
8 → 20 and adding a verifier-based structural strength measure. The corrected
result is clean: the incentivized arm writes a measurably weaker spec, the effect
rises with model capability (opus +0.18 > sonnet +0.07 > haiku +0.02), and it is
localized entirely to width-exactness (pinned vs merely bounded), confirmed by two
independent measures. The pre-registered value-tell / ≤5/8 prediction was falsified.

## ADRs ratified

- none. (The forward design constraint loom-light inherits — a structural checker
  rather than naive mutation — is captured as a binding consequence of D-0001;
  it becomes loom-light's own ADR when that epic starts.)

## Decisions captured

- D-0001 — Proceed from loom-ultralight to loom-light? — **accepted (qualified
  proceed):** the differentiator holds, but the clean pre-registered gate was not
  met (mechanism falsified; δ cleared in 1/3 models on kill-rate), so loom-light
  inherits a structural-checker requirement and a duty to re-validate the
  width-tell on a fresh, harder subject.

## Follow-ups carried forward

- none open. Every gap raised in this epic is addressed and archived — G-0001
  (mutant value-isolation), G-0002 (extractor drops multi-line specs), G-0003
  (value-tell misprediction / bank under-sampling).

## Handoff

loom-light is greenlit (qualified). It inherits three binding constraints from
D-0001: (1) the checker is structural strength — parse specs, measure
per-obligation entailment (exact / bound / absent) — not naive mutation;
(2) the width-tell is a hypothesis to re-validate on a fresh subject where
incompleteness can hide subtly, with the mechanism pre-registered after this
correction; (3) carry the two-failure-mode lesson — incentivized under-claims,
disinterested over/mis-claims, so the checker needs both a validity gate and a
strength gate. The loom-ultralight harness (`--rescore` / `--strength` modes,
the 20-mutant bank, the cached generations) remains as a reproducible reference.

## Doc findings

Scoped doc-lint over the epic change-set (README, `docs/loom-ultralight.md`,
`results/RESULTS.md`, this artefact): **clean.** All relative links resolve
(README → `results/RESULTS.md`; RESULTS → the two JSON tables; design doc →
`../experiments/loom-ultralight/results/RESULTS.md`); every cited entity id
(G-0001/G-0002/G-0003 archived, M-0001/M-0002, D-0001, E-0001) exists; no
removed-feature docs or dangling references.
