A planning entity has a `kind` and moves through a per-kind status state machine.
`IsLegal(k, from, to)` decides whether a `promote`/`cancel` may move an entity of
kind `k` from status `from` to status `to`. The two kinds and their legal edges are:

- **Epic**: `Proposed → Active`, `Proposed → Cancelled`, `Active → Done`,
  `Active → Cancelled`. `Done` and `Cancelled` are terminal.
- **Milestone**: `Draft → InProgress`, `Draft → Cancelled`, `InProgress → Done`,
  `InProgress → Cancelled`. `Done` and `Cancelled` are terminal.

`IsLegal(k, from, to)` is `true` iff `(k, from, to)` is one of those edges, and
`false` for every other triple. In particular: a terminal status (`Done`,
`Cancelled`) has no outgoing transition; a status that belongs to the other kind
never transitions (e.g. an Epic is never in `Draft`); you cannot skip an
intermediate status (`Draft → Done` is illegal) or step backwards. The negative
space — which transitions are *illegal* — is as much part of the contract as the
legal edges.
