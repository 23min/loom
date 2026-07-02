# Umbrella — archive location ⇔ FSM terminality

*loom-light property. Subject: aiwf `internal/check/archive_rules.go` @ v0.20.0. Lowering + gap report in M-0016/AC-5–AC-6.*

substrate: dafny

## intent

A file lives under `archive/` **iff** its status is FSM-terminal (modulo the pending-sweep transient). Archive is the structural projection of terminality — a non-terminal status under `archive/` is always an error, and a terminal status outside it (past the sweep) likewise.

## shows

(examples authored with the lowering in AC-6)

## does / proves

`does` = the archived-iff-terminal predicate, modeled. `proves` = the biconditional holds for every entity status.
