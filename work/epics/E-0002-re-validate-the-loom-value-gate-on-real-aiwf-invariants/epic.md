---
id: E-0002
title: Re-validate the loom value-gate on real aiwf invariants
status: proposed
---

# E-0002 — Re-validate the loom value-gate on real aiwf invariants

## Goal

Discharge D-0001's binding re-validation duty: reproduce the endogenous
claim-weakening effect on **two fresh aiwf invariants whose obligation textures
differ from id-canonicalization's**, with the discriminating mechanism
**pre-registered after the M-0002 correction** — producing a clean, falsifiable
go/no-go on whether to build the full loom-light pipeline.

## Context

E-0001 (loom-ultralight) found the load-bearing effect — an LLM graded only on
making its implementation verify writes a measurably weaker spec — but only after
correcting two harness defects (the line-scraping extractor, `G-0002`; the
under-sampling mutant bank and mispredicted clause, `G-0003`). The pre-registered
mechanism (a value-preservation tell) was **falsified**: the real tell was
width-exactness, and the clean pre-registered proceed gate was not met. D-0001 was
therefore accepted as a **qualified** proceed, conditioned on re-validating the
effect on a fresh, harder subject with the mechanism pre-registered *after* this
correction — not assuming the single-toy finding generalizes.

This epic discharges that condition. It is deliberately validate-first: it builds
only the one component re-validation requires and reuses the loom-ultralight
harness (the 20-mutant bank, `--rescore`, `--strength`, the validity + strength
two-gate). The two subjects were **pressure-tested against the strength gate's
probe shape before being locked in** (a candidate, id allocation/reallocation, was
rejected — its `reallocate` half is git-stateful I/O and its allocation
obligations are quantified over the entity collection, both of which mismatch the
single-input opaque-function probe and risk Z3-inconclusive results). The full
loom-light tool — surface, lowering, findings, aiwf integration — is a later
epic, gated on this result.

## Scope

### In scope

- Two fresh subjects drawn from real aiwf invariants, each authored as a gold spec
  + reference implementation + clause-isolated mutant bank (the `G-0001` isolation
  discipline). Both fit the strength gate's single-input opaque-function/predicate
  probe:
  - **FSM status-transition validation** (`internal/entity/transition.go`) — the
    per-kind legality relation. The load-bearing obligation is **negative space**:
    a complete spec must pin which transitions are *illegal*, not merely list the
    legal ones — plus terminality-derivation and one-directionality. Modeled with
    statuses as a finite Dafny datatype and an opaque legality predicate over
    ground `(kind, from, to)` tuples.
  - **Prosey-title detection** (`IsProseyTitle`, `internal/entity/entity.go`) — a
    pure `string → bool` with five isolable obligations (over-length, embedded
    newline, markdown markers, link brackets, and the subtle multi-sentence-boundary
    rule), each a single-input goal over an opaque predicate.
- A **committed pre-registration** per subject — the full gold-obligation set, the
  obligation(s) predicted to weaken, the falsifying outcome, and the strength
  thresholds — landed *before* that subject's paid run.
- Generalizing the structural strength gate (`--strength`) from the hardcoded
  canonicalize obligations to a **per-subject obligation list** (over an opaque
  function/predicate); no arbitrary-spec parsing required.
- Running the two-arm (disinterested vs incentivized) experiment on both subjects
  and recording the result against the pre-registration.

### Out of scope

- The claims **surface language** (ride native Dafny; markdown vs `.lm` is a later
  decision).
- The **ADR-0017 / ADR-0018 binding spike** (`does`-form, spec↔implementation
  binding) — these gate the surface/lowering, not the vacuity/strength gate.
- **Claims→Dafny lowering**, the **structured findings schema**, and **aiwf
  subprocess integration**.
- Target **code generation** (already out per ADR-0017).

## Constraints

- **Pre-registration is mechanically ordered before the run.** A subject's
  pre-registration is committed (and landed on `main`) *before* that subject's run
  milestone is promoted to `in_progress`; the recorded run result **names the
  pre-registration commit SHA and that SHA is a git ancestor of the run commit**.
  Ordering is verifiable from git, not asserted in prose — the integrity lesson
  from M-0002; no post-hoc rescue of a falsified prediction.
- **Mutant-bank granularity (the `G-0003` guard).** "Gold kills the full bank" is
  necessary but not sufficient: each subject's bank must include a mutant that
  isolates **each pre-registered predicted-tell obligation at its exact/bound
  granularity** — so a bank cannot be too coarse to catch the specific weakening
  the incentivized arm produces (the exact way G-0003 hid the effect).
- **Reuse, don't rebuild.** The loom-ultralight harness is the substrate; the only
  new code is the obligation-list generalization of the strength gate.
- **Native Dafny only** for both subjects; no new surface.
- **Single-input opaque probe.** Every gold obligation is a goal over an opaque
  function/predicate applied to one input (or a bounded quantifier over a finite
  datatype) — never a quantifier over an unbounded input collection or git state.
  Confirmed for both chosen subjects.
- **Reproducibility (G1).** The killed / survived / **inconclusive** trichotomy is
  preserved — Z3 nondeterminism is isolated and surfaced, never folded into a
  result.
- **Implementation-independent strength.** The generalized gate measures
  entailment against an opaque function/predicate, not against any one
  implementation.

## Success criteria

<!-- Observable at epic close, not tests. Reference-phrasing for the subject list. -->

- [ ] For every subject listed in *In scope*, a pre-registration artifact is
      committed whose SHA the subject's recorded run result names, and that SHA is
      a git ancestor of the run commit (pre-registration provably preceded the run).
- [ ] Each subject's pre-registration enumerates the **full gold-obligation set**,
      names the obligation(s) **predicted to weaken** under the incentivized arm,
      and states the **outcome that would falsify** the prediction (e.g. the gap is
      scattered across obligations, or concentrated in a non-predicted one).
- [ ] Every subject calibrates: its gold spec is valid against its reference
      implementation, kills its full mutant bank, and that bank contains an
      isolating mutant for each predicted-tell obligation at exact/bound granularity.
- [ ] The two-arm experiment has been run and recorded for every subject, against
      **both** the mutation kill-rate **and** the generalized structural strength
      measure.
- [ ] Each subject's result is mapped to its pre-registered edges (reproduced /
      not-reproduced / inconclusive), and a **pre-registered subject-combination
      rule** turns the per-subject results into a single epic-level go/no-go on
      building the full loom-light pipeline, recorded as a decision.

## Open questions

| Question | Blocking? | Resolution path |
|---|---|---|
| The exact slice of each subject (which kinds' transition tables; whether prosey's converse `IsProsey ==> some clause` is in scope) | no | fixed when the subject is authored, before its pre-registration |
| The pre-registered edges per subject (predicted-tell obligation, strength threshold, minimum gap) **and the subject-combination rule** | yes | committed in the subject-authoring milestone, before any run |
| Does FSM legality probe cleanly as an opaque relation over the finite status datatype (vs needing a different encoding) | no | confirmed in the gate-generalization milestone before that subject's pre-registration |

## Risks

| Risk | Impact | Mitigation |
|---|---|---|
| The effect does not reproduce on the new subjects | high | This is a *valid* result — it maps to a no-go and is cheap to have learned before building the pipeline. The epic is structured to make that outcome as informative as a positive one. |
| A per-subject mutant bank under-samples the weakened obligation (G-0003 recurrence) | med | The mutant-bank-granularity constraint requires an isolating mutant per predicted-tell obligation; calibration checks for it. |
| Z3 inconclusives corrode the signal on richer invariants | med | The single-input opaque-probe constraint deliberately excluded the collection-quantifier subject (id allocation) that would have caused this; the trichotomy isolates any residual inconclusive, never scoring it as survived. |
| Two subjects roughly double the authoring effort | low | Both ride the same harness and gate; only the subject + bank + pre-registration are per-subject. Authoring, not tooling, is the duplicated part. |

## Milestones

Sequence: M-0003 → (M-0004 ∥ M-0005) → M-0006. The gate generalization is
foundational (it confirms each subject's obligations are isolable single-input
goals before that subject's pre-registration is finalized), so it precedes subject
authoring; the two subjects then proceed in parallel.

| Milestone | Deliverable | Depends on |
|---|---|---|
| M-0003 | Generalize the structural strength gate to a per-subject obligation list (regression-verified against the canonicalize N=30 data) | — |
| M-0004 | Author and pre-register the FSM-transition subject (negative-space tell) | M-0003 |
| M-0005 | Author and pre-register the prosey-title subject (multi-sentence-rule tell) | M-0003 |
| M-0006 | Run the two-arm experiment on both subjects and record the verdict (ancestor-SHA guard, combination-rule go/no-go) | M-0004, M-0005 |
