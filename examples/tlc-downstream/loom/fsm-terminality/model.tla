---- MODULE model ----
\* loom TLA+ lowering (M-0018 second substrate) — the aiwf Milestone status FSM.
\*
\* Property (fsm-terminality): a terminal status is absorbing — it has no outgoing transitions.
\* Subject: aiwf internal/entity/transition.go @ v0.20.0 — the Milestone transition table,
\* transcribed faithfully; this model is self-contained and is NOT read from the host at check
\* time (G1). TLC checks the invariant over the full reachable state space by explicit-state
\* exploration (distinct from the Dafny seed's deductive proof of the same shape).
\* Expected verdict: PROVED (no error).

Terminal == {"done", "cancelled"}

\* Legal outgoing edges per status, from the Milestone transition table. A terminal status
\* (and any unrecognized status) has no outgoing edges.
Outgoing(s) ==
    CASE s = "draft"       -> {"in_progress", "cancelled"}
      [] s = "in_progress" -> {"done", "cancelled"}
      [] OTHER             -> {}

VARIABLE status
Init == status = "draft"
Next == \E t \in Outgoing(status) : status' = t

\* The property: whenever the FSM is in a terminal status, it has no outgoing transition —
\* terminal is absorbing. Checked at every reachable state.
TerminalAbsorbing == (status \in Terminal) => (Outgoing(status) = {})
====
