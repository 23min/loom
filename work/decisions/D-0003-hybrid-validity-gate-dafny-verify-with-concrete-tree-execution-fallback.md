---
id: D-0003
title: 'Hybrid validity gate: dafny verify with concrete-tree execution fallback'
status: accepted
---
## Question

How should the loom-ultralight validity gate decide whether a candidate spec is "valid"
(not over-claiming) for subjects — like `reallocate` — whose correct specs naturally use
constructs (existentials, iff-characterizations) that are true of the reference
implementation but not discharged by empty-body `dafny verify`?

## Decision

A **hybrid validity gate**: run `dafny verify` first (fast, sound — unchanged for
auto-provable specs); for specs it *rejects*, fall back to **executing** the spec as a
boolean predicate over a fixed battery of concrete `Tree` inputs via `dafny run --target:go`.
A spec is **valid** iff it is provable OR holds on every tree in the battery. The battery is
derived from the {R, F, C}-violation modes the mutant bank already encodes, so genuine
over-claims (false on some input) are rejected while true-but-unprovable specs pass.
Implemented in `M-0012`; the §6 thresholds, combination rule, and predictions
(`prereg-reallocate.md`, committed at `bb1d220`) are untouched.

## Reasoning

- **The flaw (`G-0006`).** Empty-body `dafny verify` operationalizes "valid" as
  "auto-provable", which is strictly narrower than "true of the reference impl". Correct
  reallocation specs include the rename existential `HasId(t', newId)` and
  iff-characterizations that are true but need a witness / precondition reasoning, so they are
  falsely marked invalid — confounding the over-claim dimension and biasing against the
  disinterested arm's thorough specs.
- **Why execution.** On a CONCRETE tree, bounded quantifiers and existentials evaluate to
  ground booleans. Prototyped: opus's full correct spec evaluates `true` on a cross-referencing
  tree, while a genuine over-claim (`refs` unchanged) evaluates `false`. `dafny verify` on the
  same concrete tree still fails the existential (no auto-witness) — so execution, not
  verification, is the mechanism.
- **More faithful.** "Over-claiming = too strong for the correct impl" is, semantically,
  "false on some input" — exactly what testing detects. The hybrid is a strict SUPERSET of the
  current gate (it only adds validity to specs that were false-invalid), so it introduces no
  new false-VALIDs beyond the battery's coverage.
- **Caveats (carried into `M-0012`).** (1) Testing is incomplete vs proof — an over-claim
  false only OFF the battery would pass; mitigated by deriving the battery from the mutant
  scenarios. (2) Ghost-only specs (unbounded quantifiers) can't execute → a narrower residual
  invalid class. (3) New toolchain dependency: a Dafny compile backend (Go + `goimports`).
- **Integrity (pre-registration preserved).** This is a PRE-RUN instrument bug-fix: no
  recorded N = 30 run has happened; the M-0011 smoke data is calibration, not a result; the §6
  procedure / thresholds / combination rule / predictions committed at `bb1d220` are unchanged;
  the prereg remains a git-ancestor of any future run. prereg §5's validity-gate description
  gets a clarifying note. The decision is recorded BEFORE the run so the correction is
  auditable, not a post-hoc tuning of the analysis.
- **Alternatives considered.** *Replace the subject* — discards the M-0009 / M-0010 + prereg
  investment; reallocate's domain genuinely strains machine-checkable validity, but the fix is
  more economical and arguably improves the harness generally. *Run anyway* with a "valid =
  auto-provable" caveat — rejected: it confounds over-claiming with spec simplicity and is
  systematically biased against the disinterested arm.
