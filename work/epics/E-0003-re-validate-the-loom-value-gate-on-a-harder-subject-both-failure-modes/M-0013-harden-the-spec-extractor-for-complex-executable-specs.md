---
id: M-0013
title: Harden the spec extractor for complex executable specs
status: in_progress
parent: E-0003
depends_on:
    - M-0010
    - M-0012
tdd: required
acs:
    - id: AC-1
      title: Spec extraction terminates at the lemma boundary
      status: open
      tdd_phase: red
    - id: AC-2
      title: Model-defined spec helpers are captured into the program
      status: open
      tdd_phase: red
    - id: AC-3
      title: Guarded id-quantifiers execute via a bounded rewrite
      status: open
      tdd_phase: red
    - id: AC-4
      title: Enriched battery rejects over-claims with no false-valids
      status: open
      tdd_phase: red
    - id: AC-5
      title: Calibration bounds the residual and confirms no arm bias
      status: open
      tdd_phase: red
---
## Goal

Harden the loom-ultralight spec extractor and program assembly so the `reallocate` over-claim
validity gate measures *correct, thorough* specs as valid — closing the three construct-validity
confounds (`G-0007`) the `M-0011` smoke surfaced beyond `G-0006`, before the recorded run.

## Context

`M-0011`'s N=1 smoke on the `M-0012` sound gate returned 3/6 `unexecutable` — correct
disinterested specs marked invalid — from three distinct causes (`G-0007`): (1) the
`ensures`-extractor overruns the lemma into trailing prose when the body brace is not at
line-start; (2) model-defined helper functions are not included in the assembled program; (3)
unbounded guarded id-quantifiers (`forall x: Id :: HasId(t, x) ==> …`) cannot be executed by the
Go backend. Thorough disinterested specs use exactly these forms, so the instrument
systematically penalizes the disinterested arm and confounds the pre-registered over-claim
comparison. This milestone is a **pre-run instrument bug-fix** (like `M-0012`): no recorded run
has happened; the §6 procedure / thresholds / predictions (`prereg-reallocate.md`, `bb1d220`)
are untouched. Each fixture below is the *actual* smoke spec, committed (its `runs/` source is
gitignored) so the regressions are reproducible offline.

## Acceptance criteria

### AC-1 — Spec extraction terminates at the lemma boundary

`extract_spec_ensures` terminates the `ensures` region at the lemma body — a trimmed line
starting with `{`, `}`, or ` ``` ` — so a lemma whose body brace is not at line-start no longer
captures the closing code fence and the prose that follows.

**Evidence (mechanical).** A regression test pins the committed `opus-4.8` disinterested smoke
fixture (lemma closed with a bare `}`): extraction returns ONLY the `ensures` clauses, and the
spec then validates via the execution fallback (`ExecValid`) instead of `Unexecutable`. Plus a
unit test of the new terminators on a minimal fixture.

### AC-2 — Model-defined spec helpers are captured into the program

The spec-block `function`/`predicate` definitions a model adds (other than `lemma Spec` and any
`Reallocate` it restates) are included in the assembled `.dfy`, so an `ensures` that calls a
helper resolves instead of erroring.

**Evidence (mechanical).** A regression test pins the committed `haiku-4.5` incentivized smoke
fixture (defines + calls `IndexOfId`): the helper is captured and the spec classifies as a
decided validity verdict (not `Unexecutable` via resolution error).

### AC-3 — Guarded id-quantifiers execute via a bounded rewrite

A guarded unbounded id-quantifier — `forall <x> :: [<other guards> &&] HasId(<tree>, <x>) ==>
<body>` — is rewritten to bounded iteration over the tree's entities
(`forall i :: 0 <= i < |<tree>| [&& <other guards>[x:=tree[i].id]] ==> <body>[x:=tree[i].id]`),
which is a sound equivalence (`HasId(tree, x)` iff `x` is some `tree[i].id`), so correct specs
that quantify over present ids execute.

**Evidence (mechanical).** A regression test pins the committed `sonnet-4.6` disinterested smoke
fixture (clauses 8–9 quantify `forall x :: … HasId(t, x) …`): it validates via the rewrite
(`ExecValid`) instead of `Unexecutable`. A non-guarded unbounded quantifier (no `HasId` bound)
stays `Unexecutable` — the genuine residual, still surfaced. If the rewrite proves intractable
or unsound for a needed pattern, this AC is met instead by a recorded decision (`aiwfx-record-decision`)
to accept-and-surface the residual, with the fixture pinned as the documented boundary.

## Constraints

- **Pre-registration preserved.** §6 procedure, thresholds, combination rule, predictions
  (`bb1d220`) untouched; no edit to `prereg-reallocate.md`. The fixes change how validity is
  *decided*, never the frozen `1 − valid/extracted` formula.
- **Soundness over coverage.** A rewrite/capture must never make an over-claiming spec validate
  (no new false-VALIDs). The guard rewrite is an exact equivalence; helper capture only adds the
  model's own definitions. When in doubt, leave a spec `Unexecutable` (surfaced) rather than risk
  a false valid.
- **TDD required; zero warnings** (`clippy -D warnings`, `fmt --check`); determinism (G1).

## Design notes

- The three fixes are independent and layer onto the `M-0012` gate: extraction feeds assembly
  feeds `validate_spec`. Keep each fix local and unit-tested against its committed fixture.
- The guard rewrite is a targeted transform on the specific `HasId(tree, x) ==>` shape, applied
  to the extracted `ensures` before `ensures_to_conjunction`. Variable substitution is scoped to
  the bound variable; bail (leave unrewritten) on any shape it does not recognize.
- Helper capture parses top-level `function`/`predicate` decls from the spec code block; exclude
  anything that would redefine a preamble symbol or the reference `Reallocate`.

## Out of scope

- The reallocate run itself + the terminal decision — `M-0011` (resumes after this; a re-smoke
  characterizes the residual as part of M-0011's run-readiness).
- Re-running or re-scoring E-0002 subjects.
- Changing the §6 procedure / thresholds / prereg.

## Dependencies

- Depends on `M-0010` (the instrument + frozen §6) and `M-0012` (the hybrid gate it extends).
- Addresses `G-0007`. **Blocks `M-0011`'s recorded run** — the run resumes once the instrument
  measures correct complex specs.

### AC-4 — Enriched battery rejects over-claims with no false-valids

### AC-5 — Calibration bounds the residual and confirms no arm bias

