// mutants/M16.dfy — "over-pad-x2": pads a narrow id to PAD*2 (killed by EXACT
// width only; survives a lower-bound width spec). See G-0003.
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value, if x.width >= PAD then x.width else PAD * 2)
}
