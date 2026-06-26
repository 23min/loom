// mutants-fsm/mt3.dfy — "terminal-outgoing-cancelled": Epic Cancelled→Done is made
// legal, giving the terminal Cancelled an outgoing edge (breaks T2 — Cancelled
// terminality; killed by the forall k,t :: !IsLegal(k, Cancelled, t) obligation).
// Clause-isolated: Done is untouched (no outgoing edge added → T1 holds), and the
// reverse edge Done→Cancelled is absent (Done is terminal) → one-directionality
// holds.
predicate IsLegal(k: Kind, from: Status, to: Status) {
  (k == Epic && (
    (from == Proposed && (to == Active || to == Cancelled)) ||
    (from == Active && (to == Done || to == Cancelled)) ||
    (from == Cancelled && (to == Done))
  )) ||
  (k == Milestone && (
    (from == Draft && (to == InProgress || to == Cancelled)) ||
    (from == InProgress && (to == Done || to == Cancelled))
  ))
}
