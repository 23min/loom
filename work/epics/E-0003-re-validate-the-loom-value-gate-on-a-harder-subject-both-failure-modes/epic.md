---
id: E-0003
title: Re-validate the loom value-gate on a harder subject, both failure modes
status: active
---

## Goal

Re-validate the loom value-gate on a genuinely more complex aiwf invariant,
pre-registering **both** ways the incentive can distort spec quality —
**under-specification** (E-0002's predicted tell) *and* **over-claiming**
(E-0002's recorded but un-pre-registered lead) — via a two-dimension §6 verdict.
The epic closes with a mechanical go/no-go that is a fair test of both failure
modes, not just one.

## Context

E-0002 returned a clean **NO-GO** (`D-0002`): the predicted claim-weakening
effect (the incentivized arm under-specifying the load-bearing content) did not
reproduce on the two fresh subjects (fsm, prosey). But the FSM incentivized arm
collapsed on the **validity** gate — 15/30 valid (50%) vs the disinterested
arm's 29/30 — i.e. the incentive *did* distort behavior, by **over-claiming**
(writing specs too strong for even the correct implementation), a different
failure mode than the pre-registered under-specification. That over-claim signal
was recorded qualitatively, **not** scored, because relabelling it as a
reproduction after the fact is exactly the post-hoc move pre-registration
forbids.

`D-0002` names the legitimate successor explicitly: *a genuinely more complex
subject, and a study that pre-registers both failure modes, as its own
pre-registered inquiry — never as a post-hoc expansion of E-0002.* This epic is
that successor. It builds on the harness generalized in E-0002 (the
`LOOM_SUBJECT` registry, the structural strength gate, the prereg-ancestry
guard) and on the two open harness gaps that E-0002 surfaced (`G-0004`,
`G-0005`), which the first milestone discharges before any new run.

## Scope

In scope:

- **Harden the harness** — close `G-0004` and `G-0005`: gate the strength
  population to valid specs via a single-source validity predicate, make the
  probe outcome routing unit-testable via an injectable closure, unify
  model-filtering across the kill-rate and strength outputs, make `verdict.json`
  self-contained (the per-arm over-claim rate legible from the artifact alone),
  and re-baseline the canonicalize golden — leaving the opus-4.8 verdict
  unchanged. This is the foundation the two-dimension verdict consumes.
- **Design the harder subject** — a fresh, more complex aiwf invariant with its
  own gold `.dfy`, mutant bank, obligation set, and calibration tests, staying
  inside Z3's decidable regime.
- **Author the two-dimension pre-registration** — the subject's §6 verdict map
  scored on **both** dimensions (under-specification *and* over-claiming), with
  the over-claim threshold and the rule combining the two dimensions into
  reproduced / not-reproduced / inconclusive fixed before the run, committed so
  the prereg commit is a git-ancestor of the run commit.
- **Run and decide** — execute the two-arm experiment on the harder subject,
  record the per-subject verdict, and apply the combination rule to a terminal
  go/no-go recorded as a decision entity.

Out of scope:

- Building the full loom-light pipeline — that is downstream of a PROCEED, not
  part of this re-validation.
- Re-running or re-scoring E-0002's frozen subjects (fsm, prosey); their results
  stand as recorded.
- Editing E-0002's frozen §6 verdict map or its pre-registrations — the
  two-dimension map is **new** code authored under this epic's own
  pre-registration; the E-0002 map and its oracle test are untouched.

## Constraints

- **Pre-registration precedes the run, both failure modes fixed before it.** The
  under-specification *and* over-claiming predictions, their thresholds, and the
  combination rule are committed before the run; ordering is verifiable from git
  via the ancestry guard, not asserted in prose (the `D-0001` / M-0002 integrity
  lesson).
- **A new subject is a new test under the same boundary.** Its own gold spec +
  mutant bank + pre-registration are committed before its run, with that
  prereg's SHA a git-ancestor of the run commit. One recorded subject — a
  replacement is a deliberate, recorded act, never an unbounded retry until one
  yields a reproduction (no subject-shopping).
- **The canonicalize golden re-baseline must be verdict-invariant.** The opus-4.8
  canonicalize verdict is verified unchanged (by running it), not assumed; only
  the non-primary-model rows move, to exclude over-claims now polluting the
  strength population.
- **loom's load-bearing principles hold throughout** — B2 schemas validated at
  boundaries, C3 atomic writes, E3/G3 audit trail on every verification decision,
  G1 reproducibility (same inputs → same outputs; Z3 nondeterminism isolated and
  surfaced, never folded into a result), D1 behaviour-pinned tests.

## Success criteria

Observable at epic close (not test names):

- A terminal **decision** (PROCEED / NO-GO / RERUN-OR-EXPAND) is recorded as a
  decision entity, derived mechanically from the two-dimension §6 verdict applied
  to the harder subject's run — no residual judgment exercised after results are
  visible.
- The decision's pre-registration commit is a git-ancestor of the run commit (the
  ancestry guard passes for every pre-registration the result names).
- Every harness gap listed in the *in scope* hardening item is closed: the
  over-claim rate is legible from `verdict.json` alone, the strength population is
  the valid population by construction, and the canonicalize golden is
  re-baselined with the opus-4.8 verdict unchanged.
- The run record is reproducible from committed artifacts with no API call: the
  decision re-derives from the recorded verdict(s) offline.

## Open questions

Deferred to milestone planning (after the hardening milestone), each with a
resolution path — none blocks the hardening:

- **The subject.** Candidates: a provenance / commit-trailer-coherence invariant,
  or an id-reallocation / collision invariant — both richer than the FSM while
  staying decidable. Resolved in the subject-design milestone via
  `aiwfx-record-decision`, constrained to Z3's decidable regime so the
  inconclusive rate stays under the pre-registered ceiling.
- **Model coverage.** opus-4.8-only (E-0002's pre-registered primary, where the
  effect was predicted strongest) vs a multi-model sweep. Fixed in the
  pre-registration before the run; the unified model-filtering from the hardening
  milestone makes either coverage clean.
- **The two-dimension combination rule.** How the under-specification and
  over-claiming dimensions map jointly to reproduced / not-reproduced /
  inconclusive (e.g. either dimension materially present ⇒ a distorting effect,
  vs both required). This is the core design of the pre-registration milestone,
  authored before the run and pinned against an oracle like E-0002's map.

## Risks

- **A harder subject pushes Z3 toward timeouts**, raising the inconclusive
  fraction toward an inconclusive verdict. Mitigation: choose a subject in the
  decidable regime, calibrate the Z3 budget before the run, and rely on the
  harness already surfacing and capping `inc` (the trichotomy never folds a
  timeout into a result).
- **Over-claiming dominates so heavily that few valid specs remain**, starving the
  strength (under-specification) measure of power. Mitigation: the validity-gate
  population is now reported per arm (the hardening milestone), the V floor guards
  power, and the two-dimension verdict scores a low valid count as the over-claim
  signal rather than discarding it as mere inconclusiveness.
- **Subject-shopping pressure** if the first subject does not reproduce.
  Mitigation: the constraint above — one recorded subject, a replacement is a
  deliberate recorded act under the identical prereg boundary, never an open loop.

## Milestones

Sequenced via `aiwfx-plan-milestones`; success references *every milestone listed
here*, not a fixed count. The foundation, subject-design, and pre-registration
milestones are allocated; the run is planned once the prereg is authored:

- [`M-0008`](M-0008-harden-the-loom-ultralight-harness.md) — **Harden the
  harness**: close `G-0004` + `G-0005`, plumb the over-claim rate, re-baseline the
  canonicalize golden (verdict-invariant). The foundation; lands first. *(no
  dependencies)*
- [`M-0009`](M-0009-design-the-id-reallocation-subject.md) — **Design the
  id-reallocation subject**: gold `.dfy` (the complete pin `{R, F, C}`),
  clause-isolated mutant bank, the reference-rewrite tell, and `--calibrate`
  calibration of the reallocation invariant. *(depends on `M-0008`)*
- [`M-0010`](M-0010-author-the-two-dimension-pre-registration.md) — **Author the
  two-dimension pre-registration**: score both failure modes (under-specification +
  over-claiming), fix each threshold and the combination rule, committed before any
  run (prereg SHA an ancestor of the run). *(depends on `M-0009`)*
- **Run and decide** *(not yet allocated)* — execute the two-arm run, record the
  verdict, apply the combination rule, and record the terminal decision
  discharging this epic.
