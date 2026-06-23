// mutants/M12.dfy — "div-100": drops the last two digits of the value (killed by
// V, and ONLY V). NumDigits(value/100) <= NumDigits(value) <= width, so the
// output stays wellformed and the canonical width is untouched.
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value / 100, if x.width >= PAD then x.width else PAD)
}
