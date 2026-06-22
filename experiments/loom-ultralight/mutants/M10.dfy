// mutants/M10.dfy — "kind-suffix": appends to the kind (killed by K, and ONLY K).
function Canonicalize(x: Id): Id {
  Id(x.kind + "X", x.value, if x.width >= PAD then x.width else PAD)
}
