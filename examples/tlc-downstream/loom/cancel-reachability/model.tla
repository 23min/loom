---- MODULE model ----
\* loom TLA+ lowering (M-0018 second substrate) — the at-risk probe on the aiwf Milestone FSM.
\*
\* Property (cancel-reachability): the claim under scrutiny is that a milestone never reaches the
\* `cancelled` status. This is the *at-risk* claim — a cancel path IS reachable (draft -> cancelled
\* is a legal edge), so TLC refutes it and prints the counterexample trace. Mirrors M-0016's
\* at-risk cancel property, now surfaced by a model checker rather than a deductive verifier.
\* Subject: aiwf internal/entity/transition.go @ v0.20.0, referenced not read (G1).
\* Expected verdict: REFUTED (a category-(B) gap carrying the counterexample).

Outgoing(s) ==
    CASE s = "draft"       -> {"in_progress", "cancelled"}
      [] s = "in_progress" -> {"done", "cancelled"}
      [] OTHER             -> {}

VARIABLE status
Init == status = "draft"
Next == \E t \in Outgoing(status) : status' = t

\* The (false) claim: `cancelled` is never reached. TLC finds draft -> cancelled and refutes it.
NeverCancelled == status # "cancelled"
====
