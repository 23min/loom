// mutants/M14.dfy — "value-swap": rewrites one value (7->8) and leaves the rest
// alone (killed by V, and ONLY V). A subtle single-point value bug: only a spec
// that pins value exactly catches it.
function Canonicalize(x: Id): Id {
  Id(x.kind, if x.value == 7 then 8 else x.value, if x.width >= PAD then x.width else PAD)
}
