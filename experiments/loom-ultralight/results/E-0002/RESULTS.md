# E-0002 — re-validation result: **NO-GO**

**Decision: NO-GO.** The claim-weakening effect found in M-0002 (canonicalize) did
**not reproduce** on either of the two fresh, harder aiwf invariants. Per the
pre-registered combination rule (M-0007), this discharges **D-0001** with a clean
falsification: do **not** build the full loom-light pipeline on this evidence.

## The run

- **Model:** `opus-4.8` only — the pre-registered primary model (prereg §5: "the
  strongest effect in M-0002; the effect there rose with capability").
- **N = 30** trials per arm (disinterested / incentivized), per subject.
- **Subjects:** the aiwf status-transition FSM (`prereg-fsm.md`) and the prosey-title
  check (`prereg-prosey.md`).
- **Sample quality:** 0 API errors, 0 extraction failures — every generation produced
  an extractable spec; the missing valid counts are genuine over-claims caught by the
  validity gate, not noise.
- Raw per-subject artifacts (kill-rate, structural strength, verdict): `fsm/` and
  `prosey/` next to this file (copied from the gitignored `runs/<subject>/<ts>/`).

## Per-subject verdicts (the mechanical §6 map)

Pre-registered thresholds (shared, both subjects): material gap **Δ⁺ = 0.20**,
localization ceiling **Δ⁰ = 0.10**, minimum power **V = 10** valid specs/arm,
inconclusive ceiling **I = 0.10**.

| subject | valid_d | valid_i | tell_d | tell_i | **tell gap** | easy gap | inc | verdict |
|---|---|---|---|---|---|---|---|---|
| **fsm** | 29 | 15 | 0.979 | 0.960 | **0.019** | 0.000 | 0.000 | not-reproduced |
| **prosey** | 29 | 28 | 1.000 | 1.000 | **0.000** | 0.000 | 0.006 | not-reproduced |

Both subjects are well-powered (valid ≥ V = 10) with inconclusive rates far under I.
The tell gaps (0.019, 0.000) are an order of magnitude below Δ⁺ = 0.20: the
pre-registered prediction — that the incentivized arm under-specifies the load-bearing
content (the FSM's negative space; prosey's multi-sentence rule) — did not appear.
Among valid, resolving specs, the incentivized arm pins the tell at 96–100%, matching
the disinterested arm. On the most capable model, where the effect was predicted to be
strongest, it is essentially absent.

`combine(not-reproduced, not-reproduced)` → **NO-GO**.

## A real signal, in a different channel (recorded, not scored)

The FSM incentivized arm collapsed on the **validity** gate: **15/30 valid (50%)** vs
the disinterested arm's 29/30 (97%) — 15 of the run's 19 total invalid specs. So the
incentive **did** distort behavior on the FSM subject — by **over-claiming** (writing
specs too strong for even the correct `IsLegal` implementation, excluded by the
validity gate), which is a *different* failure mode than M-0002's localized
under-specification.

Two consequences, both deliberate:

1. **It vindicates D-0001's two-gate design** (and AC-1's per-arm validity-rate
   requirement): without the validity gate, those 15 over-claimed specs would have
   polluted the strength measurement. The over-claim mode was surfaced, as required.
2. **It does not rescue the prediction.** The pre-registration fixed *under-specification
   of the tell* as the effect to detect; that is falsified. Relabeling the over-claim
   signal as a reproduction after observing it is exactly the post-hoc move
   pre-registration forbids. The verdict is faithfully not-reproduced; the over-claim
   texture is recorded here for the audit trail and as a lead for future, separately
   pre-registered work.

## Pre-registration precedes the run (the AC-2 guard)

Each prediction was committed before the run; the recorded result names its
pre-registration commit, and `loom-ultralight --check-prereg-ancestry` verifies each is
a git ancestor of the run commit (this commit):

| pre-registration | commit |
|---|---|
| `prereg-fsm.md` | `22cd65e` |
| `prereg-prosey.md` | `91faa23` |
| `prereg-combination.md` (the combination rule) | `3e58ca1` |

The harness code that produced these results is committed together with them in the
run commit (so the code ↔ result pair is reproducible, G1).

## What NO-GO means — and what it does not

- It means: the single-toy-invariant M-0002 finding does **not generalize in the
  predicted form** to these two fresh, harder subjects, with this model. Do not build
  loom-light on this evidence.
- It does **not** mean incentives have no effect on spec quality — the FSM over-claim
  signal shows they do. It does **not** mean M-0002 was fake. And it is only 2 subjects
  and 1 model: absence of the predicted effect is not proof of its impossibility.

This NO-GO is one honest input to the eventual loom-light go/no-go. A genuinely more
complex subject — and a study that pre-registers **both** failure modes
(under-specification *and* over-claiming) — is the legitimate next step, as its own
pre-registered inquiry (a successor epic), never as a post-hoc expansion of this one.

## Reproduce (no API — from the committed artifacts)

The per-subject `verdict.json` records the inputs and the mechanical verdict; re-derive
the decision from the two verdicts:

```sh
cargo run --release -- --decide results/E-0002/fsm results/E-0002/prosey
# fsm = not-reproduced / prosey = not-reproduced / => decision: NO-GO
```

To re-run the experiment from scratch (paid, needs `ANTHROPIC_API_KEY`):

```sh
LOOM_MODELS=opus-4.8 LOOM_TRIALS=30 ./run.sh --full
```
