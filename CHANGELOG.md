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

### Changed — E-0003: re-validated the loom value-gate on a harder subject, both failure modes (NO-GO)

Re-tested the loom value-gate on a genuinely harder, decidable aiwf invariant — the
id-reallocation / reference-rewrite invariant — pre-registering **both** ways the incentive can
distort spec quality (under-specification *and* over-claiming) in a two-dimension §6 verdict
(every milestone listed in the epic's `wrap.md`). To make the over-claim dimension trustworthy,
the validity gate was rebuilt as a hybrid — `dafny verify` with a concrete-tree execution
fallback (`D-0003`) — and the spec instrument was certified against a bounded,
adversarially-reviewed residual with no false-valids (`D-0004`). On the pre-registered primary
(`opus-4.8`, N=30/arm) **neither** failure mode reproduced — both arms 30/30 valid, tell and
over-claim gaps at 0.0 — so decision `D-0005` records a terminal **NO-GO**: the gaming
hypothesis is not supported on the primary model across four subjects now. New harness surface:
the hybrid `validate_spec` gate, helper-capture + guarded-quantifier-rewrite spec extraction,
and a self-contained multi-model `verdict.json`.

### Added — E-0004: dogfooded the whole umbrella loop on real aiwf code (qualified PROCEED)

Turned loom's whole umbrella loop — a non-formal author writing prose + examples, blind subagents
authoring the formal umbrella, a Dafny verifier + gap report closing the loop — on **real** aiwf
code (every milestone listed in the epic's `wrap.md`): the decidable status-transition FSM and
string-based id-canonicalization (laddered flat → recursive). The loop turned end-to-end with the
human entirely at the prose / gap-report layer (zero Dafny/Go read) and delivered real value in
both directions the bidirectional discipline predicts — a **code** gap filed upstream, and
**intent** errors the operator accepted (the emit-wide / accept-narrow conflation). It also mapped
loom's tractability boundary on strings (modeling + concrete-checking tractable; blind
universal-property discharge degrades and a `(B)`-failure stops self-diagnosing). Decision `D-0006`
records a **qualified PROCEED** to build the thin loom-light tool, scoped to decidable / structured
invariants first. New design docs: `docs/loom-loop-poc.md` (the whole-loop direction) and
`docs/loom-completeness-poc.md` (its superseded predecessor + a four-reviewer critique).
