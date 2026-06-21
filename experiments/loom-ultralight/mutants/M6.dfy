// mutants/M6.dfy — "pad-to-3": off-by-one canonical width (killed by W).
// Assembled by the harness with the preamble from canonicalize.dfy.
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value, if x.width >= PAD then x.width else 3)
}
