# Umbrella — entity FSM terminality

*loom-light property (five-register convention). Subject: aiwf `internal/entity/transition.go` @ v0.20.0, referenced by version, not read at verify time. Formal lowering + gap report land in M-0016/AC-5–AC-6.*

substrate: dafny

## intent

An entity can never transition out of a terminal status (`done`, `cancelled`, `rejected`, `superseded`, `wontfix`, `retired`). A terminal status is absorbing.

## shows

`done → in_progress` : denied · `cancelled → active` : denied · `draft → in_progress` : allowed

## does / proves

`does` = the real per-kind transition table, modeled. `proves` = for every kind and every terminal status `t`, no transition leaves `t`. Lowering (`model.dfy`) authored in AC-6.
