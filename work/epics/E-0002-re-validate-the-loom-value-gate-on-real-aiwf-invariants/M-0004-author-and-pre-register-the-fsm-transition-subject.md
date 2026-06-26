---
id: M-0004
title: Author and pre-register the FSM-transition subject
status: in_progress
parent: E-0002
depends_on:
    - M-0003
tdd: advisory
acs:
    - id: AC-1
      title: Gold FSM spec + reference impl verify
      status: met
    - id: AC-2
      title: 'Mutant bank: gold kills full bank, isolating mutant per tell'
      status: met
    - id: AC-3
      title: Each obligation probes as isolable single-input goal via M-0003 gate
      status: met
    - id: AC-4
      title: Committed pre-registration with falsifiable verdict mapping
      status: open
---
## Goal

Author the FSM status-transition subject — gold spec, reference implementation, and
clause-isolated mutant bank — and commit its pre-registration, so the two-arm
experiment can run on a relational invariant whose load-bearing obligation is
**negative space** (which transitions are *illegal*).

## Context

E-0002's first subject. aiwf's `internal/entity/transition.go` defines per-kind
legal status transitions; a faithful gold spec must assert not only the legal edges
but the **illegal** ones, terminality-as-no-outgoing, and one-directionality. The
negative-space obligation is exactly where a weak (positive-only) spec hides.
Depends on M-0003's generalized gate to confirm the obligations probe as isolable
single-input goals before the pre-registration is finalized. (aiwf source cloned at
`/tmp/aiwf-src`, github.com/23min/aiwf.)

## Acceptance criteria

Tracked as `acs[]` in frontmatter (AC-1 … AC-4); the full detail lives under the
per-AC sections at the foot of this spec.

## Constraints

- Single-input opaque probe — no quantifier over an entity collection; statuses are
  a bounded finite datatype.
- Clause-isolated mutants (G-0001): each breaks exactly one obligation.
- The pre-registration is committed before M-0006 is promoted to `in_progress`; its
  SHA will be asserted a git ancestor of the run commit in M-0006.

## Design notes

- Subject modeled from the per-kind table in `transition.go`; the exact kinds/edges
  slice is fixed at authoring.
- The predicted tell is the negative-space obligation: `!IsLegal(kind, from, to)`
  for illegal edges (a positive-only spec entails no exclusion → measured weak).

## Surfaces touched

- A new subject directory under `experiments/loom-ultralight/` (gold `.dfy`,
  mutant bank, prompts or reuse, the pre-registration artifact).

## Out of scope

- Running the experiment (M-0006). The prosey subject (M-0005).

## Dependencies

- M-0003 (the generalized strength gate).

## References

- E-0002 epic spec; `/tmp/aiwf-src/internal/entity/transition.go`; D-0001.

---

## Work log

All four ACs landed in the feat commit `22cd65e`; the wrap-side prose here is
blessed separately via `aiwf edit-body`.

### AC-1 — Gold FSM spec + reference impl verify

Authored `fsm.dfy` (Epic + Milestone, transcribed from `transition.go`): `Kind`/
`Status` datatypes, reference `IsLegal` predicate, gold `GoldSpec` lemma whose
ensures are the obligations **L** (4 legal edges), **X_skip**, **X_cross**, **T**
(Done/Cancelled terminality), **D** (one-directionality). `dafny verify fsm.dfy`
passes; pinned by `fsm_gold_verifies`.

### AC-2 — Mutant bank: gold kills full bank, isolating mutant per tell

Authored `mutants-fsm/` (11 clause-isolated mutants). `fsm_gold_kills_full_mutant_bank`
confirms the gold kills all 11; `fsm_mutants_are_clause_isolated` (#[ignore], 9×11
Dafny sweep) confirms each mutant breaks **exactly** its mapped obligation and that
the bank isolates **every** one of the 9 gold clauses (the G-0003 coverage guard).

### AC-3 — Each obligation probes as isolable single-input goal via M-0003 gate

`FSM_SUBJECT` (the gate's obligation list) + `fsm_obligations_probe_and_discriminate`:
the full spec entails all obligations; a positive-only spec entails the legal edges
but **none** of `{X_skip, X_cross, T1, T2, D}` (resolve-guarded) — the negative-space
tell discriminates the two specs.

### AC-4 — Committed pre-registration with falsifiable verdict mapping

`prereg-fsm.md`: full obligation set, the obligation↔mutant map, the predicted tell
(negative-space under-specification), strength thresholds, the falsifying outcome,
and a **total, falsifiable** mapping of run observations → reproduced /
not-reproduced / inconclusive (with the inconclusive boundary and the per-probe
inconclusive-denominator rule pinned). Its commit SHA becomes a git ancestor of the
M-0006 run commit.

## Decisions made during implementation

- FSM slice fixed at **Epic + Milestone** (2 of 6 kinds) — enough to exercise
  kind-dependence and all four obligation types without the full datatype; recorded
  in `prereg-fsm.md` §1.

## Validation

- `cargo test` — **14 passed, 2 ignored** (~40s). The FSM additions:
  `fsm_gold_verifies`, `fsm_obligations_probe_and_discriminate`,
  `fsm_gold_kills_full_mutant_bank` (default) and `fsm_mutants_are_clause_isolated`
  (#[ignore], ~3 min, exact mutant→obligation mapping + coverage).
- `dafny verify fsm.dfy` — 1 verified, 0 errors (AC-1).
- `cargo clippy` / `cargo fmt --check` — no new warnings or drift; the diff is
  **additive-only** (no reformatting of pre-existing canonicalize/run code).

## Deferrals

- (none) — M-0006 promotes `FSM_SUBJECT` into the production run path and authors
  the runtime two-arm prompts (the prompt templates are canonicalize-specific);
  that is M-0006's planned scope, not deferred M-0004 work.

## Reviewer notes

- **Independent two-lens review (wrap step 2):** code-quality (`wf-review-code`) →
  **approve** (all ACs verified by running; transcription faithful to
  `transition.go`; mutant→obligation map reproduced byte-for-byte; diff
  additive-only). Design (`wf-rethink`) → **refine** (sound). Four non-blocking
  findings were applied in-milestone: (a) added L2/L3 to `FSM_SUBJECT` so the
  instrument measures all four legal edges the pre-reg names (single source of
  truth); (b) the isolation test now asserts the **exact** mutant→obligation
  mapping + full coverage, not just cardinality — pinning the prereg §3 table and
  mechanizing the G-0003 guard; (c) pinned the per-probe inconclusive-denominator
  rule in the pre-reg so entailment rates are a deterministic function of raw probe
  outcomes (no post-hoc latitude); (d) fixed a garbled `mt3.dfy` comment.
- `FSM_SUBJECT` lives in the `#[cfg(test)]` module (it is only exercised by M-0004's
  probe test); M-0006 promotes it to production when wiring the run path.
- The per-subject verdict is pre-registered on `opus-4.8` (strongest M-0002 effect);
  there is intentionally no model fallback — opus under-production → inconclusive,
  which M-0007's combination rule handles.

### AC-1 — Gold FSM spec + reference impl verify

A gold spec + reference implementation for FSM transition legality exist and
`dafny verify` passes (gold valid against the reference impl). Statuses are a
finite Dafny datatype; legality is an opaque predicate over ground
`(kind, from, to)` tuples.

### AC-2 — Mutant bank: gold kills full bank, isolating mutant per tell

A clause-isolated mutant bank exists; the gold spec **kills the full bank** at
calibration; and the bank contains a mutant **isolating each pre-registered
predicted-tell obligation** — including the negative-space/exclusion obligation —
at the granularity the strength gate distinguishes (the G-0003 guard).

### AC-3 — Each obligation probes as isolable single-input goal via M-0003 gate

Each gold obligation **probes as an isolable single-input goal** through the M-0003
gate (a `StrengthSubject` whose obligations are exclusion goals over ground tuples
and bounded quantifiers over the finite status datatype).

### AC-4 — Committed pre-registration with falsifiable verdict mapping

A **pre-registration artifact is committed** naming the full obligation set, the
obligation(s) predicted to weaken under the incentivized arm, the outcome that
would falsify the prediction, the strength thresholds, **and a total, falsifiable
mapping from this subject's possible run observations into exactly one of
reproduced / not-reproduced / inconclusive (including the inconclusive boundary)**
— so no per-subject verdict judgment remains for after the run. Landed before the
M-0006 run.

