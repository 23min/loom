// mutants/M5.dfy — "zero-value": destroys the value, zeroing it (killed by V,
// and ONLY V). The canonical width is preserved (not forced to PAD), so a
// wide id keeps its width and only value-preservation breaks (G-0001).
// Assembled by the harness with the preamble from canonicalize.dfy.
function Canonicalize(x: Id): Id {
  Id(x.kind, 0, if x.width >= PAD then x.width else PAD)
}
