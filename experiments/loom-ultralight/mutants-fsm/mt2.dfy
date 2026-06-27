// mutants-fsm/mt2.dfy — "terminal-outgoing-milestone": Milestone Done→Draft is made
// legal, giving the terminal Done an outgoing edge (breaks T; killed by the
// forall k,t :: !IsLegal(k, Done, t) obligation). Clause-isolated: Draft→Done is not
// in the reference, so one-directionality holds, and this is not the X_skip edge.
predicate IsLegal(k: Kind, from: Status, to: Status) {
  (k == Epic && (
    (from == Proposed && (to == Active || to == Cancelled)) ||
    (from == Active && (to == Done || to == Cancelled))
  )) ||
  (k == Milestone && (
    (from == Draft && (to == InProgress || to == Cancelled)) ||
    (from == InProgress && (to == Done || to == Cancelled)) ||
    (from == Done && (to == Draft))
  ))
}
