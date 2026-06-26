// fsm.dfy — the FSM status-transition legality subject + the gold contract.
//
// Subject: a faithful model of aiwf's per-kind status-transition FSM
// (aiwf internal/entity/transition.go). For two kinds — Epic and Milestone —
// IsLegal(kind, from, to) decides whether `aiwf promote` / `aiwf cancel` may move
// an entity of that kind from status `from` to status `to`. The load-bearing
// obligation is NEGATIVE SPACE: a complete spec must pin which transitions are
// *illegal* — terminal states have no outgoing edge, a status of the wrong kind
// never transitions, you cannot skip a state or step backwards — not merely list
// the legal edges. A positive-only spec hides exactly there.
//
//   dafny verify fsm.dfy   # GoldSpec must verify (M-0004 AC-1)
//
// This file is ALSO the single source the harness slices for the shared preamble,
// the reference implementation, and the gold spec's `ensures` clauses (the
// BEGIN/END sentinels below are load-bearing — the Rust harness greps for them).

// === BEGIN PREAMBLE ===
datatype Kind = Epic | Milestone
datatype Status = Proposed | Active | Draft | InProgress | Done | Cancelled
// === END PREAMBLE ===

// === BEGIN REFERENCE IMPL ===
// THE REFERENCE IMPLEMENTATION (correct) — the per-kind legal-edge set,
// transcribed from transition.go's `transitions` map for Epic and Milestone:
//   Epic:      proposed→{active,cancelled}   active→{done,cancelled}      done,cancelled terminal
//   Milestone: draft→{in_progress,cancelled} in_progress→{done,cancelled} done,cancelled terminal
predicate IsLegal(k: Kind, from: Status, to: Status) {
  (k == Epic && (
    (from == Proposed && (to == Active || to == Cancelled)) ||
    (from == Active && (to == Done || to == Cancelled))
  )) ||
  (k == Milestone && (
    (from == Draft && (to == InProgress || to == Cancelled)) ||
    (from == InProgress && (to == Done || to == Cancelled))
  ))
}
// === END REFERENCE IMPL ===

// the gold spec, as a lemma over IsLegal; `dafny verify` checks it.
lemma GoldSpec()
// === BEGIN GOLD SPEC ENSURES ===
  // (L) positive space — every reference-legal edge is legal
  ensures IsLegal(Epic, Proposed, Active)                                     // (L) legal edge
  ensures IsLegal(Epic, Active, Done)                                         // (L) legal edge
  ensures IsLegal(Milestone, Draft, InProgress)                              // (L) legal edge
  ensures IsLegal(Milestone, InProgress, Done)                               // (L) legal edge
  // (X_skip) negative space — you cannot skip an intermediate state  <-- the tell
  ensures !IsLegal(Milestone, Draft, Done)                                    // skipping in_progress
  // (X_cross) negative space — a status of the wrong kind never transitions  <-- the tell
  ensures !IsLegal(Epic, Draft, Active)                                       // Draft is not an Epic status
  // (T) terminality — Done and Cancelled have no outgoing legal edge, any kind  <-- the tell
  ensures forall k: Kind, t: Status :: !IsLegal(k, Done, t)                   // Done terminal
  ensures forall k: Kind, t: Status :: !IsLegal(k, Cancelled, t)             // Cancelled terminal
  // (D) one-directionality — no legal edge has a legal reverse  <-- the tell
  ensures forall k: Kind, f: Status, t: Status :: IsLegal(k, f, t) ==> !IsLegal(k, t, f)  // asymmetry
// === END GOLD SPEC ENSURES ===
{ }
