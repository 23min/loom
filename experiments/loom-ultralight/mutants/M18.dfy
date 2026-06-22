// mutants/M18.dfy — "wide-plus-1": widens an already-canonical (>= PAD) id by one
// (killed by EXACT width / the no-op-on-canonical obligation, and only that).
// Value, kind and wellformedness are preserved.
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value, if x.width >= PAD then x.width + 1 else PAD)
}
