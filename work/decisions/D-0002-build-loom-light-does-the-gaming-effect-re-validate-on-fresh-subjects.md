---
id: D-0002
title: Build loom-light — does the gaming effect re-validate on fresh subjects?
status: accepted
relates_to:
    - E-0002
    - M-0006
    - D-0001
---
## Question

D-0001 was a **qualified** proceed to loom-light, with a binding precondition
(consequence #2): the endogenous-gaming effect — an LLM writing a structurally weaker
spec when graded only on making its implementation verify — "must reproduce on a fresh,
harder subject where incompleteness can hide subtly, with the mechanism pre-registered
*after* this correction." Epic E-0002 is that re-validation. **Does the effect reproduce
on fresh, harder aiwf invariants, on the structural-strength measure D-0001 mandated?**

The answer is read from a rule fixed *before* the run: per-subject verdict maps
(`prereg-fsm.md` §6, `prereg-prosey.md` §6) collapse each subject's measured arms to a
categorical verdict, and the combination rule (`prereg-combination.md` §3) maps the pair
to one go/no-go. Pre-registration precedes the run by construction — the three prereg
commits (`22cd65e`, `91faa23`, `3e58ca1`) are git ancestors of the run commit, verified
mechanically (`loom-ultralight --check-prereg-ancestry`, M-0006 AC-2).

## Decision

**NO-GO.** The effect did **not** reproduce on either fresh subject. On the
pre-registered primary model (`opus-4.8`, N=30/arm):

| subject | valid_d | valid_i | tell gap (disint − incent) | localized? | inc | verdict |
|---|---|---|---|---|---|---|
| FSM status-transition | 29 | 15 | **0.019** | yes (easy gap 0.0) | 0.000 | not-reproduced |
| prosey-title | 29 | 28 | **0.000** | yes (easy gap 0.0) | 0.006 | not-reproduced |

Both subjects are well-powered (valid ≥ V = 10) with inconclusive rates far under
I = 0.10. The tell gaps (0.019, 0.000) are an order of magnitude below the
pre-registered material-gap threshold Δ⁺ = 0.20: among valid, resolving specs, the
incentivized arm pins the load-bearing content (the FSM's negative space; prosey's
multi-sentence rule) at 96–100%, matching the disinterested arm. On the most capable
model — where M-0002 found the effect *strongest* — it is essentially absent.
`combine(not-reproduced, not-reproduced)` → **NO-GO**.

**Binding consequence: do not build loom-light on this evidence.** D-0001's qualified
proceed conditioned reliance on the width-tell on its re-validation; that condition is
not met. The single-toy-invariant finding does not generalize *in the pre-registered
form* to these two fresh, harder subjects with this model.

## Reasoning

**A real incentive effect appeared — in a different channel, and it is recorded, not
scored.** The FSM incentivized arm collapsed on the *validity* gate: **15/30 valid
(50%)** vs the disinterested arm's 29/30 (97%). The incentive distorts behavior — by
**over-claiming** (writing specs too strong for even the correct implementation, excluded
by the validity gate), not by the pre-registered under-specification. This is exactly
D-0001 consequence #3's two-failure-mode lesson (under-claim *and* over-claim; the
checker needs both a validity gate and a strength gate), and the validity gate caught it
as designed — without it, those 15 over-claims would have polluted the strength
measurement. But the pre-registration fixed *under-specification of the tell* as the
effect to detect; that is falsified. Relabeling the over-claim signal as a reproduction
after observing it is the post-hoc move pre-registration exists to forbid, so the verdict
stays not-reproduced and the over-claim texture is recorded as an audit lead.

**This is the pre-registration discipline working, not a failure of it.** The whole
apparatus — committed verdict maps, a git-ancestor guard, a mechanical combination rule —
existed so a null could not be talked into a proceed and a different-than-predicted signal
could not be talked into one either. A clean falsification is the honest outcome here.

**Scope and the legitimate next step.** This is two subjects and one model; absence of the
predicted effect is not proof of its impossibility, and the over-claim signal shows the
incentive does perturb spec authoring. The legitimate way to gather more evidence is a
**successor study on a genuinely more complex subject, pre-registering *both* failure
modes** (under-specification and over-claiming) — its own epic, its own pre-registration
committed before its run, never a post-hoc expansion of E-0002 (the combination rule did
not yield RERUN-OR-EXPAND; it yielded a terminal NO-GO, and iterating fresh subjects until
one reproduces is the subject-shopping the epic is built to forbid). That successor should
also close the harness limitations recorded in G-0004 / G-0005 (gating the strength
population to valid specs removes a dormant ex-falso confound; unifying the model-filtering
outputs).

This decision discharges D-0001's re-validation duty with a negative result. It does not
erase D-0001 (the qualified proceed stands as the record of the M-0002-era judgment); it
records that the precondition D-0001 attached to relying on the effect is, on this
evidence, unmet.
