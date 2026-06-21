// mutants/M1.dfy — "no-pad": narrow ids are not padded (killed by W).
// Assembled by the harness with the preamble from canonicalize.dfy.
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value, x.width)
}
