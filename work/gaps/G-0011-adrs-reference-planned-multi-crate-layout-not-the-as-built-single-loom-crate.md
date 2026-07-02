---
id: G-0011
title: ADRs reference planned multi-crate layout, not the as-built single loom crate
status: open
discovered_in: M-0016
---
## Problem

`docs/adr/ADR-0002-dafny-as-verifier.md` and `docs/adr/ADR-0003-python-as-target.md`
describe a planned multi-crate architecture that M-0016 did not build:

- ADR-0002 refers to `crates/loom-compile-dafny` (the backend "behind the
  `Verifier` trait") and `crates/loom-verify/src/counterexample.rs`.
- ADR-0003 refers to `crates/loom-compile-python`.

The as-built E-0005 implementation is a single `crates/loom` crate: substrate
dispatch is a `Backend` enum + exhaustive `dispatch` in `src/backend.rs`, not a
`Verifier` trait across per-substrate crates. A reader following the ADRs looks
for crates and a trait that do not exist. ADR-0003 is additionally moot (rejected
decision; no Python codegen — ADR-0017), and ADR-0001 already carries a revision
note retiring `loom-compile-python`.

## Direction

A deliberate decision-record update (not a wrap-commit rider): either add revision
notes to ADR-0002/0003 pointing at the as-built single-crate layout, or supersede
them. Decision-record prose is edited on purpose, with its own rationale — hence a
gap rather than an inline fix during the M-0016 wrap.

## Discovered
M-0016 wrap review (doc-lint step).
