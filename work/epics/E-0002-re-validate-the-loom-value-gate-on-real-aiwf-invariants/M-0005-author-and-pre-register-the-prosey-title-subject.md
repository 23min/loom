---
id: M-0005
title: Author and pre-register the prosey-title subject
status: in_progress
parent: E-0002
depends_on:
    - M-0003
tdd: advisory
acs:
    - id: AC-1
      title: Gold prosey spec and reference implementation verify
      status: met
    - id: AC-2
      title: Clause-isolated mutant bank that the gold spec kills
      status: open
    - id: AC-3
      title: Each gold obligation probes as an isolable goal through the gate
      status: open
    - id: AC-4
      title: Pre-registration committed with a total falsifiable verdict map
      status: open
---
## Goal

Author the prosey-title subject — gold spec, reference implementation, and
clause-isolated mutant bank — and commit its pre-registration, so the two-arm
experiment can run on a boolean-predicate invariant with five isolable obligations,
the subtle one being the multi-sentence-boundary rule.

## Context

E-0002's second subject, parallel to M-0004. aiwf's `IsProseyTitle`
(`internal/entity/entity.go`) is a pure `string → bool` rejecting prosey/invalid
titles via five checks: over-length, embedded newline, markdown markers, link
brackets, and a multi-sentence boundary (sentence-mark + space + capital, with an
off-by-one rune window). It is the natural fit for the strength gate's single-input
predicate probe, and the multi-sentence rule is where a weak spec hides. Depends on
M-0003's generalized gate. (aiwf source at `/tmp/aiwf-src`.)

## Acceptance criteria

The four ACs are tracked in frontmatter `acs[]`; each criterion and its mechanical
evidence is detailed under its `### AC-N` section below.

## Constraints

- Single-input opaque predicate over a string; no collection or state.
- Clause-isolated mutants (G-0001): each breaks exactly one of the five checks.
- The pre-registration is committed before M-0006 is promoted to `in_progress`; its
  SHA will be asserted a git ancestor of the run commit in M-0006.

## Design notes

- Subject modeled from `IsProseyTitle` in `entity.go` — the five checks map to five
  obligations; the multi-sentence-boundary rule (threshold ≥ 1, off-by-one rune
  window) is the subtle obligation and the predicted tell.

## Surfaces touched

- A new subject directory under `experiments/loom-ultralight/` (gold `.dfy`,
  mutant bank, the pre-registration artifact).

## Out of scope

- Running the experiment (M-0006). The FSM subject (M-0004).

## Dependencies

- M-0003 (the generalized strength gate).

## References

- E-0002 epic spec; `/tmp/aiwf-src/internal/entity/entity.go` (`IsProseyTitle`); D-0001.

---

## Work log

All four ACs landed in one additive change-set (the feat commit `91faa23`); no new
production logic — the subject reuses M-0003's generalized gate (the epic's
"reuse, don't rebuild" constraint).

### AC-1 — Gold prosey spec and reference implementation verify
Gold `prosey.dfy` verifies (`5 verified, 0 errors`) · commit `91faa23` · `prosey_gold_verifies`.

### AC-2 — Clause-isolated mutant bank that the gold spec kills
Six clause-isolated mutants; gold kills 6/6; each pins to exactly one obligation ·
commit `91faa23` · `prosey_gold_kills_full_mutant_bank`, `prosey_mutants_are_clause_isolated`.

### AC-3 — Each gold obligation probes as an isolable goal through the gate
All six probe through the gate; positive-only refutes both multi-sentence
obligations · commit `91faa23` · `prosey_obligations_probe_and_discriminate`.

### AC-4 — Pre-registration committed with a total falsifiable verdict map
`prereg-prosey.md` with full obligation set, predicted tell, thresholds, total
verdict map · commit `91faa23`.

## Decisions made during implementation

- **Witness-vs-`forall` encoding.** Strings are an unbounded domain, so a naive
  `forall s :: <check>(s) ==> IsProsey(s)` goal pushes Z3 into sequence-quantifier
  reasoning that times out. Resolution: `over_length` stays a `forall` (its length
  branch short-circuits before any scan — decidable, and more faithful), while the
  four scan-based checks are probed with **minimal 3–6 char concrete witnesses** that
  Dafny ground-evaluates. Recursive string helpers carry `{:fuel 12, 12}` (enough to
  scan the short witnesses to the end). This keeps every gate probe decidable and
  holds the inconclusive rate low (G1).

## Validation

- `dafny verify prosey.dfy` → `5 verified, 0 errors`.
- `cargo test` (non-ignored) → **19 passed, 0 failed, 3 ignored**.
- `cargo test prosey_mutants_are_clause_isolated -- --ignored` → pass (6×6 sweep).
- `cargo build` → green. `cargo fmt --check` → only the 9 pre-existing drifts at
  lines < 640 (the M-0005 code is fmt-clean). `cargo clippy` → no new warnings (only
  the pre-existing line-438 complex-type one).

## Deferrals

- (none)

## Reviewer notes

Independent two-lens review before wrap: **code-quality (`wf-review-code`) → APPROVE**
— every load-bearing claim verified by running the tools, no blocking findings;
**design (`wf-rethink`) → SOUND**, no blocking defects. Findings applied as corrective
work before the AC promotions:

- `mlen.dfy`'s header comment now describes the `forall` over_length obligation (it
  referenced the superseded 81-char witness).
- **C1 / D2 seam guard added.** The obligation goals live in two sources — the gate's
  `PROSEY_SUBJECT` and the gold `.dfy`'s ensures (what the mutant bank calibrates
  against). `prosey_subject_goals_match_gold_ensures` (and `fsm_subject_goals_match_gold_ensures`
  for the sibling subject) pin the two sets equal, so editing a witness in one source
  without the other is now a build failure rather than silent drift.
- `prereg-prosey.md` §5 clarified that the decidability is engineered in the
  obligation **goals**, not the candidate spec the gate **assumes** — a thorough spec
  may still probe inconclusively, which the `inc > I` boundary absorbs (never folded
  into a verdict).

Deliberate, non-blocking limitations recorded (raised by the design review, accepted):

- **Single witness per scan-based trigger.** One concrete witness probes a point of
  the rule, not the whole rule, so a weak spec could entail it "by accident." This is
  a power cost vs the FSM subject's genuine `forall` goals, but it biases toward
  *not-reproduced* (a false negative cannot manufacture a false positive) — acceptable
  for a falsification design.
- **Easy-trigger disjunction gaps.** No obligation separately witnesses `\r`, `__`,
  backtick, or `?`/`!`; the easy triggers are controls measured *differentially*, so
  equal over-credit cancels in `easy_d − easy_i`.
- **Thin negative space vs FSM.** The only negative obligation is `ms_needs_capital`
  (the multi-sentence precision carrier); the subject deliberately targets the
  multi-sentence rule, and the differential framing tolerates an over-eager spec.
- **Faithful to the code, not the comment.** aiwf's `IsProseyTitle` returns
  `sentenceEndings >= 1` though its own doc-comment says "more than once"; the gold
  models the **code** (threshold ≥ 1), which is the correct ground truth — and makes
  the threshold a realistic place for a spec-writer to blur.

### AC-1 — Gold prosey spec and reference implementation verify

`prosey.dfy` carries the reference `IsProsey` (a faithful transcription of aiwf's
`IsProseyTitle`: empty → false, then an OR of over-length, newline, markdown marker,
link bracket, and the multi-sentence boundary) plus recursive string helpers, and a
`GoldSpec` lemma asserting all six gold obligations. `dafny verify prosey.dfy`
succeeds. The subject is the opaque predicate `IsProsey(s)` over a single string;
every check is a single-input obligation.

**Evidence:** `prosey_gold_verifies` (and direct `dafny verify prosey.dfy` →
`5 verified, 0 errors`).

### AC-2 — Clause-isolated mutant bank that the gold spec kills

`mutants-prosey/` holds six mutants (`mlen`, `mnl`, `mmd`, `mlink`, `mms_drop`,
`mms_nocap`), each breaking exactly one obligation — including both halves of the
multi-sentence tell (`mms_drop` drops the rule → `ms_present`; `mms_nocap` keeps a
boundary check but ignores the capital → `ms_needs_capital`). The gold kills the full
bank, and the isolation sweep pins each mutant to exactly its mapped obligation
(G-0001) with every obligation covered (G-0003).

**Evidence:** `prosey_gold_kills_full_mutant_bank` (6/6); `prosey_mutants_are_clause_isolated`
(asserts `broken == [want]` per mutant + full coverage).

### AC-3 — Each gold obligation probes as an isolable goal through the gate

Every obligation probes as an isolable single-input goal through the M-0003 gate
(`PROSEY_SUBJECT`). The full spec entails all six; the positive-only spec (the
predicted incentivized shape) entails the four easy triggers but the resolve-guarded
`refutes` confirms it entails **neither** multi-sentence obligation — the tell
discriminates the two specs.

**Evidence:** `prosey_obligations_probe_and_discriminate`. The encoding holds every
gate probe in Dafny's decidable ground-evaluation regime: `over_length` is a `forall`
(its length branch short-circuits, so it is decidable without an 81-char literal that
made Z3 time out), the other four are minimal 3–6 char witnesses.

### AC-4 — Pre-registration committed with a total falsifiable verdict map

`prereg-prosey.md` names the full obligation set, the predicted tell (the
multi-sentence rule weakens, localized — the easy triggers do not), the falsifying
outcomes, the strength thresholds (Δ⁺ = 0.20, Δ⁰ = 0.10, V = 10, I = 0.10 — shared
with the FSM subject so M-0007 combines them on one scale), and a **total**
function mapping every run observation into exactly one of reproduced /
not-reproduced / inconclusive (including the inconclusive boundary and the per-probe
inconclusive-denominator rule). No per-subject verdict judgment remains for after the
run. It lands before M-0006 is promoted to `in_progress`; its SHA will be asserted a
git ancestor of the run commit there.

**Evidence:** `prereg-prosey.md` committed in `91faa23`.

