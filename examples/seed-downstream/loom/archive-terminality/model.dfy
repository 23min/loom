// model.dfy — loom lowering: archive location is the projection of FSM-terminality.
//
// Property (archive-terminality): a non-milestone entity is pending archive-sweep iff its status is
// FSM-terminal (modulo the pending-sweep transient). Milestones ride with their parent epic and are
// excluded. Subject: aiwf internal/check/archive_rules.go @ v0.20.0 (isPendingSweep) +
// internal/entity/transition.go (IsTerminal). Self-contained; not read from the host (G1). Expected
// verdict: PROVED (category A).

datatype Kind = Epic | Milestone | ADR | Gap | Decision | Contract
datatype Status =
    Proposed | Active | Draft | InProgress | Done | Cancelled
  | Open | Addressed | Wontfix | Accepted | Rejected | Superseded
  | Deprecated | Retired
  | Unknown

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

predicate Known(k: Kind, s: Status) {
  match k
  case Epic => s in {Proposed, Active, Done, Cancelled}
  case Milestone => s in {Draft, InProgress, Done, Cancelled}
  case ADR => s in {Proposed, Accepted, Superseded, Rejected}
  case Decision => s in {Proposed, Accepted, Superseded, Rejected}
  case Gap => s in {Open, Addressed, Wontfix}
  case Contract => s in {Proposed, Accepted, Deprecated, Retired, Rejected}
}

predicate IsTerminal(k: Kind, s: Status) { Known(k, s) && Outgoing(k, s) == {} }

// isPendingSweep mirrors archive_rules.go: an un-archived, known-status, terminal, non-milestone
// entity awaits the sweep. (The Go `status == ""` guard is subsumed by Known here — an empty status
// is not a recognized Status value.)
predicate IsPendingSweep(k: Kind, s: Status, archived: bool) {
  !archived && Known(k, s) && IsTerminal(k, s) && k != Milestone
}

lemma ArchiveCoupledToTerminality()
  // soundness: a pending sweep is always terminal and never a milestone — no non-terminal is swept
  ensures forall k: Kind, s: Status, a: bool ::
    IsPendingSweep(k, s, a) ==> IsTerminal(k, s) && k != Milestone
  // completeness: every un-archived, known, terminal, non-milestone entity is pending
  ensures forall k: Kind, s: Status ::
    (Known(k, s) && IsTerminal(k, s) && k != Milestone) ==> IsPendingSweep(k, s, false)
  // an already-archived entity is never "pending" — the transient has completed
  ensures forall k: Kind, s: Status :: !IsPendingSweep(k, s, true)
{ }
