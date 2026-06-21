// mutants/M8.dfy — "over-pad": pads narrow ids to 5, not 4 (killed by W).
// Assembled by the harness with the preamble from canonicalize.dfy.
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value, if x.width >= PAD then x.width else PAD + 1)
}
