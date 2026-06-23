// mutants/M7.dfy — "value-0-bug": corner-case value bug at value==0 (killed by V).
// Assembled by the harness with the preamble from canonicalize.dfy.
function Canonicalize(x: Id): Id {
  Id(x.kind, if x.value == 0 then 1 else x.value, if x.width >= PAD then x.width else PAD)
}
