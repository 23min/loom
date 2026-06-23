// mutants/M3.dfy — "always-PAD": shrinks already-wide ids (killed by W).
// Assembled by the harness with the preamble from canonicalize.dfy.
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value, PAD)
}
