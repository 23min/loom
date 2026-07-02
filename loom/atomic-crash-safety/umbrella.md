# Umbrella — report writes are crash-safe

*loom self-host property (five-register convention). Subject: loom `crates/loom/src/atomic.rs` `loom::atomic` (the `write_atomic` temp→rename protocol) @ v0.1.0, referenced by symbol + version, not read at verify time (G1). The pinned symbol is the `subject-*` fields below; this line restates it for the reader.*

substrate: dafny

subject-repo: loom
subject-ref: v0.1.0
subject-path: crates/loom/src/atomic.rs
subject-symbol: loom::atomic

## intent

A report write is **crash-safe**: contents are staged to a sibling temp file, then atomically renamed into place. A crash at any point leaves the destination either fully-old (or absent, if none existed) or fully-new — never a partial or torn report (C3).

## shows

crash before staging : dest unchanged · crash after staging, before rename : dest still fully-old/absent (temp is a sibling, not the dest) · rename failed : dest unchanged, temp cleaned up · crash after rename : dest fully-new

## does / proves

`does` = the temp-write→rename protocol, modeled as the destination's observable state across every phase. `proves` = at every crash point the observed destination is fully-old (`== start`, covering both the prior report and absence) or fully-new — never partial. Lowering: `model.dfy`. Expected verdict: PROVED (category A).
