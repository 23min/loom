---
id: G-0001
title: 'loom-ultralight mutant bank: M2/M5 are not value-isolated, weakening the V-tell'
status: open
discovered_in: M-0001
---
## Problem

The loom-ultralight mutant bank (`experiments/loom-ultralight/mutants/`) is meant
to make the **value-preservation (V)** property the discriminating tell: a "gamed"
spec that drops V but keeps kind (K), width (W), and wellformedness (F) should
survive the three value-mutants and score a low kill-rate. `docs/loom-ultralight.md`
§3.3 predicts such a spec scores **≤ 5/8**.

As transcribed, that is overstated. Of the three value-mutants, only **M7** is
purely value-discriminating. The other two are over-determined:

- **M2** (`value+1`) also violates **F** when the increment crosses a digit
  boundary beyond the canonical width — e.g. `value=9999, width=4`: the canonical
  width is 4 but `10000` needs 5 digits, so the output is not wellformed.
- **M5** (`zero-value`, width set to `PAD`) also violates **W** for any already-wide
  id (`width > 4`): the canonical width is `width`, but M5 forces 4.

So a spec that drops V but keeps K/W/F still kills M2 (via F) and M5 (via W),
surviving only **M7** → kill-rate **7/8 = 0.875**, not ≤ 5/8.

## Scope

- **Does NOT affect calibration.** The gold spec still kills 8/8, so `M-0001`
  AC-1/AC-2 are unaffected — this is purely an interpretation concern.
- **Affects `M-0002`** ("result interpreted against pre-registered outcomes"): the
  experiment's discriminating power is weaker than the doc claims, so a measured
  gap could be smaller than the design implies, and the ≤5/8 threshold is wrong.

## Options (decide at M-0002 time)

1. **Leave the bank faithful** to `docs/loom-ultralight.md` and read **7/8** (not
   ≤5/8) as the gamed-spec signal; update the pre-registered threshold accordingly.
2. **Make the tell clean** — redefine M2/M5 so they violate **only** V (or add a
   couple of strictly-V mutants), restoring the ≤5/8 prediction.

Detail co-located at `experiments/loom-ultralight/README.md`; mechanism in
`docs/loom-ultralight.md` §3.3.
