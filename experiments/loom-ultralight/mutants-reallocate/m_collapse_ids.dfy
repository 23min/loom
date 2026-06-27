// mutants-reallocate/m_collapse_ids.dfy — "clobbers the frame": maps every entity
// id to newId, so every NON-target entity is changed (breaks F, the frame). The
// target is still renamed to newId (R holds) and references are rewritten (C holds)
// — this mutant is isolated to clause F. (A tree with >= 2 entities exhibits the
// frame violation; Dafny finds that counterexample.)
// Assembled with the preamble + gold ensures from reallocate.dfy.
function Reallocate(t: Tree, oldId: Id, newId: Id): Tree {
  seq(|t|, i requires 0 <= i < |t| =>
    Entity(newId, RwRefs(t[i].refs, oldId, newId)))
}
