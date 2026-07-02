# Umbrella — cancel-target edge legality

*loom-light property. Subject: aiwf `internal/entity/transition.go` `CancelTarget` @ v0.20.0. The **at-risk** property — the recognition probe flagged that the existing test proves "terminal + allowed" but NOT that the `from → target` edge is FSM-legal, and nothing downstream re-checks it. Lowering + gap report in M-0016/AC-5–AC-6.*

substrate: dafny

## intent

`cancel` may only route a transition the FSM permits: for any non-terminal `(kind, status)`, `CancelTarget` is either empty or a status `t` such that `status → t` is a legal FSM edge — cancel can never fabricate an illegal transition.

## shows

(examples authored with the lowering in AC-6)

## does / proves

`does` = `CancelTarget` + the transition table, modeled. `proves` = for every kind and non-terminal `s`, `CancelTarget(k,s) == "" ∨ legal(k, s, CancelTarget(k,s))`. Expected to surface a `(B)` gap — the at-risk finding.
