---
id: M-0014
title: Turn the umbrella loop on the status-transition FSM
status: in_progress
parent: E-0004
tdd: advisory
acs:
    - id: AC-1
      title: The FSM umbrella is authored under the burden split
      status: met
    - id: AC-2
      title: The transition logic is modeled in Dafny and cross-checked for fidelity
      status: met
    - id: AC-3
      title: 'The loop closes: a gap report is produced and recorded'
      status: met
    - id: AC-4
      title: The four observations are recorded
      status: open
---
## Goal

Turn loom's whole umbrella loop **end-to-end, once**, on the real aiwf status-transition
logic: the human authors prose intent + concrete examples, the LLM authors the formal claims +
a plain-English back-translation, a verifier checks the implementation against the claims, and
the gap report plus the four observations are recorded. The first real test of whether the loop
works — and whether it is worth evolving.

## Context

`E-0004`'s first loop. The status-transition FSM (`internal/entity/transition.go`) is chosen
first because it is the most verifier-friendly real component: discrete, enumerable status
transitions with guards, no string parsing or recursion — the cleanest path to a loop that
*completes*, so this milestone learns the loop mechanics, ergonomics, faithfulness, and
gap-report value before the canonicalization loop stresses tractability. Whatever minimal
scaffolding the loop needs is built here (manual invocation plus a thin Dafny shell-out is
acceptable; build only what this loop forces). The reference design is
[`docs/loom-loop-poc.md`](../../../docs/loom-loop-poc.md).

## Acceptance criteria

### AC-1 — The FSM umbrella is authored under the burden split

The umbrella for the status-transition contract exists with its sections authored by the right
party: **Intent** (prose) and **Examples** (concrete `from-status × to-status × guard →
allowed?` cases) by the human; **Claims** (formal Dafny) and a plain-English **back-translation**
by the LLM. The claims **mechanically agree** with every human example — a claim that contradicts
an example is caught without the human reading Dafny.

**Evidence (mechanical).** A recorded check that each example is consistent with the claims (the
claims, evaluated on each example tuple, match the human's expected allow/deny); the umbrella
artifact is committed.

### AC-2 — The transition logic is modeled in Dafny and cross-checked for fidelity

The real `transition` logic is modeled in Dafny, and the model is **cross-checked against the
same examples** (and against the real Go behavior on those examples) so a gap report drawn from
it reflects the real component, not a divergent model.

**Evidence (mechanical).** A recorded check that the Dafny model and the real Go agree on every
example tuple (same allow/deny); the model is committed.

### AC-3 — The loop closes: a gap report is produced and recorded

The verifier runs the modeled implementation against the umbrella's claims and emits a **gap
report** distinguishing claimed-and-proved (A), claimed-but-unproved (B, with the timeout /
limitation / failure sub-reason), and — where reachable — proved-but-unclaimed (C). The report
is recorded as an artifact.

**Evidence (mechanical).** The committed gap report (machine + human form) for the FSM loop; the
scaffold that produced it re-derives the same report (G1).

### AC-4 — The four observations are recorded

For this loop the four observations are written up: **tractability** (did it verify, or stall in
category-(B) timeouts), **faithfulness** (did the claims match the examples and intent; did the
back-translation read true), **value** (did any gap or category-(C) finding say something true
and non-obvious), and **effort** (iterations, and — the load-bearing one — whether the human had
to read any formal text to steer).

**Evidence (recorded).** A committed observations note covering the four, feeding the epic's
terminal decision.

## Constraints

- **The human never authors the formal section.** If steering the loop required the human to read
  or edit Dafny, AC-4 records it as a finding — not papered over.
- **Real component.** The model is of the actual `transition` logic, cross-checked against the
  real Go (AC-2), not a convenient invention.
- **Minimal scaffold.** Build only what this loop needs to close; reuse the existing Dafny
  shell-out plumbing, not `E-0003`'s reallocate-specific validity gate.
- **Feasibility, not a metric.** No pass/fail threshold; AC-4 records what happened, including an
  unflattering result (the loop stalls, or the claims are subtly unfaithful).
- **`tdd: advisory`.** The mechanical ACs (1–3) carry tests where it fits; the observational AC-4
  does not force a red→green phase.

## Design notes

- Loop shape (per `docs/loom-loop-poc.md` §3): human Intent + Examples → LLM Claims +
  back-translation + Dafny model of the impl → verifier (impl vs claims, claims vs examples) →
  gap report → human reads / decides.
- The FSM is a relation over an enum (statuses) with transition guards; the natural Dafny shape
  is a predicate `allowed(from, to, guards)`, with the umbrella's claims as `ensures` about it.
- The examples double as the fidelity oracle for both the claims (AC-1) and the model (AC-2) —
  the non-expert's anchor from below.
- **No metered API; the loop turns interactively via blind subagents.** The formal umbrella
  (Claims + back-translation) and the Dafny impl model are authored by fresh-context assistant
  subagents, not by a batch harness against a metered API key: the **umbrella-author** sees only
  the human's Intent + Examples (blind to the impl); the **impl-modeler** sees only the real
  `transition.go` (blind to the claims). Their isolation enforces the loom blinding — spec-author
  blind to impl, impl-modeler blind to claims — so the gap report is a genuine confrontation, not
  one hand harmonizing both sides. The verifier (Dafny + Z3) and the Go cross-check run locally
  and free. Independent/blinded authoring *at scale* and the metered batch API belong to the
  deferred confirmatory epic, not here.

## Out of scope

- The later loops (canonicalization tractability stress; a stateful invariant) and the epic's
  terminal decision — separate candidate milestones under `E-0004`.
- Any productized tool, claims language, or pre-registered confirmatory measurement.
- Iterating the umbrella to "perfection" — one honest end-to-end turn, with its gaps recorded, is
  the deliverable.

## Dependencies

- First milestone of `E-0004`; no milestone dependencies. Reference design:
  [`docs/loom-loop-poc.md`](../../../docs/loom-loop-poc.md).

## Work log

The whole loop turned once, end-to-end, on the real milestone FSM. Artifacts under
`experiments/loom-loop/milestone-fsm/` (umbrella, verifiable `.dfy`, gap report).

- **AC-1** — umbrella authored under the burden split: human Intent + Examples; a blind
  umbrella-author subagent wrote the Claims + back-translation; claims consistent with the
  examples. · `umbrella.md` · commit `fece1b4`
- **AC-2** — the real `transition.go` @ v0.20.0 modeled by a blind impl-modeler subagent;
  fidelity cross-checked three ways — `file:line` source evidence, the examples check, and a
  behavioral probe (a throwaway no-AC milestone promoted to `in_progress` on the live binary,
  then reset away). · commit `fece1b4`
- **AC-3** — the loop closed: `dafny verify` over model + claims produced the A/B/C gap report
  (**6 verified, 3 errors**), reproducible from the committed `.dfy` (G1). · `milestone-fsm.dfy`,
  `gap-report.md` · commit `fece1b4`
- **AC-4** — the four observations (tractability, faithfulness, value, effort) recorded. ·
  `gap-report.md` · commit `fece1b4`

## Decisions made during implementation

- No decision entity. The loop ran on the interactive / no-metered-API strategy recorded in the
  Design notes (blind in-session subagents author; local verifier).
- The headline result is a **finding about aiwf**, not a loom decision: aiwf permits a no-AC
  milestone → `in_progress` (candidate aiwf gap — see Deferrals).

## Validation

- `dafny verify experiments/loom-loop/milestone-fsm/milestone-fsm.dfy` → **6 verified, 3 errors**
  (C1, C2, and Examples-at-ex2), reproducible (G1).
- **C1 behaviorally confirmed** against real aiwf v0.20.0: promoting a no-AC milestone to
  `in_progress` succeeded (throwaway, reset away).
- `aiwf check`: 0 errors.
- **Independent fresh-context review: SOUND**, no blocking issues (see Reviewer notes).

## Deferrals

- **Candidate aiwf gap** (not a loom gap; aiwf is a separate repo): "a milestone can be promoted to
  `in_progress` with no acceptance criteria." Triple-confirmed by this loop; to be raised against
  aiwf if the operator judges it unintended. Recorded in `gap-report.md` Follow-ups.

## Reviewer notes

- **Independent adversarial review (fresh-context)** re-ran the verifier and traced `transition.go`
  at four layers (FSM table, `ValidateTransition`, the projection gate, the CLI) — verdict
  **SOUND**, the no-AC-start finding is **real, not a modeling artifact**, no blocking issues.
- The review surfaced a refinement now folded into `gap-report.md`: the two cancel surfaces diverge
  — `aiwf cancel M` (the cancel verb) *blocks* cancelling an open-AC milestone
  (`promote.go:246-253`), so the intent's "cancellable with ACs unmet" (C6) is refuted on that
  surface; C5/C6 verified only because the model took the more-permissive `promote cancelled` path.
  A second, smaller intent-vs-code gap.
- Model fidelity — the `(hasAC, allACsMet)` interface's lossiness vs the code's real "no open AC"
  guard — is disclosed in `umbrella.md` and `gap-report.md`, not hidden.
- This is a `tdd: advisory` feasibility milestone: the ACs are observational / loop-mechanical, so
  no red→green TDD cycle applies; evidence is the committed artifacts + the verifier + the review.
