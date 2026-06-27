---
id: G-0004
title: Unify loom-ultralight model-filtering across kill-rate and strength outputs
status: open
discovered_in: M-0006
---

## What's missing

`active_models()` (`LOOM_MODELS`) narrows the kill-rate path (`score_trials`), so under a
single-model run `results.json` carries only the active models' rows — but the strength
path (`compute_strength` / `strength_rows_json`) still iterates all of `MODELS`, emitting
zero rows for ungenerated models. Same run, two output files, different row membership.
Resolve the active-model list once in `main` and thread it as an explicit parameter into
both paths (removing the env read deep inside `score_trials`), so the two files agree.
Also make `verdict.json` self-contained: carry the per-arm validity *rate* (or `trials`),
not just the valid count, so the over-claim signal is legible from the verdict artifact
alone without cross-referencing `results.json`.

## Why it matters

The successor (harder-subject) study reuses this harness; the row-membership divergence
and the env-deep-inside coupling are a trap for a future reader (the `wf-rethink` design
review flagged the comment as misleading). Harmless to the recorded E-0002 NO-GO (opus-4.8
is present in both files), so it is a cleanliness/maintainability fix, not a correctness
one — but worth closing before the harness is run again.
