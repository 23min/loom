// mutants/M15.dfy — "over-pad-2": pads a narrow id to PAD+2 instead of PAD
// (killed by EXACT width, and only that). Wellformed and >= PAD, so a spec whose
// width clause is only a lower bound (width >= PAD) survives it; the exact
// equality width == max(x.width, PAD) kills it. This is the demonstrated
// incentivized weakening (G-0003).
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value, if x.width >= PAD then x.width else PAD + 2)
}
