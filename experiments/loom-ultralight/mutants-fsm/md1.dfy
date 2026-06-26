// mutants-fsm/md1.dfy — "reverse-milestone-draft-inprogress": Milestone
// InProgress→Draft is made legal — the reverse of the legal Draft→InProgress edge
// (breaks D; killed by the forall ... IsLegal(k,f,t) ==> !IsLegal(k,t,f) obligation).
// Clause-isolated: both endpoints are non-terminal and InProgress→Draft is not a
// listed exclusion, so T, X_skip, and X_cross still hold.
predicate IsLegal(k: Kind, from: Status, to: Status) {
  (k == Epic && (
    (from == Proposed && (to == Active || to == Cancelled)) ||
    (from == Active && (to == Done || to == Cancelled))
  )) ||
  (k == Milestone && (
    (from == Draft && (to == InProgress || to == Cancelled)) ||
    (from == InProgress && (to == Done || to == Cancelled || to == Draft))
  ))
}
