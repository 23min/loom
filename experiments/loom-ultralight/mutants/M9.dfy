// mutants/M9.dfy — "kind-const": replaces the kind with a constant (killed by K,
// and ONLY K). Value and canonical width are preserved, so a spec that drops the
// kind-preservation clause survives it.
function Canonicalize(x: Id): Id {
  Id("Z", x.value, if x.width >= PAD then x.width else PAD)
}
