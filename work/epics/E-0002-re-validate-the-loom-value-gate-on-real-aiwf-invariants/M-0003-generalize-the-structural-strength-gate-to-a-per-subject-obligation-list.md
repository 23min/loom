---
id: M-0003
title: Generalize the structural strength gate to a per-subject obligation list
status: in_progress
parent: E-0002
tdd: advisory
acs:
    - id: AC-1
      title: Strength gate driven by per-subject obligation spec
      status: open
    - id: AC-2
      title: Canonicalize N=30 strength matches golden fixture
      status: open
---
## Goal

Generalize loom-ultralight's structural strength gate (`--strength`) from the
hardcoded id-canonicalization obligations to a **per-subject obligation list** over
an opaque function/predicate — the one new component E-0002 requires — without
changing any verdict on the existing subject.

## Context

The strength gate (`experiments/loom-ultralight/src/main.rs`, the
`strength` / `entails` / `assemble_strength` functions) currently hardcodes the
four canonicalize obligations (K/V/W with a two-rung width ladder, F). E-0002
re-validates the spec-weakening effect on two new subjects whose obligations
differ in shape, so the gate must accept an arbitrary per-subject obligation set.
This milestone is foundational: M-0004 and M-0005 author subjects against this
generalized interface and use it to confirm their obligations are isolable
single-input goals before pre-registering.

## Acceptance criteria

Tracked as `acs[]` in frontmatter (AC-1, AC-2); the full obligation detail lives
under the per-AC sections at the foot of this spec.

## Constraints

- Behavior-preserving on the existing subject — the canonicalize strength verdicts
  must not change (the regression AC is the guard).
- Implementation-independent: obligations are probed against an opaque
  function/predicate (`function {:opaque} F`), never a concrete implementation.
- The killed / survived / inconclusive trichotomy and the probe-error guard carry
  over unchanged.

## Design notes

- Build on the existing `assemble_strength` / `entails` / `strength` functions;
  replace the hardcoded `STRENGTH_GOALS` constant and obligation classification
  with a per-subject obligation spec passed in.
- No arbitrary Dafny-spec parsing — the obligation list is authored per subject (an
  explicit spec object), not inferred from the candidate.

## Surfaces touched

- `experiments/loom-ultralight/src/main.rs` (the strength-gate functions).

## Out of scope

- Authoring the new subjects (M-0004, M-0005).
- Any change to the mutation kill-rate path or the API/run path.

## Dependencies

- None — foundational. Reuses the cached canonicalize N=30 run for the regression.

## References

- E-0002 epic spec; D-0001; `experiments/loom-ultralight/results/RESULTS.md`.

---

## Work log

### AC-1 — Strength gate driven by per-subject obligation spec

Introduced `Obligation` (`Single` / `Ladder`) + `StrengthSubject`; recovered the
existing gate as the `CANONICALIZE` subject; generalized `assemble_strength`,
`classify_ladder`, `probe_spec`, `compute_strength`, and `strength_rows_json` to be
subject-driven — no canonicalize obligation remains hardcoded in the strength path.
Three real-Dafny shape fixtures (exclusion `!P` over a ground tuple, bounded `∀`
over a finite datatype, unary opaque predicate over a single value) each prove the
interface entails a *pinned* obligation and does **not** entail an unpinned one, so
the gate is shown to discriminate. Pure `classify_ladder` / `keys()` /
byte-identical-source tests, plus fast `probe_spec` / `compute_strength` tests,
cover every branch. tests 11/11 (1 ignored). Landed in `0cdd036`.

### AC-2 — Canonicalize N=30 strength matches golden fixture

Froze the 178-file N=30 generation corpus under `tests/fixtures/strength-n30/`
(the live `runs/` dir is gitignored) so the regression is reproducible from a clean
clone (G1). `golden_canonicalize_n30_strength_is_reproduced` re-runs the
generalized gate over the corpus and diffs the serialized result against the
committed `results/strength-n30.json`; any changed verdict fails. Marked `#[ignore]`
(full N=30 Dafny sweep, hundreds of probes); behavior-equality is additionally
pinned fast by the byte-identical probe-source test. Landed in `0cdd036`.

## Decisions made during implementation

- (none)

## Validation

- `cargo test` — 11 passed, 1 ignored (~40s). Covers every branch in the
  generalized strength path: ladder rung selection (exact / bound-only / free) and
  short-circuit, the probe-error guard, the missing-file and unextractable-response
  skips, and zero-default serialization — plus the three new obligation shapes via
  real Dafny.
- `cargo test -- --ignored golden_canonicalize_n30_strength_is_reproduced` — passes
  (1/1, ~31 min): reproduces the committed golden fixture byte-for-byte from the
  frozen N=30 corpus (AC-2). Independently corroborated — the pre-generalization
  binary reproduces the same golden from the same corpus.
- Behavior preservation: `canonicalize_probe_source_is_byte_identical` asserts the
  generalized probe source equals the pre-generalization template verbatim, so no
  canonicalize verdict can change.
- `cargo clippy` / `cargo fmt --check` — the diff adds no new warnings or
  formatting drift. (Pre-existing drift in the unrelated API-run/mutant path is left
  untouched — see Reviewer notes.)

## Deferrals

- (none)

## Reviewer notes

- **Independent two-lens review (wrap step 2):** code-quality (`wf-review-code`) →
  **approve** — all five load-bearing claims verified by running, including the
  shape negatives confirmed to fail at Dafny's *verification* stage (genuine
  non-entailment), not the resolve stage. Design (`wf-rethink`) → **keep-as-is**.
  Two non-blocking items were applied in-milestone: (a) the negative shape fixtures
  now resolve-guard via a `refutes(...)` helper, so a future typo can't masquerade
  as non-entailment; (b) a `StrengthSubject::keys()` helper DRYs the
  column-deriving sites and `debug_assert!`s key uniqueness — closing the one latent
  footgun for M-0004/M-0005 subject authors (a reused JSON key would otherwise
  silently collapse a column).
- The `--strength` CLI path now selects `CANONICALIZE` internally; M-0004/M-0005
  author their subjects against the same `StrengthSubject` interface — no further
  gate changes are expected for them.
- **Stdout-only behavior change:** `print_strength_table` moved from the
  canonicalize-specific `K%/V%/F%` percentage table to a generic per-key count
  table (it must work for any subject). The durable record — `strength.json` — is
  unchanged, and AC-2 pins it.
- **Pre-existing, out of scope, untouched:** one clippy `complex type` warning on
  `score_trials` (the API-run path, `src/main.rs:438`) and 9 rustfmt drifts in the
  mutant/run code predate this milestone. A separate `cargo fmt` + type-alias chore
  would clear them; left out of M-0003's diff per minimal-change.

### AC-1 — Strength gate driven by per-subject obligation spec

Given a per-subject obligation spec over a named opaque function/predicate, the
gate emits per-obligation **exact/bound/free verdicts driven entirely by that
spec**, with no canonicalize obligation hardcoded in the strength path. Exercised
by minimal fixtures covering the obligation **shapes the new subjects need**: at
least an **exclusion goal** (`!P` over a ground tuple), a **bounded quantifier
over a finite datatype** (the FSM shapes), and a **unary opaque predicate over a
single value** (the prosey shape) — proving the interface is general beyond the
canonicalize unary-function shape.

### AC-2 — Canonicalize N=30 strength matches golden fixture

Re-running the generalized gate on the cached canonicalize N=30 generations
reproduces a **committed golden strength fixture** (the per-condition K/V/F
entailment counts and the width exact/bound/free distribution), diffed
mechanically; any changed verdict fails this AC.

