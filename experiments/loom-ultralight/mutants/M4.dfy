// mutants/M4.dfy — "drop-kind": loses the kind tag (killed by K).
// Assembled by the harness with the preamble from canonicalize.dfy.
function Canonicalize(x: Id): Id {
  Id("", x.value, if x.width >= PAD then x.width else PAD)
}
