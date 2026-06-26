// mutants-fsm/md2.dfy — "reverse-epic-proposed-active": Epic Active→Proposed is made
// legal — the reverse of the legal Proposed→Active edge (breaks D; killed by the
// forall ... IsLegal(k,f,t) ==> !IsLegal(k,t,f) obligation). Clause-isolated: both
// endpoints are non-terminal and Active→Proposed is not a listed exclusion.
predicate IsLegal(k: Kind, from: Status, to: Status) {
  (k == Epic && (
    (from == Proposed && (to == Active || to == Cancelled)) ||
    (from == Active && (to == Done || to == Cancelled || to == Proposed))
  )) ||
  (k == Milestone && (
    (from == Draft && (to == InProgress || to == Cancelled)) ||
    (from == InProgress && (to == Done || to == Cancelled))
  ))
}
