// mutants/M20.dfy — "wide-plus-2": widens an already-canonical id by two (killed
// by EXACT width / no-op-on-canonical, and only that).
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value, if x.width >= PAD then x.width + 2 else PAD)
}
