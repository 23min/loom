---
id: G-0009
title: Dafny backend bounds runs by wall-clock, not a deterministic resource limit (G1)
status: open
discovered_in: M-0016
---
## Problem

`crates/loom/src/backend.rs` bounds a Dafny run with a wall-clock timeout
(`DAFNY_TIMEOUT = 120s`, applied via `wait_timeout` in `run_dafny`). Because Z3
is nondeterministic, the `proved`/`error` boundary is machine-speed-dependent: a
slow CI box can time out (→ `error`) where a fast one proves. This violates G1
(same inputs → same outputs) for any property near the ceiling. The seed models
verify well within 120s, so the current suite is reproducible only by luck of
budget headroom.

## Direction

Pass a deterministic Dafny `--resource-limit N` (Z3 rlimit) and `--cores 1`
instead of relying on wall clock; keep a coarse wall-clock kill only as a
true-hang backstop. Out-of-resource already maps to `error` (fixed in M-0016's
wrap, commit `dd4d7b0`), so this is purely the determinism half: pick an rlimit,
confirm the three seed models still verify under it, and record the chosen
budget where the invocation lives.

## Discovered
M-0016 wrap review (code lens, finding N1).
