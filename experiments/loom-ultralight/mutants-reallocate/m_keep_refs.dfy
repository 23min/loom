// mutants-reallocate/m_keep_refs.dfy — "drops every rewrite": renames ids correctly
// but leaves every reference sequence untouched, so no reference to oldId is ever
// rewritten (breaks C; killed by the per-entity refs-rewrite obligation). The id map
// is correct, so R and F still hold — this mutant is isolated to clause C, the
// predicted tell (the crude, rewrite-nothing case).
// Assembled with the preamble + gold ensures from reallocate.dfy.
function Reallocate(t: Tree, oldId: Id, newId: Id): Tree {
  seq(|t|, i requires 0 <= i < |t| =>
    Entity(Rw(t[i].id, oldId, newId), t[i].refs))
}
