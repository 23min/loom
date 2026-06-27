# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/), and the project aims to follow
semantic versioning.

## [Unreleased]

### Added — E-0001: validate the loom differentiator (loom-ultralight)

Materialized and ran the loom-ultralight experiment (every milestone listed in the
epic's `wrap.md`): a Dafny + Rust harness that tests whether an LLM writes a weaker
spec when graded only on making its own implementation verify. The N=30 × 3-model
run showed a real, capability-scaling weakening localized to width-exactness,
confirmed by two independent measures — mutation kill-rate and a verifier-based
structural strength check. Decision `D-0001` records a qualified proceed to
loom-light. New harness modes: `--rescore` (re-score cached generations with no
API) and `--strength` (structural per-obligation entailment).

### Changed — E-0002: re-validated the loom value-gate on fresh aiwf invariants (NO-GO)

Generalized the loom-ultralight strength gate to any registered subject (`LOOM_SUBJECT`)
and ran the two-arm experiment (opus-4.8, N=30/arm) on two fresh, harder invariants — the
aiwf status-transition FSM and the prosey-title check — with the discriminating mechanism
pre-registered after the M-0002 correction. The endogenous claim-weakening effect did
**not** reproduce on either subject (tell gaps an order of magnitude below the
pre-registered threshold): decision `D-0002` records the resulting **NO-GO** — loom-light
is not greenlit on this evidence. Every milestone is listed in the epic's `wrap.md`. New
harness modes: `--decide` (apply the combination rule to two subjects' verdicts) and
`--check-prereg-ancestry` (the pre-registration-precedes-run git guard).
