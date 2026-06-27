# Pre-registration — cross-subject combination rule (E-0002 / M-0007)

**Committed before the run.** This document is landed on `main` (via the epic
branch) before M-0006 is promoted to `in_progress`; its commit SHA will be named by
the recorded run result and **must be a git ancestor of the M-0006 run commit**.
Ordering is verifiable from git, not asserted in prose (the M-0002 integrity lesson).
The rule below may not be edited after the run — it is fixed here so the terminal
go/no-go is a deterministic function of the two subject verdicts, with no residual
judgment exercised after results are visible.

The rule as machine-checkable code is `combine` in [`src/main.rs`](src/main.rs); its
totality and exact mapping are pinned by `combine_matches_preregistered_truth_table`
(and symmetry by `combine_is_symmetric`). M-0006 applies `combine` to the actual
verdicts and records the decision entity.

---

## 1. What this combines

E-0002 re-validates D-0001's qualified proceed on **two** fresh aiwf invariants. Each
subject's own pre-registration produces one categorical verdict via its §6 verdict
map:

- **FSM status-transition subject** — [`prereg-fsm.md`](prereg-fsm.md);
- **Prosey-title subject** — [`prereg-prosey.md`](prereg-prosey.md).

Each verdict is one of **reproduced** (R), **not-reproduced** (N), or **inconclusive**
(I). This rule maps the *pair* of verdicts to one epic-level **decision**. It is
**subject-agnostic and symmetric** — neither subject is privileged; only the multiset
of the two verdicts matters. It is authored *after* both subjects exist (so it is
informed by what they turned out to be) but committed *before* the run (so it cannot
be tuned to results).

## 2. The decisions

- **PROCEED** — the effect re-validated; build the full loom-light pipeline.
- **NO-GO** — generalization is not established; do not build loom-light on this
  evidence.
- **RERUN-OR-EXPAND** — the result is not yet decidable: at least one subject is
  unmeasured (and neither is a genuine negative). Resolve the inconclusive subject,
  then re-apply this same rule. This is an explicit "inconclusive → act" outcome, not
  a residual judgment. The two resolution paths carry **different latitude, and both
  are fenced so neither reintroduces post-hoc freedom** (the M-0002 lesson):
  - **Rerun** — the *same* pre-registered subject, re-measured with more sampled specs
    or a longer Z3 budget. No new degrees of freedom: the subject, obligations, tell,
    and thresholds are already fixed; only the measurement's power changes. To close
    optional stopping, a rerun targets the pre-registered power floor (V = 10 valid
    specs/arm) and the verdict is read **once that floor is reached** — not by
    re-evaluating after each added sample and stopping when the gap happens to cross
    Δ⁺. Sampling raises power toward the floor; it never selects a stopping point.
  - **Expand/replace** — a *different* fresh subject. Choosing a new subject is
    choosing a new test, so it inherits the **identical pre-registration boundary**:
    its own gold spec + mutant bank + pre-registration committed *before* its run, with
    that prereg's SHA a git-ancestor of the run commit (E-0002's standing constraint).
    It is a deliberate, recorded act (its own milestone/decision entity), **not** an
    unbounded retry — the loop may not iterate fresh subjects until one happens to
    yield R. A replacement subject's verdict re-enters this same rule; the decision is
    PROCEED/NO-GO/RERUN on the new pair, never "keep swapping until proceed".

## 3. The combination rule (total over all 3×3 verdict pairs)

> **PROCEED** iff **both** subjects are R.
> **NO-GO** iff **either** subject is N.
> **RERUN-OR-EXPAND** otherwise (i.e. at least one I, no N, and not both R).

Evaluated in that order, every pair lands in exactly one decision. The full truth
table (symmetric — rows and columns are the two subjects, interchangeably):

| | other = R | other = N | other = I |
|---|---|---|---|
| **R** | PROCEED | NO-GO | RERUN-OR-EXPAND |
| **N** | NO-GO | NO-GO | NO-GO |
| **I** | RERUN-OR-EXPAND | NO-GO | RERUN-OR-EXPAND |

## 4. Why a genuine negative dominates (and the stability property)

D-0001's condition exists precisely because the original loom-ultralight finding was
on a single toy invariant **and its pre-registered mechanism was falsified** — so the
bar is "the effect robustly reproduces on fresh, harder subjects," *not* "assume it
generalizes." Two design commitments follow:

1. **Any N → NO-GO.** A genuine not-reproduced on either fresh subject is exactly the
   falsification signal the re-validation exists to detect. It is **never outweighed**
   by the other subject's R — a conjunctive proceed bar (both must reproduce) is the
   faithful discharge, and one clean negative is decisive against generalization.

2. **PROCEED requires both R.** One reproduction with the other subject merely
   unmeasured (not negative) is not sufficient re-validation for a *qualified* proceed
   that already burned one falsified prediction; it is RERUN-OR-EXPAND until the
   second subject is measured.

This yields a **stability property** that makes RERUN-OR-EXPAND principled rather than
a catch-all: PROCEED and NO-GO are **invariant under any resolution** of an
inconclusive subject, while RERUN-OR-EXPAND is exactly the set of pairs where
resolving the I *could* change the decision —

- `(N, I)` → NO-GO: if the I later resolves to R it is `(N, R)` = NO-GO; if to N it is
  `(N, N)` = NO-GO. Determinate regardless, so it is decided now.
- `(R, I)` → RERUN-OR-EXPAND: resolving the I gives `(R, R)` = PROCEED or `(R, N)` =
  NO-GO — genuinely undecided, so the rule defers to measuring it.
- `(I, I)` → RERUN-OR-EXPAND: nothing is measured; resolving could reach any outcome.

So RERUN-OR-EXPAND fires **only** when, and **always** when, the missing measurement
is outcome-determining — never gratuitously, never masking a decidable result.

## 5. Totality and falsifiability

The rule is a **total function** of the two verdicts: all 3 × 3 = 9 pairs map to a
defined decision, leaving no combination unhandled and no post-hoc latitude. This is
mechanical, not asserted: `combine_matches_preregistered_truth_table` enumerates the
full 3×3 grid against an independent hand-written oracle of §3's table — every pair is
covered exactly once (totality) and `combine` matches the committed decision on each
(no drift). It is **falsifiable** at the epic level in the same sense the per-subject
maps are: a NO-GO is a definite, pre-committed negative outcome that the observation
can force, and PROCEED is reached only by the one pre-committed positive combination.

## 6. Deliberate trade-offs (recorded, not latitude)

These are conscious conservative choices, fixed here so they are not relitigated after
the run:

- **The conjunctive proceed bar is intentionally above D-0001's literal floor.** D-0001
  conditions on re-validating on "a fresh, harder subject" (singular); this epic
  escalates to **two** subjects and requires **both** to reproduce. Stricter than the
  floor — justified because D-0001 already burned one falsified pre-registered
  mechanism, so the bar for "the effect generalizes" is deliberately high. A stricter
  bar cannot be an unfaithful (too-lax) discharge.
- **Symmetry rests on a shared-scale invariant.** Combining symmetrically (only the
  multiset of verdicts matters) is valid because each subject's verdict is already on
  the *same* categorical scale — both per-subject preregs pin the identical thresholds
  (Δ⁺ = 0.20, Δ⁰ = 0.10, V = 10, I = 0.10; see `prereg-prosey.md` §6 "shared with the
  FSM subject"). Asymmetric *weighting* (e.g. trusting finite-domain FSM over
  inconclusive-prone prosey) is deliberately rejected: it would reintroduce a judgment
  knob and weaken the generalization logic, which is strongest when diverse subjects
  are equal-weight independent tests.
- **`(R, N)` → NO-GO collapses texture-specificity into a terminal no-go.** A clean
  reproduction on one texture plus a genuine negative on the other is decisively NO-GO
  — loom-light is a *general* gate, and building it on evidence that it fails on one of
  two textures is exactly what D-0001's "do not assume it generalizes" warns against.
  If `(R, N)` actually occurs, M-0006's qualitative record notes the texture asymmetry;
  the go/no-go itself stays NO-GO.
- **Categorical-before-combining trades statistical power for falsifiability.** Each
  subject collapses to one verdict via its own §6 map before this rule sees it, so two
  subjects both *just* under Δ⁺ in the same direction read as `(N, N)` → NO-GO rather
  than pooling into a significant aggregate. This is the correct trade for an integrity
  gate: fine-grained pooling (weighting, fixed/random effects) reintroduces exactly the
  un-pre-registerable researcher degrees of freedom D-0001's correction exists to
  remove, and the lost case errs conservative (decline to build).

## 7. Boundary

This pre-registration is committed before M-0006 is promoted to `in_progress` (the
M-0006 → M-0007 dependency edge plus the git-ancestor check on the recorded run commit
enforce the ordering). M-0006 reads the two per-subject verdicts produced under their
own pre-registrations, applies `combine`, and records the resulting `Decision` as the
terminal artifact discharging D-0001. No part of this rule is decided after the run.
