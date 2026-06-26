// mutants-fsm/mxcross.dfy — "add-cross-kind": Epic Draft→Active is made legal, but
// Draft is not an Epic status (breaks X_cross; killed by the
// !IsLegal(Epic, Draft, Active) obligation). Clause-isolated: Active→Draft is not
// added, so one-directionality holds, and Draft is non-terminal.
predicate IsLegal(k: Kind, from: Status, to: Status) {
  (k == Epic && (
    (from == Proposed && (to == Active || to == Cancelled)) ||
    (from == Active && (to == Done || to == Cancelled)) ||
    (from == Draft && (to == Active))
  )) ||
  (k == Milestone && (
    (from == Draft && (to == InProgress || to == Cancelled)) ||
    (from == InProgress && (to == Done || to == Cancelled))
  ))
}
