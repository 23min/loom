---
id: M-0007
title: Pre-register the subject-combination rule and go/no-go procedure
status: in_progress
parent: E-0002
depends_on:
    - M-0004
    - M-0005
tdd: advisory
acs:
    - id: AC-1
      title: Combination-rule pre-registration artifact is committed
      status: open
    - id: AC-2
      title: The rule is total and falsifiable over every verdict pair
      status: open
    - id: AC-3
      title: Artifact lands before the run for the M-0006 ancestor check
      status: open
---
## Goal

Pre-register the cross-subject **combination rule** — how the two per-subject
verdicts (reproduced / not-reproduced / inconclusive) combine into a single
epic-level go/no-go on building the full loom-light pipeline — committed before any
run, so it cannot be tuned to results.

## Context

The terminal decision (D-0001's discharge) rests on combining two per-subject
results. Each subject's pre-registration (M-0004, M-0005) defines when *that*
subject reproduces; this milestone pre-registers the rule that maps the *pair* of
categorical verdicts to proceed/no-go. It depends on both subjects being authored
and calibrated (so the rule is informed by what they turned out to be) but lands
before M-0006's run — a hard milestone boundary, the same integrity the per-subject
pre-registrations get.

## Acceptance criteria

The three ACs are tracked in frontmatter `acs[]`; each criterion and its evidence is
detailed under its `### AC-N` section below.

## Constraints

- Committed before any run; no result is visible when this is authored — the
  dependency edge plus the M-0006 ancestor guard enforce the boundary.
- Total and falsifiable: every verdict pair maps to a defined outcome.

## Design notes

- The rule operates on the categorical per-subject verdicts, which are defined by
  each subject's own pre-registration (M-0004, M-0005) — so it is subject-agnostic in
  form, but committed after the subjects exist so it can reflect them.
- It records the decision *procedure*; M-0006 mechanically applies it and records the
  actual decision entity.

## Out of scope

- Running the experiment or recording the decision entity (M-0006).
- The per-subject "reproduced" criteria (set in M-0004, M-0005).

## Dependencies

- M-0004 and M-0005 (both subjects authored, calibrated, and pre-registered).

## References

- E-0002 epic spec; D-0001 (the duty this epic discharges).

---

## Work log

All three ACs landed in the feat commit `3e58ca1` (the `combine` function + tests +
`prereg-combination.md`). No subject artifacts — this milestone records the decision
procedure; M-0006 applies it.

### AC-1 — Combination-rule pre-registration artifact is committed
`prereg-combination.md` states the rule, mixed-result resolution, and no-go
conditions · commit `3e58ca1`.

### AC-2 — The rule is total and falsifiable over every verdict pair
`combine` total over the 3×3 grid, pinned against an independent oracle · commit
`3e58ca1` · `combine_matches_preregistered_truth_table`, `combine_is_symmetric`.

### AC-3 — Artifact lands before the run for the M-0006 ancestor check
Committed and merged to the epic branch before M-0006 starts; ordering enforced by
the dependency edge · commit `3e58ca1`.

## Decisions made during implementation

- **Combination semantics — conjunctive proceed, any-negative no-go (chosen over a
  permissive bar).** PROCEED iff both subjects reproduce; NO-GO iff either is a genuine
  negative; else RERUN-OR-EXPAND. A real not-reproduced on either fresh subject is the
  falsification signal re-validation exists to detect and is never outweighed by the
  other's positive. The bar is intentionally above D-0001's literal "a fresh subject"
  floor — justified because D-0001 already burned one falsified pre-registered
  mechanism.
- **Procedure-as-code + totality test (not doc-only).** The rule is encoded as a pure
  `combine` function with an exhaustive truth-table test, so AC-2's "total and
  falsifiable" is mechanical rather than prose-asserted, and M-0006 applies the exact
  same single-sourced rule. Forward-declared with a named `#[allow(dead_code)]` (removal
  trigger: M-0006 wiring).

## Validation

- `cargo test` (non-ignored) → **21 passed, 0 failed, 3 ignored** (includes
  `combine_matches_preregistered_truth_table`, `combine_is_symmetric`).
- `cargo build` → green, **no warnings** (the `#[allow(dead_code)]` items are clean).
- `cargo fmt --check` → only the 9 pre-existing drifts at lines < 640 (the M-0007 code
  is fmt-clean). `cargo clippy` → no new warnings (only the pre-existing line-438 one).

## Deferrals

- (none)

## Reviewer notes

Independent two-lens review before wrap: **code-quality (`wf-review-code`) → APPROVE**
(every claim measured — tests, build, clippy, fmt, match-exhaustiveness via a real
4th-variant `rustc` probe, all 9 code↔doc cells); **design (`wf-rethink`) → SOUND**, a
faithful (deliberately conservative) discharge of D-0001, with one integrity finding.

Findings applied before the AC promotions:

- **Integrity fix — fenced the RERUN-OR-EXPAND remedies.** The design review found
  `expand/replace a subject` was unfenced post-hoc latitude (subject-shopping). §2 now
  binds any expand/replace subject to the identical pre-registration boundary (own
  prereg committed before its run, SHA a git-ancestor of the run commit) and bounds the
  loop — any `N` is terminal, so it cannot iterate fresh subjects toward reproduced.
- **A scoped independent confirmation review** verified that fix closes the gap and
  introduced no new leak, and caught a residual on the *rerun* path (un-pre-committed
  sample size → optional stopping). That too is now fenced: a rerun targets the
  pre-registered V power floor and the verdict is read once the floor is reached, never
  by stopping when the gap happens to cross Δ⁺.
- **§6 records four deliberate conservative trade-offs** the design review asked be
  documented rather than changed (conjunctive bar above D-0001's literal floor;
  symmetry resting on the shared-threshold invariant both sibling preregs pin;
  `(R,N)` → NO-GO collapses texture-specificity; categorical-before-combining trades
  pooling power for falsifiability).

Deliberately retained: `combine_is_symmetric` is deductively implied by the full
truth-table test, but kept as an intention-revealing, cheap guard.

### AC-1 — Combination-rule pre-registration artifact is committed

`prereg-combination.md` states the subject-combination rule: the mapping from the
pair of per-subject verdicts (reproduced / not-reproduced / inconclusive) to a single
epic-level decision. It spells out the **mixed** result (one reproduces, the other
inconclusive → RERUN-OR-EXPAND), and what counts as a **no-go** (either subject a
genuine negative). The two RERUN-OR-EXPAND remedies are fenced: a rerun re-measures
the *same* pre-registered subject (no new latitude, read at the V power floor), and an
expand/replace subject inherits the identical commit-before-run / git-ancestor
boundary — never an unbounded retry toward reproduced.

**Evidence:** `prereg-combination.md` committed in `3e58ca1`.

### AC-2 — The rule is total and falsifiable over every verdict pair

The rule yields a definite outcome (PROCEED / NO-GO / RERUN-OR-EXPAND) for **every**
one of the 3×3 verdict pairs, with no residual judgment after the run. This is
mechanical, not prose: `combine` is a total function (Rust match exhaustiveness;
a future `Verdict` variant forces a compile error, no catch-all), and
`combine_matches_preregistered_truth_table` enumerates the full grid against an
**independent** hand-written oracle — asserting every pair is covered exactly once
(totality) and `combine` matches the committed decision on each (no drift). NO-GO and
PROCEED are definite pre-committed outcomes the observation can force (falsifiable).

**Evidence:** `combine_matches_preregistered_truth_table`, `combine_is_symmetric`.

### AC-3 — Artifact lands before the run for the M-0006 ancestor check

The artifact is committed (`3e58ca1`) on the milestone branch and merged to the epic
branch (→ `main`) before M-0006 is promoted to `in_progress`. The ordering is
structurally enforced by the `M-0006 depends_on: M-0007` edge plus E-0002's standing
constraint that the recorded run commit names this prereg's SHA as a git ancestor. The
SHA-ancestor *check* runs in M-0006 (it owns the run commit); M-0007's obligation is
that the artifact exists and lands ahead of the run, which the dependency edge
guarantees.

**Evidence:** `prereg-combination.md` committed in `3e58ca1`; `M-0006` carries
`depends_on: M-0007` and is still `draft` (run not yet started).

