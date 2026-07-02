# Umbrella — umbrella parsing is total

*loom self-host property (five-register convention). Subject: loom `crates/loom/src/umbrella.rs` `parse` @ v0.1.0, referenced by symbol + version, not read at verify time (G1).*

substrate: dafny

subject-repo: loom
subject-ref: v0.1.0
subject-path: crates/loom/src/umbrella.rs
subject-symbol: loom::umbrella::parse

## intent

Umbrella parsing is **total**: every input yields either a parsed umbrella or a typed rejection — never a panic, never a silent misparse. The classification over the `substrate:` declarations found in the source is exhaustive and mutually exclusive.

## shows

no `substrate:` : rejected (missing) · empty `substrate:` value : rejected (missing) · two `substrate:` lines : rejected (duplicate) · one unknown value : rejected (unknown) · one known value : parsed

## does / proves

`does` = the parse's decision over the sequence of `substrate:` declarations, modeled as a total function (abstracting the byte-level line scan). `proves` = for every possible input, the parser returns exactly one of {ok, missing, duplicate, unknown} — the classification is a total partition, and each branch is reachable (faithfulness spot-checks). Lowering: `model.dfy`. Expected verdict: PROVED (category A).
