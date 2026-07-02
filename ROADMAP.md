# Roadmap

## E-0001 — Validate the loom differentiator (loom-ultralight) (done)

### Goal

Cheaply test the load-bearing hypothesis behind loom **before** building loom-light:
that an LLM authoring a formal spec writes a *weaker* spec when it is also graded on
making its own implementation pass (incentivized) than when it only specifies
(disinterested) — **and** that a mutation / kill-rate check catches the difference.

If the gap appears and the check catches it, loom's differentiator is real and we
proceed to loom-light. If not, we learned it cheaply, before committing to a Rust
engine, a claims surface, or a verifier integration. The full design is in
`docs/loom-ultralight.md`.

| Milestone | Title | Status |
|---|---|---|
| M-0001 | Materialize the loom-ultralight experiment into runnable files | done |
| M-0002 | Run the loom-ultralight experiment and record the kill-rate gap | done |

## E-0002 — Re-validate the loom value-gate on real aiwf invariants (done)

### Goal

Discharge D-0001's binding re-validation duty: reproduce the endogenous
claim-weakening effect on **two fresh aiwf invariants whose obligation textures
differ from id-canonicalization's**, with the discriminating mechanism
**pre-registered after the M-0002 correction** — producing a clean, falsifiable
go/no-go on whether to build the full loom-light pipeline.

| Milestone | Title | Status |
|---|---|---|
| M-0003 | Generalize the structural strength gate to a per-subject obligation list | done |
| M-0004 | Author and pre-register the FSM-transition subject | done |
| M-0005 | Author and pre-register the prosey-title subject | done |
| M-0006 | Run the two-arm experiment on both subjects and record the verdict | done |
| M-0007 | Pre-register the subject-combination rule and go/no-go procedure | done |

## E-0003 — Re-validate the loom value-gate on a harder subject, both failure modes (done)

### Goal

Re-validate the loom value-gate on a genuinely more complex aiwf invariant,
pre-registering **both** ways the incentive can distort spec quality —
**under-specification** (E-0002's predicted tell) *and* **over-claiming**
(E-0002's recorded but un-pre-registered lead) — via a two-dimension §6 verdict.
The epic closes with a mechanical go/no-go that is a fair test of both failure
modes, not just one.

| Milestone | Title | Status |
|---|---|---|
| M-0008 | Harden the loom-ultralight harness | done |
| M-0009 | Design the id-reallocation subject | done |
| M-0010 | Author the two-dimension pre-registration | done |
| M-0011 | Run the reallocate sweep and record the terminal decision | done |
| M-0012 | Harden the validity gate for executable-spec subjects | done |
| M-0013 | Harden the spec extractor for complex executable specs | done |

## E-0004 — Dogfood the whole umbrella loop on real aiwf code (done)

### Goal

Establish — by feasibility dogfood, **not** a pre-registered experiment — whether a
non-formal author can drive loom's *whole* umbrella loop on **real** aiwf code: prose +
examples in, an LLM-authored formal section, a verifier and a gap report out. Decide from
observation whether to build the thin loom-light tool.

| Milestone | Title | Status |
|---|---|---|
| M-0014 | Turn the umbrella loop on the status-transition FSM | done |
| M-0015 | Turn the loop on real string-based canonicalization | done |

## E-0005 — Build loom-light: the opt-in, contained verification overlay and runner (active)

### Goal

Turn the E-0004 qualified proceed (`D-0006`) into a real, growable tool: an **opt-in verification overlay** a downstream repo adds under one removable directory, plus an external **runner** (living in this loom repo) that verifies umbrellas and emits schema-stable gap reports — architected **contracts-first** so the path from PoC to product is *additive, never a rewrite*.

| Milestone | Title | Status |
|---|---|---|
| M-0016 | Stand up the loom-light overlay, runner, and frozen contracts | in_progress |

