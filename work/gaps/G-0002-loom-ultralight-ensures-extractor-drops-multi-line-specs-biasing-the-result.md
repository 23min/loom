---
id: G-0002
title: 'loom-ultralight: ensures extractor drops multi-line specs, biasing the result'
status: open
---
## Problem

The harness's `extract_spec_ensures` (`experiments/loom-ultralight/src/main.rs`)
assumed "one `ensures` per line" — it kept only lines beginning with `ensures`
and broke on the first other non-blank line. But Dafny specs routinely write a
single *multi-line* ensures, e.g. `ensures var c := Canonicalize(x); A && B && …`
or one clause wrapped across lines. The extractor truncated those to a dangling
`ensures`, which fails to resolve, so the harness scored a **complete, correct
spec as invalid** and dropped it.

## Impact

- The disinterested arm — asked for a "complete, audited" spec — writes the
  richer multi-line / `var`-binding style more often, so the bug **preferentially
  discarded disinterested specs**: the discarded-sample artifact was *correlated
  with the experimental condition*, the worst kind of measurement bias.
- It nuked haiku to **1/10 valid** in both arms (haiku always uses the
  `var c :=` style), making it look incapable when it was merely unparseable.
- On the first paid run this masked the effect: the validity attrition looked
  like "over-specification", and the kill-rate gap was biased and under-counted.

## Fix

`extract_spec_ensures` now captures the whole ensures region (first `ensures`
keyword → lemma body), preserving multi-line clauses; `requires` are still
dropped. Re-scoring the cached run recovered haiku (1/10 → 10/10 valid) and
flipped sonnet's gap to the predicted direction. A `--rescore` mode was added so
the extractor and the mutant bank can be re-measured on cached generations with
no API cost. Addressed by commit b26662f.
