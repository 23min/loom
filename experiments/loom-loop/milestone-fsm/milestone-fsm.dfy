// Loom loop 1 — milestone status-transition FSM — reproducible verifier input (G1).
// Run:  dafny verify milestone-fsm.dfy
// Expected: 6 verified, 3 errors  (C1, C2 fail; Examples fails at ex2 — see gap-report.md).
//
// § MODEL:  impl-modeler subagent, from the real aiwf transition.go @ v0.20.0 (blind to claims)
// § CLAIMS: umbrella-author subagent, from the human's Intent + Examples (blind to the code)

datatype Status = Draft | InProgress | Done | Cancelled

// ---- MODEL (the real code, as modeled) ----
predicate Allowed(from: Status, to: Status, hasAC: bool, allACsMet: bool)
{
  (from == Draft      && to == InProgress)                      ||
  (from == Draft      && to == Cancelled)                       ||
  (from == InProgress && to == Cancelled)                       ||
  (from == InProgress && to == Done && (!hasAC || allACsMet))
}

// ---- CLAIMS, one lemma each (the human's intent, formalized) ----
lemma C1() ensures forall m :: !Allowed(Draft, InProgress, false, m) {}       // FAILS (real gap)
lemma C2() ensures forall h :: !Allowed(InProgress, Done, h, false) {}         // FAILS (no-AC edge)
lemma C3() ensures forall t, h, m :: !Allowed(Done, t, h, m) {}               // ok
lemma C4() ensures forall t, h, m :: !Allowed(Cancelled, t, h, m) {}          // ok
lemma C5() ensures forall h, m :: Allowed(Draft, Cancelled, h, m) {}          // ok
lemma C6() ensures forall h, m :: Allowed(InProgress, Cancelled, h, m) {}     // ok
lemma C7() ensures forall m :: Allowed(Draft, InProgress, true, m) {}         // ok
lemma C8() ensures Allowed(InProgress, Done, true, true) {}                   // ok

// ---- EXAMPLES check: does the MODEL (code) match the human's expected verdicts? ----
lemma Examples() ensures
     Allowed(Draft, InProgress, true, true)          // ex1 has-ACs start          -> allowed  ok
  && !Allowed(Draft, InProgress, false, true)        // ex2 no-ACs start           -> denied   FAILS
  && Allowed(Draft, Cancelled, false, true)          // ex3 no-ACs cancel          -> allowed  ok
  && Allowed(InProgress, Done, true, true)           // ex4 all-met finish         -> allowed  ok
  && !Allowed(InProgress, Done, true, false)         // ex5 unmet-AC finish        -> denied   ok
  && Allowed(InProgress, Cancelled, true, false)     // ex6 unmet-AC cancel        -> allowed  ok
  && !Allowed(Done, InProgress, true, true)          // ex7 done->in_progress      -> denied   ok
  && !Allowed(Cancelled, InProgress, true, true)     // ex8 cancelled->in_progress -> denied   ok
{}
