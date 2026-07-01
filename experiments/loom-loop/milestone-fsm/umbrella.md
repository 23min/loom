# Umbrella — milestone status-transition FSM

Loom loop 1 (`M-0014`, `E-0004`). The umbrella's sections, authored under the **burden split**:
Intent + Examples by the human; Claims + back-translation by a blind umbrella-author subagent;
Model by a blind impl-modeler subagent (from the real aiwf `transition.go` @ v0.20.0). Neither
subagent saw the other's output — so the gap report (`gap-report.md`) is a genuine confrontation.

## § Intent  *(human)*

A milestone is in one of four statuses — `draft`, `in_progress`, `done`, `cancelled` — starting at
`draft`. It may go `draft → in_progress` only if it has at least one acceptance criterion (no ACs ⇒
cannot start). It may go `in_progress → done` only if all of its ACs are met (any unmet ⇒ cannot
finish). It may be cancelled — including with no ACs, and with ACs unmet. `done` and `cancelled`
are terminal.

## § Examples  *(human — the concrete anchor)*

| from | to | condition | expected |
|---|---|---|---|
| draft | in_progress | has ACs | allowed |
| draft | in_progress | no ACs | **denied** |
| draft | cancelled | no ACs | allowed |
| in_progress | done | all ACs met | allowed |
| in_progress | done | an AC unmet | denied |
| in_progress | cancelled | an AC unmet | allowed |
| done | in_progress | — | denied (terminal) |
| cancelled | in_progress | — | denied (terminal) |

## § Shared interface  *(the vocabulary both blind authors bound to)*

```dafny
datatype Status = Draft | InProgress | Done | Cancelled
// hasAC     — the milestone has at least one acceptance criterion
// allACsMet — every one of its acceptance criteria is met
predicate Allowed(from: Status, to: Status, hasAC: bool, allACsMet: bool)
```

## § Claims  *(LLM umbrella-author — blind to the code)*

```dafny
// C1: no ACs => can't start
ensures forall m :: !Allowed(Draft, InProgress, false, m)
// C2: not all met => can't finish
ensures forall h :: !Allowed(InProgress, Done, h, false)
// C3: done terminal
ensures forall t, h, m :: !Allowed(Done, t, h, m)
// C4: cancelled terminal
ensures forall t, h, m :: !Allowed(Cancelled, t, h, m)
// C5: draft always cancellable
ensures forall h, m :: Allowed(Draft, Cancelled, h, m)
// C6: in_progress always cancellable
ensures forall h, m :: Allowed(InProgress, Cancelled, h, m)
// C7: has AC => can start
ensures forall m :: Allowed(Draft, InProgress, true, m)
// C8: has ACs, all met => can finish
ensures Allowed(InProgress, Done, true, true)
```

## § Back-translation  *(LLM — audited by the human against Intent)*

1. A draft with **no** AC is never allowed to move to in_progress.
2. An in_progress milestone is never allowed to move to done while **not all** ACs are met.
3. Nothing ever leaves **done** (terminal).
4. Nothing ever leaves **cancelled** (terminal).
5. A **draft** may always be cancelled, whatever its ACs.
6. An **in_progress** milestone may always be cancelled, whatever its ACs.
7. A draft that **has** an AC may move to in_progress.
8. An in_progress milestone with **all** ACs met may move to done.

## § Model  *(LLM impl-modeler — blind to the claims; from `transition.go` @ v0.20.0)*

```dafny
predicate Allowed(from: Status, to: Status, hasAC: bool, allACsMet: bool)
{
  (from == Draft      && to == InProgress)                      ||  // transition.go:24
  (from == Draft      && to == Cancelled)                       ||  // transition.go:24
  (from == InProgress && to == Cancelled)                       ||  // transition.go:25
  (from == InProgress && to == Done && (!hasAC || allACsMet))       // transition.go:25 + AC guard
}
```

### Model fidelity notes — where `(hasAC, allACsMet)` is lossy vs the real code

- **The done-guard is "no *open* AC", not "all *met*".** ACs that are `deferred` or `cancelled`
  (not `met`) still let a milestone finish (`MilestoneCanGoDone`, `transition.go:265-275`; the
  standing check `milestone-done-incomplete-acs`, `acs.go:303-339`). Forced onto `allACsMet`, the
  model is *stricter* than the code on the deferred/cancelled case.
- **The done AC-guard rides in via a check-projection, not the FSM table.** `ValidateTransition`
  (`transition.go:79-94`) is pure table lookup with no AC check; the AC precondition is a
  `SeverityError` finding run in `projectionFindings` (`promote.go:171`).
- **`--force` relaxes FSM *direction* but not the AC gate** (`promote.go:92`, projection runs
  regardless of force).
- **Two cancel paths differ:** `aiwf promote M cancelled` allows cancel with any AC state (what the
  model encodes); `aiwf cancel M` blocks on an `open` AC (`promote.go:246-253`).
- **No sovereign / tdd-phase gate on milestone edges** (sovereign shapes are epic-only,
  `sovereign.go:39-46`; tdd-phase governs ACs not milestone status).
