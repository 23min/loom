# Umbrella — substrate dispatch is total

*loom self-host property (five-register convention). Subject: loom `crates/loom/src/backend.rs` `dispatch` @ v0.1.0, referenced by symbol + version, not read at verify time (G1).*

substrate: dafny

subject-repo: loom
subject-ref: v0.1.0
subject-path: crates/loom/src/backend.rs
subject-symbol: loom::backend::dispatch

## intent

Substrate → backend routing is **total**: every substrate maps to exactly one backend, and no substrate is silently left unverified. In Rust this is total by construction — `dispatch` is an exhaustive `match` over `Substrate` with no catch-all — so adding a substrate without a backend is a compile error.

## shows

`dafny → dafny-backend` : routed · every substrate → a backend that verifies : holds · a backend that runs no verifier : does not exist in the codomain

## does / proves

`does` = the `Substrate → Backend` routing, modeled as a total function. `proves` = for every substrate, dispatch yields a backend that actually verifies (`Verifies(Dispatch(s))` for all `s`) — none is routed to an inert sink. Lowering: `model.dfy`. Expected verdict: PROVED (category A).
