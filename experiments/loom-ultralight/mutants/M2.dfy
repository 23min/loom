// mutants/M2.dfy — "value+1": mangles the numeric value (killed by V).
// Assembled by the harness with the preamble from canonicalize.dfy.
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value + 1, if x.width >= PAD then x.width else PAD)
}
