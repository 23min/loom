// mutants-fsm/ml4.dfy — "drop-milestone-draft-inprogress": the Milestone
// Draft→InProgress edge is removed (breaks L; killed by the
// IsLegal(Milestone, Draft, InProgress) obligation).
predicate IsLegal(k: Kind, from: Status, to: Status) {
  (k == Epic && (
    (from == Proposed && (to == Active || to == Cancelled)) ||
    (from == Active && (to == Done || to == Cancelled))
  )) ||
  (k == Milestone && (
    (from == Draft && (to == Cancelled)) ||
    (from == InProgress && (to == Done || to == Cancelled))
  ))
}
