// mutants-fsm/ml1.dfy — "drop-epic-proposed-active": the Epic Proposed→Active edge
// is removed (breaks L; killed by the IsLegal(Epic, Proposed, Active) obligation).
// Assembled with the preamble + gold ensures from fsm.dfy.
predicate IsLegal(k: Kind, from: Status, to: Status) {
  (k == Epic && (
    (from == Proposed && (to == Cancelled)) ||
    (from == Active && (to == Done || to == Cancelled))
  )) ||
  (k == Milestone && (
    (from == Draft && (to == InProgress || to == Cancelled)) ||
    (from == InProgress && (to == Done || to == Cancelled))
  ))
}
