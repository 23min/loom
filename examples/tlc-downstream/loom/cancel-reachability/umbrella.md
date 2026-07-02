# Umbrella — cancel reachability (model-checked, at-risk)

*loom property, second substrate (five-register convention). Subject: aiwf `internal/entity/transition.go` @ v0.20.0, referenced by symbol + version, not read at check time (G1).*

substrate: tla

subject-repo: aiwf
subject-ref: v0.20.0
subject-path: internal/entity/transition.go
subject-symbol: transitions[Milestone]

## intent

The **at-risk** probe: the claim under scrutiny is that a Milestone never reaches the `cancelled` status. It should NOT hold — `draft → cancelled` is a legal edge — so the model checker is expected to refute it and surface the reachable cancel path as a counterexample.

## shows

`draft → cancelled` : reachable in one step · claim `status ≠ cancelled` : violated · counterexample : `draft ⇒ cancelled`

## does / proves

`does` = the Milestone FSM. `proves` = attempts the invariant `NeverCancelled` (`status ≠ "cancelled"`); TLC finds the reachable state `cancelled` and returns the counterexample trace. Lowering: `model.tla` + `model.cfg`. Expected verdict: REFUTED — a category-(B) gap carrying the trace.
