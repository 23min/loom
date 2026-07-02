// model.dfy — loom lowering: terminal statuses are absorbing.
//
// Property (fsm-terminality): an entity can never transition out of a terminal status.
// Subject: aiwf internal/entity/transition.go @ v0.20.0 — the per-kind `transitions` map and
// IsTerminal. Transcribed faithfully; this model is self-contained and is NOT read from the host
// at verify time (G1). Expected verdict: PROVED (category A).

datatype Kind = Epic | Milestone | ADR | Gap | Decision | Contract
datatype Status =
    Proposed | Active | Draft | InProgress | Done | Cancelled
  | Open | Addressed | Wontfix | Accepted | Rejected | Superseded
  | Deprecated | Retired
  | Unknown // stands for any status string the kind does not recognize

// Outgoing legal edges per (kind, status), transcribed from transition.go's `transitions`.
// An unrecognized status has no outgoing edges.
function Outgoing(k: Kind, s: Status): set<Status> {
  match k
  case Epic =>
    if s == Proposed then {Active, Cancelled}
    else if s == Active then {Done, Cancelled}
    else {}
  case Milestone =>
    if s == Draft then {InProgress, Cancelled}
    else if s == InProgress then {Done, Cancelled}
    else {}
  case ADR =>
    if s == Proposed then {Accepted, Rejected}
    else if s == Accepted then {Superseded}
    else {}
  case Decision =>
    if s == Proposed then {Accepted, Rejected}
    else if s == Accepted then {Superseded}
    else {}
  case Gap =>
    if s == Open then {Addressed, Wontfix}
    else {}
  case Contract =>
    if s == Proposed then {Accepted, Rejected}
    else if s == Accepted then {Deprecated, Rejected}
    else if s == Deprecated then {Retired}
    else {}
}

// Known mirrors the Go map's key set: a status the kind recognizes.
predicate Known(k: Kind, s: Status) {
  match k
  case Epic => s in {Proposed, Active, Done, Cancelled}
  case Milestone => s in {Draft, InProgress, Done, Cancelled}
  case ADR => s in {Proposed, Accepted, Superseded, Rejected}
  case Decision => s in {Proposed, Accepted, Superseded, Rejected}
  case Gap => s in {Open, Addressed, Wontfix}
  case Contract => s in {Proposed, Accepted, Deprecated, Retired, Rejected}
}

// IsTerminal mirrors transition.go: a known status with no outgoing edges. Unknown → false.
predicate IsTerminal(k: Kind, s: Status) { Known(k, s) && Outgoing(k, s) == {} }

predicate IsLegalEdge(k: Kind, from: Status, to: Status) { to in Outgoing(k, from) }

// The property: a terminal status is absorbing — no legal edge leaves it, for any kind.
lemma TerminalIsAbsorbing()
  ensures forall k: Kind, s: Status, t: Status :: IsTerminal(k, s) ==> !IsLegalEdge(k, s, t)
  // faithfulness spot-checks — positive and negative space
  ensures IsLegalEdge(Epic, Proposed, Active)
  ensures !IsLegalEdge(Milestone, Draft, Done) // cannot skip in_progress
  ensures IsTerminal(Epic, Done) && IsTerminal(Contract, Retired)
  ensures !IsTerminal(Epic, Unknown) // an unrecognized status is NOT terminal (matches Go)
{ }
