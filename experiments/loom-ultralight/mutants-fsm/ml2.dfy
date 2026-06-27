// mutants-fsm/ml2.dfy — "drop-milestone-inprogress-done": the Milestone
// InProgress→Done edge is removed (breaks L; killed by the
// IsLegal(Milestone, InProgress, Done) obligation).
predicate IsLegal(k: Kind, from: Status, to: Status) {
  (k == Epic && (
    (from == Proposed && (to == Active || to == Cancelled)) ||
    (from == Active && (to == Done || to == Cancelled))
  )) ||
  (k == Milestone && (
    (from == Draft && (to == InProgress || to == Cancelled)) ||
    (from == InProgress && (to == Cancelled))
  ))
}
