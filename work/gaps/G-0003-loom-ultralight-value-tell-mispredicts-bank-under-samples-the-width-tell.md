---
id: G-0003
title: 'loom-ultralight: value-tell mispredicts; bank under-samples the width tell'
status: open
---
## Problem

The loom-ultralight design (`docs/loom-ultralight.md` §3.3) pre-registered
**value-preservation (V)** as the discriminating tell, predicting a gamed spec
scores ≤ 5/8; `G-0001` then sharpened the bank to make that V-tell clean. But the
endogenous weakening the models actually exhibit is in **width-exactness (W)**,
not value: under incentive they pin width as a lower bound (`width >= PAD`)
instead of the exact `width == max(x.width, PAD)`. Value, kind and wellformedness
are pinned exactly by ~100% of specs in *both* arms.

## Impact

- The pre-registered ≤ 5/8 threshold and the V-tell framing are **wrong**: the
  incentivized kill-rate is ~0.82, not ≤ 0.625. The `G-0001` value-isolation work
  was sharpening a clause the models do not weaken.
- The 8-mutant bank had only **one** mutant (M8, over-pad to `PAD+1`) sensitive to
  the width loosening, so the real effect showed up as a marginal ~1/8 — easy to
  dismiss as noise.

## Fix

- The bank was grown 8 → 20 (commit b26662f), adding a width over-pad cluster
  (M8/M15/M16/M17) that a lower-bound width spec survives but the exact clause
  kills — resolving the effect across four mutants instead of one.
- A verifier-based structural strength measure (`--strength`, commit 15d891e)
  confirms the localization independently: K/V/F entailed by 100% of specs in
  every arm; the entire gap is width exact-vs-bound. Addressed by commits
  b26662f, 15d891e.
