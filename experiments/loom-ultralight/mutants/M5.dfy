// mutants/M5.dfy — "zero-value": destroys the value (killed by V).
// Assembled by the harness with the preamble from canonicalize.dfy.
function Canonicalize(x: Id): Id {
  Id(x.kind, 0, PAD)
}
