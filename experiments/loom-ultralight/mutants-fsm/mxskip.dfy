// mutants-fsm/mxskip.dfy — "add-skip": Milestone Draft→Done is made legal, skipping
// InProgress (breaks X_skip; killed by the !IsLegal(Milestone, Draft, Done)
// obligation). Clause-isolated: Draft is non-terminal and Done→Draft is not added,
// so terminality and one-directionality still hold.
predicate IsLegal(k: Kind, from: Status, to: Status) {
  (k == Epic && (
    (from == Proposed && (to == Active || to == Cancelled)) ||
    (from == Active && (to == Done || to == Cancelled))
  )) ||
  (k == Milestone && (
    (from == Draft && (to == InProgress || to == Cancelled || to == Done)) ||
    (from == InProgress && (to == Done || to == Cancelled))
  ))
}
