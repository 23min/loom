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
