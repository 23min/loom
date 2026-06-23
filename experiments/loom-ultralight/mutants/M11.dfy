// mutants/M11.dfy — "kind-swap": rewrites one kind tag ("E"->"M") and leaves the
// rest alone (killed by K, and ONLY K). A subtle single-point kind bug: only a
// spec that pins kind exactly catches it.
function Canonicalize(x: Id): Id {
  Id(if x.kind == "E" then "M" else x.kind, x.value, if x.width >= PAD then x.width else PAD)
}
