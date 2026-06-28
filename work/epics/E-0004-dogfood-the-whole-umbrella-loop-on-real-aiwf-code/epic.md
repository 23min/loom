---
id: E-0004
title: Dogfood the whole umbrella loop on real aiwf code
status: proposed
---
## Goal

Establish — by feasibility dogfood, **not** a pre-registered experiment — whether a
non-formal author can drive loom's *whole* umbrella loop on **real** aiwf code: prose +
examples in, an LLM-authored formal section, a verifier and a gap report out. Decide from
observation whether to build the thin loom-light tool.

## Context

`E-0001` / `E-0002` / `E-0003` tested a single adversarial side-threat — whether an
incentivized LLM *weakens* a spec under a grading incentive (the "value-gate" / gaming
hypothesis). Three subjects under pre-registration returned NO-GO (`D-0002`, `D-0005`); the
one positive (`D-0001`) was post-hoc and did not replicate. That result is *good* for the
architecture — LLM-co-authored umbrellas are not adversarially hollowed out — but it left
loom's actual top-line claim untested: those experiments used decidable toy *models* and
stripped out the loop's content (the prose section, the examples, the human, and the gap
report itself).

This epic tests the concept that was never on trial: the **whole loop**, at small scale, on
**real** aiwf components. The human authors what they can (prose intent + concrete examples);
the LLM authors the formal claims and a plain-English back-translation; a verifier checks the
implementation against the claims and the claims against the examples; the difference surfaces
as a gap report the human reads and acts on. The full design is in
[`docs/loom-loop-poc.md`](../../../docs/loom-loop-poc.md).

## Scope

In scope:

- Turn the loop **end-to-end** on at least one real aiwf component (the status-transition FSM
  first), recording the umbrella, the gap report, and the four observations: **tractability**
  (does real code verify, or drown in category-(B) timeouts), **faithfulness** (do the claims
  match the examples and the intent), **value** (did a gap or a proved-but-unclaimed finding
  tell us something true and useful), and **effort** (iterations, and whether the human had to
  read any formal text).
- The burden split: the human authors **Intent + Examples**; the LLM authors **Claims +
  back-translation**; the verifier + **gap report** close the loop.
- Whatever minimal scaffolding the loop needs to turn — manual plus a thin Dafny shell-out is
  acceptable for the first loop; build only what the next loop forces.
- A terminal **decision entity** recording the proceed-to-tool / rethink call, derived from
  what was observed.

Out of scope:

- **Building the thin tool** (the `loom-light` direction) — that is *building*, opened only as
  a separate epic if this one decides proceed.
- **A pre-registered confirmatory experiment** with thresholds (the `E-0003` epistemics) — a
  separate, later epic if feasibility warrants measuring.
- The `.lm` claims language — the umbrella's sections are realized as prose / examples /
  LLM-authored Dafny / gap report; a dedicated readable surface is a later evolution.
- Codegen, multi-user, cross-umbrella composition, and any human-subjects study (the
  originator dogfoods).

## Constraints

- **Real components, not toy models.** The subjects are actual aiwf invariants
  (`internal/entity/transition.go`, `internal/entity/canonicalize.go`, …), not models authored
  for the test.
- **The human never authors the formal section.** If the author has to read Dafny to steer the
  loop, that is a recorded **finding**, not a workaround.
- **Examples are the trust rail.** Claims are mechanically checked to agree with the human's
  concrete examples; the back-translation is audited against intent. Neither closes the
  spec-vs-intent gap fully — the gap report surfaces the residual rather than hiding it.
- **Reuse only the Dafny shell-out plumbing**, not `E-0003`'s reallocate-specific certified
  validity gate (it is subject-specific).
- **loom's load-bearing principles where they apply:** E3 (the gap report *is* the audit trail
  of each verification decision), G3 (observable), G1 (a recorded loop re-derives from its
  committed artifacts).
- **Feasibility framing:** no pre-registered pass/fail threshold; the output is recorded
  observation, not a verdict engineered to clear a bar.

## Success criteria

Observable at epic close (observational, not metric thresholds):

- The loop ran **end-to-end** on at least one real component, with the umbrella and the gap
  report recorded as artifacts.
- The four observations (tractability, faithfulness, value, effort) are recorded for every loop
  listed in the *Milestones* section that was run.
- Whether the human had to read formal text to steer is recorded — the load-bearing effort
  signal.
- A terminal **decision entity** records the proceed-to-tool / rethink call, derived from the
  recorded observations (and may close the epic early if the first loop is a clear stop).

## Open questions

Resolved just-in-time by the early loops; none blocking:

- **Impl fidelity.** Can the real implementation be soundly modeled in Dafny, and how is the
  model cross-checked against the real Go (against the same examples)? Resolved in the first
  loops.
- **The right tractability stress.** Which real component best exercises the verifier's limits
  — canonicalization's string / recursion path, or a stateful invariant? Resolved by the FSM
  loop's outcome.
- **Minimal scaffolding.** How much tooling does the loop need — manual plus a thin shell-out,
  or a small harness — decided as each loop forces it.

## Risks

- **Tractability wall.** Real code may drown the verifier in timeouts (category-(B) noise).
  Mitigation: finding *where* the wall is **is** a result; start with the Dafny-friendly FSM and
  add one source of realism at a time.
- **Model divergence.** The Dafny model of the real Go impl may not faithfully reflect it.
  Mitigation: cross-check the model against the same examples (and the real Go) before trusting
  a gap report drawn from it.
- **Over-structuring a feasibility probe.** Mitigation: plan milestones just-in-time, one loop
  at a time; do not front-load a full plan that will rot.

## Milestones

Sequenced via `aiwfx-plan-milestones`; success references every milestone listed here that was
run, not a fixed count. The first loop is specced in detail next; later loops are candidates
planned just-in-time, and any may be cancelled if an earlier loop closes the question:

- [`M-0014`](M-0014-turn-the-umbrella-loop-on-the-status-transition-fsm.md) — **the transition-FSM
  loop** (the first loop): turn the whole loop end-to-end on the real `transition` logic
  (Dafny-friendly: discrete, no string parsing), plus whatever minimal scaffolding the first loop
  needs. Validates the loop mechanics, ergonomics, faithfulness, and gap-report value.
  *(no dependencies)*
- **The canonicalization loop** — the string-based, per-kind-width, composite-recursion
  `Canonicalize`, a deliberate **tractability** stress run after the loop itself is validated.
  *(candidate; depends on the transition-FSM loop)*
- **The stateful-invariant loop** — push past pure functions toward the messiness real loom must
  survive. *(candidate; depends on the transition-FSM loop)*
- **The terminal decision** — record proceed-to-tool / rethink as a decision entity over the
  accumulated observations. *(closes the epic)*
