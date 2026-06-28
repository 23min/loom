// mutants-reallocate/m_leave_old.dfy — "forgot to rename": rewrites references
// but never renames the target entity, so the target keeps oldId instead of newId
// (breaks R, the rename obligation). Every other id is untouched and refs are
// rewritten, so F and C still hold — this mutant is isolated to clause R.
// Assembled with the preamble + gold ensures from reallocate.dfy.
function Reallocate(t: Tree, oldId: Id, newId: Id): Tree {
  seq(|t|, i requires 0 <= i < |t| =>
    Entity(t[i].id, RwRefs(t[i].refs, oldId, newId)))
}
