// mutants-fsm/mt1.dfy — "terminal-outgoing-epic": Epic Done→Cancelled is made legal,
// giving the terminal Done an outgoing edge (breaks T; killed by the
// forall k,t :: !IsLegal(k, Done, t) obligation). Clause-isolated: Cancelled→Done is
// not added, so one-directionality holds.
predicate IsLegal(k: Kind, from: Status, to: Status) {
  (k == Epic && (
    (from == Proposed && (to == Active || to == Cancelled)) ||
    (from == Active && (to == Done || to == Cancelled)) ||
    (from == Done && (to == Cancelled))
  )) ||
  (k == Milestone && (
    (from == Draft && (to == InProgress || to == Cancelled)) ||
    (from == InProgress && (to == Done || to == Cancelled))
  ))
}
