// mutants/M2.dfy — "div-10": mangles the numeric value by dropping its last
// digit (killed by V, and ONLY V). NumDigits(value/10) <= NumDigits(value) <=
// width, so the output stays wellformed and the canonical width is untouched —
// the bug is purely value-preservation (G-0001).
// Assembled by the harness with the preamble from canonicalize.dfy.
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value / 10, if x.width >= PAD then x.width else PAD)
}
