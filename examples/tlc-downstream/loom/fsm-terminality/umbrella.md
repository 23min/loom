# Umbrella — Milestone FSM terminality (model-checked)

*loom property, second substrate (five-register convention). Subject: aiwf `internal/entity/transition.go` @ v0.20.0, referenced by symbol + version, not read at check time (G1).*

substrate: tla

subject-repo: aiwf
subject-ref: v0.20.0
subject-path: internal/entity/transition.go
subject-symbol: transitions[Milestone]

## intent

A Milestone can never transition out of a terminal status (`done`, `cancelled`) — a terminal status is absorbing. Same claim as the Dafny seed's `fsm-terminality`, now discharged by an **explicit-state model checker** (TLC) exploring the reachable state space, rather than a deductive verifier.

## shows

`draft → in_progress` : legal · `draft → cancelled` : legal · `done → *` : none · `cancelled → *` : none

## does / proves

`does` = the Milestone transition table, modeled as a TLA+ state machine. `proves` = the invariant `TerminalAbsorbing` holds at every reachable state — a terminal status has no outgoing transition. Lowering: `model.tla` + `model.cfg`. Expected verdict: PROVED (category A).
