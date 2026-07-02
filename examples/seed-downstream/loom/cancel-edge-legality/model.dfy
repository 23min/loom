// model.dfy — loom lowering: cancel routes only FSM-legal edges. THE AT-RISK PROPERTY.
//
// Property (cancel-edge-legality): for every non-terminal (kind, status), CancelTarget is empty OR a
// status t such that status -> t is a legal FSM edge — cancel can never fabricate an illegal
// transition. Subject: aiwf internal/entity/transition.go @ v0.20.0 (CancelTarget + `transitions`).
//
// Expected verdict: REFUTED (category B). CancelTarget for Epic/Milestone/Gap hardcodes the target
// ("cancelled"/"wontfix") WITHOUT checking the from->target edge is legal. An unrecognized status
// (which IsTerminal treats as non-terminal, matching the Go code) therefore receives an FSM-illegal
// target, and nothing downstream re-checks it — the recognition probe's at-risk flag. ADR,
// Decision, and Contract were fixed to return "" in those cases (G-0131/G-0163), so they do not
// exhibit the gap; Epic/Milestone/Gap were not.

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

predicate IsLegalEdge(k: Kind, from: Status, to: Status) { to in Outgoing(k, from) }

datatype CancelResult = NoTarget | To(target: Status)

// CancelTarget mirrors transition.go's switch. Epic/Milestone/Gap ignore the current status and
// return a fixed terminal target; ADR/Decision/Contract are state-aware and return NoTarget where
// the edge would be illegal.
function CancelTarget(k: Kind, s: Status): CancelResult {
  match k
  case Epic => To(Cancelled)
  case Milestone => To(Cancelled)
  case Gap => To(Wontfix)
  case ADR => if s == Proposed then To(Rejected) else NoTarget
  case Decision => if s == Proposed then To(Rejected) else NoTarget
  case Contract =>
    if s == Proposed || s == Accepted then To(Rejected)
    else if s == Deprecated then To(Retired)
    else NoTarget
}

// The claim: cancel never fabricates an illegal transition out of a non-terminal status.
// This lemma does NOT verify — Dafny surfaces the (Epic/Milestone/Gap, unrecognized-status)
// counterexample, which is the (B) finding.
lemma CancelRoutesOnlyLegalEdges()
  ensures forall k: Kind, s: Status ::
    !IsTerminal(k, s) ==>
      (CancelTarget(k, s).NoTarget? || IsLegalEdge(k, s, CancelTarget(k, s).target))
{ }
