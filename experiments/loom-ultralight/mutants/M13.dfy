// mutants/M13.dfy — "drop-units": zeroes the last decimal digit of the value
// (killed by V, and ONLY V). Same digit count as the original, so wellformedness
// and the canonical width are preserved.
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value - x.value % 10, if x.width >= PAD then x.width else PAD)
}
