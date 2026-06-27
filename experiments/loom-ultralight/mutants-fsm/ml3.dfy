// mutants-fsm/ml3.dfy — "drop-epic-active-done": the Epic Active→Done edge is
// removed (breaks L; killed by the IsLegal(Epic, Active, Done) obligation).
predicate IsLegal(k: Kind, from: Status, to: Status) {
  (k == Epic && (
    (from == Proposed && (to == Active || to == Cancelled)) ||
    (from == Active && (to == Cancelled))
  )) ||
  (k == Milestone && (
    (from == Draft && (to == InProgress || to == Cancelled)) ||
    (from == InProgress && (to == Done || to == Cancelled))
  ))
}
