---
id: M-0015
title: Turn the loop on real string-based canonicalization
status: in_progress
parent: E-0004
depends_on:
    - M-0014
tdd: advisory
acs:
    - id: AC-1
      title: Canonicalize umbrella authored under the burden split
      status: open
    - id: AC-2
      title: Real string canonicalize modeled and cross-checked
      status: open
    - id: AC-3
      title: Loop turned across the ladder; tractability recorded
      status: open
---
## Goal

Turn the whole umbrella loop on the **real, string-based** `Canonicalize(id string) string`
(`internal/entity/canonicalize.go` @ v0.20.0), **laddered** — single format → six formats +
per-kind widths → composite recursion — to find **where Dafny/Z3's tractability breaks on real
string-heavy code**, or that it holds. This is `E-0004`'s load-bearing question: the reviewers'
biggest untested unknown.

## Context

`M-0014` turned the loop on the *easy* end (a discrete FSM table Dafny verified instantly). This
milestone deliberately points at the hard end: `Canonicalize` parses a string id, zero-pads the
numeric part to a per-kind minimum width, reassembles the string, and **recurses** for composite
ids (`M-1/AC-2`). Strings are exactly where SMT solvers struggle. We go **string-level** (model
`Canonicalize` as a real string→string function), **not** the structured `Id = (kind, value,
width)` abstraction (`E-0001`) — that abstraction dodges the string layer, which is the thing under
test. Reference: [`docs/loom-loop-poc.md`](../../../docs/loom-loop-poc.md); umbrella convention as
established in `M-0014`.

## Acceptance criteria

### AC-1 — Canonicalize umbrella authored under the burden split

The umbrella follows the five-register `.lm` convention (`knows`/`relates`/`shows`/`does`/`proves`/
`gap`): **Intent + `shows`** by the human; **`proves` + back-translation** and the **`does`** model
by blind subagents (neither seeing the other). The human never authors the formal section.

**Evidence.** The committed umbrella artifact in the register shape; claims consistent with the
examples.

### AC-2 — Real string canonicalize modeled and cross-checked

The impl-modeler produces a Dafny `does` model of the real string-level canonicalize logic for the
rung under test — **or**, if faithfully modeling string parsing in Dafny is itself infeasible,
that barrier is characterized precisely (it is a first-class result). Where a model exists it is
**cross-checked against the real Go** on the examples (same in→out).

**Evidence.** The committed model (or the characterized modeling barrier) + the Go cross-check
result on the rung's examples.

### AC-3 — Loop turned across the ladder; tractability recorded

The loop is turned rung by rung — (1) single format, no composite (e.g. `E-7 → E-0007`); (2) six
formats + per-kind widths; (3) composite recursion — stopping at the first rung that breaks. For
each rung attempted, the gap report **and the tractability verdict** (verified / category-(B)
timeouts / could-not-model, with *where* it broke) are recorded.

**Evidence.** The committed per-rung gap report(s) and the recorded tractability verdict naming the
rung and cause where verification (or modeling) breaks, or that it held through rung 3.

### AC-4 — The four observations are recorded, with tractability as the headline

The four observations (tractability, faithfulness, value, effort) are written up. **Tractability is
the headline finding** — the precise point where Dafny/Z3 (or the modeling step) stops coping, on
real string code.

**Evidence.** A committed observations note; tractability stated as a precise wall location (or
"held through composite recursion").

## Constraints

- **Real code, string-level.** Model the actual `Canonicalize` string→string logic; the structured
  `(kind, value, width)` abstraction is **out** (it dodges the test).
- **Ladder one source of complexity at a time.** Do not jump to composite recursion before the
  single-format string-padding rung is understood. Stop at the first break — that *is* the result.
- **The modeling barrier is a first-class result.** If Dafny can't faithfully model the string
  logic at all, record that as the finding; do not fall back to the structured abstraction to force
  a green.
- **The human never authors the formal section**; if steering required reading Dafny, AC-4 records
  it.
- **Feasibility, not a metric.** No pass/fail threshold; a wall found is a success of the
  experiment, not a failure. `tdd: advisory`; interactive, no metered API (blind in-session
  subagents; local Dafny + Go cross-check).

## Design notes

- Loop shape + burden split + umbrella convention: inherited from `M-0014` / `docs/loom-loop-poc.md`.
- The ladder is *within* this loop (rungs 1→3), each adding one source of string complexity.
- Expect the wall. A precise "it breaks at X" (string padding / multi-format / recursion / or even
  the modeling step) is the deliverable — more valuable than a green on a dodged abstraction.

## Out of scope

- The structured `Id`-abstraction model (`E-0001`'s approach) — explicitly excluded.
- Modeling `IDGrepAlternation` / regexp construction — out; the target is `Canonicalize`.
- Building the tool, other loops, the epic's terminal decision.

## Dependencies

- Depends on `M-0014` (the loop mechanics + umbrella convention it inherits). Reference:
  [`docs/loom-loop-poc.md`](../../../docs/loom-loop-poc.md).
