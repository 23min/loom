---
id: M-0009
title: Design the id-reallocation subject
status: in_progress
parent: E-0003
depends_on:
    - M-0008
tdd: required
acs:
    - id: AC-1
      title: Gold spec verifies against the reference impl within timeout
      status: met
      tdd_phase: done
    - id: AC-2
      title: Obligation set is pinned to the gold ensures and ranks a weaker spec lower
      status: met
      tdd_phase: done
    - id: AC-3
      title: Mutant bank is clause-isolated and fully killed by the gold spec
      status: met
      tdd_phase: done
    - id: AC-4
      title: Over-claim fixture is caught by the validity gate
      status: met
      tdd_phase: done
    - id: AC-5
      title: Reallocation subject registered and calibrates green end-to-end
      status: met
      tdd_phase: done
---
## Goal

Build and *calibrate* a new loom subject — the `Reallocate(tree, oldId, newId)`
invariant — as the instrument E-0003's two-dimension study will run on: a
reference Dafny implementation, a complete gold spec, an obligation set pinned to
that spec, a clause-isolated mutant bank, and an over-claim fixture, all wired
into the `LOOM_SUBJECT` registry and green under `--calibrate`. This milestone
produces the **instrument**, not the experiment — no pre-registration and no
two-arm run.

## Context

E-0002 falsified the under-specification tell on the FSM and prosey subjects but
surfaced **over-claiming** as a live, unscored lead. E-0003 re-tests both failure
modes on a *genuinely harder* invariant; `M-0008` already hardened the harness
(single-source validity gate, injectable probe routing, self-contained
`verdict.json`). What remains before any run is the subject itself.

The chosen invariant (`D-0002`'s "genuinely more complex subject", resolved to
id-reallocation in this epic's subject-choice decision) is a **self-contained
Dafny model** of `aiwf reallocate`'s contract — rename an entity id and rewrite
every cross-reference to it, preserving uniqueness and leaving no orphan. Like
every loom subject (`canonicalize`, `fsm`, `prosey`), it is an *idealization* of
the invariant's shape, not a binding to the real Go tool: the experiment measures
the Dafny spec an LLM writes, so the subject must be something Z3 can verify and
whose gold contract we own. Its *shape* is lifted from aiwf's real reallocate
semantics, which is what makes it a relevant, richer subject than the FSM (a
complete pointwise pin over a tree — rename, frame, and a complete reference
rewrite — versus a flat edge set), and naturally over-claim-prone, exercising
E-0003's second dimension.

The subject plugs into the existing per-invariant surface (`main.rs`): a gold
`.dfy` carrying the `BEGIN/END PREAMBLE | REFERENCE IMPL | GOLD SPEC ENSURES`
sentinels (`main.rs:435-449`), a `StrengthSubject` (opaque decls + binder +
requires + obligations, `main.rs:919-930`) whose goal strings are pinned equal to
the gold ensures by a C1/D2 seam guard (the `fsm_subject_goals_match_gold_ensures`
pattern, `main.rs:990-991`), a clause-isolated mutant bank (the `FSM_MUTANTS` /
`PROSEY_MUTANTS` shape, `main.rs:1088-1091`), and a `Subject` registry entry
(`main.rs:1098-1171`). `--calibrate` (`main.rs:534-581`) is the end-to-end gate:
the gold spec must be **valid** against the reference impl and **kill the full
bank cleanly** (`killed == bank && survived == 0 && inconclusive == 0`).

## Acceptance criteria

### AC-1 — Gold spec verifies against the reference impl within timeout

A reference Dafny implementation of `Reallocate` and a complete gold ensures
block exist in `reallocate.dfy` (the three sentinel sections), and the reference
impl **verifies** against the gold spec — `score_spec(...).valid` is true — within
`LOOM_DAFNY_TIMEOUT` (default 30s). This is the **tractability gate**: the gold
contract quantifies over the tree's entities and their reference sequences
(nested bounded `forall`), and this AC proves Z3 discharges it inside the budget
before any mutant bank or run is built. It is therefore also the honest go/no-go
on the subject: if the quantified frame conditions cannot be made to verify within
timeout after reasonable massaging, this AC fails loudly here, not mid-run.

**Evidence (mechanical).** A test invokes the validity gate (`validate_spec` /
`score_spec`) on the reference impl against the gold ensures under the harness's
own Dafny invocation and asserts `valid == true` within the timeout. The test
fails if the gold contract regresses to unverifiable or the timeout is exceeded.

### AC-2 — Obligation set is pinned to the gold ensures and ranks a weaker spec lower

The `StrengthSubject` obligation goals are pinned **equal** to `reallocate.dfy`'s
`GOLD SPEC ENSURES` block by a seam guard (the C1/D2 single-source pattern), so
the strength probe and the gold spec can never drift. The obligation set is the
three **independent** clauses of the complete pointwise pin, each a single goal —
the renamed entity becomes `newId` (`R`), every other id is unchanged (`F`, the
frame), and every cross-reference is rewritten (`C`, the tell) — with no redundant
or derived clause (the structural invariants no-orphan and uniqueness are
*entailed* by the pin, so they are proven as consequence lemmas, not scored). The
measure discriminates: the gold spec entails all three; a hand-weakened spec that
pins the two controls (`R`, `F`) but drops the reference-rewrite tell entails
strictly fewer.

**Evidence (mechanical).** A `reallocate_subject_goals_match_gold_ensures` test
pins every obligation goal to the gold `.dfy` ensures text; a
`reallocate_strength_ranks_weaker_spec_lower` test shows the gold spec entails all
three obligations while the reference-rewrite-dropping spec entails the two
controls but is refuted on the tell. The test fails if an obligation goal diverges
from the gold ensures or the measure stops ranking a weaker spec lower.

### AC-3 — Mutant bank is clause-isolated and fully killed by the gold spec

A mutant bank (`mutants-reallocate/`, listed in report order in a
`REALLOCATE_MUTANTS` const) carries **a mutant per obligation clause**, plus a
sharper second tell-violator — each a reference impl wrong in exactly one way:
`m_leave_old` keeps the old id instead of renaming (breaks `R`); `m_collapse_ids`
maps every id to `newId`, clobbering the frame (breaks `F`); `m_keep_refs` leaves
all references un-rewritten and `m_partial_refs` rewrites only the renamed entity's
refs, forgetting the distant cross-references (both break `C`). The gold spec kills
the whole bank cleanly; each clause is load-bearing — the mutant for clause *k*
survives a spec with clause *k* removed, so no mutant is redundant and no clause is
dead weight.

**Evidence (mechanical).** `--calibrate` reports `killed == bank, survived 0,
inconclusive 0`; per-clause `reallocate_*` tests assert that removing clause *k*
from the spec lets mutant *k* survive (the clause-isolation property the
`fsm_*` / `prosey_*` calibration tests pin for the E-0002 banks).

### AC-4 — Over-claim fixture is caught by the validity gate

At least one **over-claim** spec — an ensures block too strong for even the
correct reference impl (here: asserting references are *unchanged*, the exact
opposite of the rewrite the correct impl performs) — is committed as a calibration
fixture, and the
single-source validity gate (`validate_spec`, `M-0008` AC-1) catches it: the
reference impl fails to verify against it, so the harness counts it **invalid**
and excludes it from the strength population. This proves the over-claiming
failure mode is *detectable* on this subject before the pre-registration milestone
scores it.

**Evidence (mechanical).** A test feeds the over-claim fixture through
`validate_spec` and asserts a non-`Verified` outcome counted `invalid` (mirroring
`probe_spec_excludes_overclaim_invalid_specs` from `M-0008`). The test fails if an
over-claim slips through the validity gate.

### AC-5 — Reallocation subject registered and calibrates green end-to-end

The reallocation subject is wired into the `SUBJECTS` registry alongside
`canonicalize` / `fsm` / `prosey` — gold file, mutants dir + bank, `impl_signature`,
`intent_file` (authored here, exercised at run time), `StrengthSubject`, and the
`tell_keys` / `easy_keys` §6 partition (the reference rewrite as the tell, the id
map — rename + frame — as the control). `LOOM_SUBJECT=reallocate --calibrate`
passes end-to-end (validity + clean full-bank kill, subsuming AC-1 and AC-3 over
the live registry path), and a golden calibration fixture is committed.

**Evidence (mechanical).** A test selects the subject by name and asserts the
`--calibrate` outcome (valid gold, full clean kill, zero inconclusive) over the
registered subject; `subject_by_name("reallocate")` resolves and the keys-unique
debug-assert holds. The canonicalize / fsm / prosey rows and the committed golden
fixtures are untouched.

## Constraints

- **Stays in Z3's decidable regime.** The model uses finite `seq`/`datatype`
  domains and bounded quantifiers (`forall i | 0 <= i < |s| :: …`) only — no
  unbounded quantification over infinite domains, no undecidable theory. AC-1 is
  the mechanical proof the budget holds; the 30s `LOOM_DAFNY_TIMEOUT` is the line.
- **No regression to existing subjects or frozen results.** `canonicalize`,
  `fsm`, `prosey`, their goldens, and E-0002's frozen §6 verdict map and oracle
  tests are untouched. The new subject is additive — a new registry entry, a new
  gold `.dfy`, a new mutant dir; no field removed or repurposed (B2 additive).
- **TDD required** — every AC red → green → refactor, with the branch-coverage
  audit on the diff before any AC flips to `met`.
- **Zero warnings** — `cargo clippy -- -D warnings` clean, `cargo fmt --check`
  clean.

## Design notes

- **The model (settled in shape, exact Dafny pinned in implementation).** `type
  Id` (a finite-domain identifier), `datatype Entity = Entity(id: Id, refs:
  seq<Id>)`, `type Tree = seq<Entity>`, `predicate Valid(t)` (no duplicate ids).
  `function Reallocate(t, oldId, newId): Tree` renames the `oldId` entity to
  `newId` and rewrites every `refs` entry `oldId → newId`. `new`/`old` are avoided
  as identifiers (`new` is a Dafny keyword) — hence `oldId` / `newId`.
- **The gold ensures → obligations decomposition.** Three **independent** Single
  obligations forming the complete pointwise pin: `result[i].id == newId` for the
  target (`R`), `result[i].id == t[i].id` for every other entity (`F`, the frame),
  and `result[i].refs == RwRefs(t[i].refs, …)` for all (`C`, the tell). No ladder:
  reallocation has no natural *graded* axis, and a forced one would leave the seam
  guard or the mutant bank unclean (see *Decisions made during implementation*). The
  two structural invariants — no orphaned `oldId`, preserved uniqueness — are
  *entailed* by `{R, F}`, so they are proven as consequence lemmas in
  `reallocate.dfy` (`StructuralInvariantsFollow`) but deliberately not sliced —
  stating them alongside the pin would make them redundant.
- **One source of truth for the contract.** The gold `.dfy` ensures is the single
  owner; the `StrengthSubject` goals and the kill-rate lemma both derive from it,
  pinned by the AC-2 seam guard (the `{fsm,prosey}_subject_goals_match_gold_ensures`
  pattern). The strength probe states obligations against an `{:opaque} Reallocate`
  so an entailment holds for *any* implementation (`main.rs:915-918`).
- **Opaque length exposure.** The strength probe's `{:opaque} Reallocate` exposes
  `|result| == |t|` (length, not contents) so the per-entity `R`/`F`/`C` clauses are
  well-formed against the hidden body. Length-preservation is not an obligation, so
  it never leaks into the `{R, F, C}` measure.

## Out of scope

- The **two-dimension pre-registration** (the §6 verdict map scoring
  under-specification *and* over-claiming, thresholds, and the combination rule) —
  the next E-0003 milestone, under its own prereg whose SHA must ancestor the run.
  This milestone names the construct-validity caveat for that prereg to scope: the
  subject is a model of the invariant, not the live tool, and the instrument
  observes under-specification only along the obligation axes it pins (`R`, `F`,
  `C`) — any claim must scope to those, not to "reallocate specs" in general. It
  does not author the prereg.
- The **two-arm run and the terminal decision** — the milestone after the prereg.
- Any change to `canonicalize` / `fsm` / `prosey`, their goldens, or E-0002's
  frozen results.

## Dependencies

- Depends on `M-0008` (the hardened harness: single-source validity gate,
  injectable probe routing, self-contained `verdict.json`) — the instrument this
  subject plugs into.
- Builds on the E-0002 harness generalization (the `LOOM_SUBJECT` registry, the
  sentinel-delimited gold `.dfy`, the structural strength gate).
- Blocks the pre-registration and run milestones — neither can proceed until this
  calibrated instrument exists.

## Decisions made during implementation

- **Independent Single obligations, not a graded ladder.** AC-2 was authored
  describing a frame-completeness *ladder*. The harness's ladder mechanism does
  exist for nested obligations (canonicalize's width axis), but reallocation has no
  natural *graded* axis, and its candidate clauses have logical dependencies that
  make a clause-isolated mutant bank (AC-3's load-bearing requirement: a mutant
  violating exactly one clause) clean only for a **mutually independent** set. So the
  obligations are independent Singles (like the FSM), each with a clean isolated
  mutant — more faithful than a forced ladder, and the C1 single-source seam stays
  clean.
- **The gold is the complete pin `{R, F, C}`, not the structural invariants
  `{O, U, C}`.** The first cut used the three structural invariants — no-orphan
  (`O`), uniqueness (`U`), complete rewrite (`C`). Independent review (the design
  lens) caught that `{O, U, C}` is *incomplete*: it never pins that the renamed
  entity becomes `newId`, so an impl that renames the target to a *wrong* fresh id
  while rewriting refs to `newId` satisfies all three yet leaves a dangling
  reference — and, worse for the experiment, a faithful spec that *does* pin the
  rename would score identically to one that omits it (the G-0002
  richer-spec-invisibility pattern, structural this time). The fix: the gold is the
  **complete pointwise pin** — target renamed (`R`), frame preserved (`F`),
  references rewritten (`C`). The same three mutants re-attribute cleanly
  (`m_leave_old`→`R`, `m_collapse_ids`→`F`, `m_keep_refs`→`C`), a fourth
  (`m_partial_refs`, the realistic forgot-the-distant-refs case) sharpens the `C`
  tell, and `{O, U}` survive as proven consequence lemmas. The over-claim fixture
  ("references unchanged") is the exact mirror of the tell (`C`). Recorded here
  rather than as a separate decision entity — a within-milestone refinement
  (including the review correction), not an architectural change.

## Work log

- **AC-1** — `reallocate.dfy` gold verifies against the reference impl in ~2s
  (budget 30s); pinned by `reallocate_gold_spec_is_valid_against_reference_impl`.
  Tractability retired emphatically — the quantified tree-rewrite contract is well
  inside Z3's decidable regime.
- **AC-2** — obligation goals pinned to the gold ensures
  (`reallocate_subject_goals_match_gold_ensures`); the measure ranks a spec dropping
  the tell — or the rename — strictly lower (`reallocate_strength_ranks_weaker_spec_lower`).
- **AC-3** — 4-mutant clause-isolated bank, gold kills all cleanly; each clause
  load-bearing (`reallocate_gold_calibrates_clean`, `reallocate_mutants_are_clause_isolated`).
- **AC-4** — the "references unchanged" over-claim is caught by the validity gate
  (`reallocate_over_claim_is_caught_by_validity_gate`).
- **AC-5** — `reallocate` registered in `SUBJECTS`; `LOOM_SUBJECT=reallocate
  --calibrate` green; covered through the production path by
  `production_scorer_calibrates_every_subject`.

The implementation landed in the single wrap commit; the spec, AC phases, and
milestone status are their own trailered aiwf commits. Phase timeline in
`aiwf history M-0009/AC-<N>`.

## Validation

- `LOOM_SUBJECT=reallocate --calibrate`: gold valid, **4/4 killed**, 0 survived, 0
  inconclusive.
- `dafny verify reallocate.dfy`: 7 verified, 0 errors (~2s).
- `cargo test` (full non-ignored): **37 passed, 0 failed**; reallocate suite 6/6.
- `cargo test production_scorer_calibrates_every_subject --ignored`: every
  registered subject (incl. reallocate) calibrates clean via the production path.
- `cargo clippy --all-targets -- -D warnings`: clean. `cargo fmt --check`: clean.
- No regression: canonicalize / fsm / prosey rows and goldens untouched (the diff
  is additive — a new registry entry, gold `.dfy`, and mutant dir).

## Reviewer notes

- **Two-lens independent review** (fresh-context subagents). Code-quality:
  **APPROVE**, verified by perturbation (mutating the gold / each mutant drives the
  expected test red). Design: initially **CONCERNS** — the first-cut gold
  `{O, U, C}` omitted reallocation's defining property (rename → `newId`),
  permitting a wrong-id impl and risking the G-0002 richer-spec-invisibility pattern.
- **Resolved in this milestone, not deferred:** switched the gold to the complete
  pointwise pin `{R, F, C}`. Same three mutants re-attribute; a fourth
  (`m_partial_refs`, the realistic forgot-the-distant-refs case) sharpens the `C`
  tell; `{O, U}` survive as proven consequence lemmas. A focused re-review confirmed
  **RESOLVED** — the gold is now complete, the pathology rejected, and dropping
  `{O, U}` as scored obligations introduces no new blind spot (they are strict
  consequences; scoring them would dilute the tell).
- **Deliberate trade-offs:** all-Single obligations, no ladder (reallocation has no
  natural graded axis); the opaque strength-probe `Reallocate` exposes
  `|result| == |t|` (length, not contents) for well-formedness, leaking nothing into
  the measure; `{O, U}` kept as consequence lemmas rather than redundant obligations.

## Deferrals

None. The construct-validity caveat and the two-dimension pre-registration are owned
by the next E-0003 milestone (already planned), not deferred work from this one.
