// mutants-reallocate/m_partial_refs.dfy — "forgot the distant references": rewrites
// references only inside the entity being renamed, leaving every OTHER entity's refs
// untouched (breaks C). This is the realistic under-rewrite — the direct analog of a
// hand `git mv` that fixes the renamed entity but orphans the cross-references
// pointing at it from elsewhere — a sharper C-violator than m_keep_refs. The id map
// is correct, so R and F still hold; isolated to clause C.
// Assembled with the preamble + gold ensures from reallocate.dfy.
function Reallocate(t: Tree, oldId: Id, newId: Id): Tree {
  seq(|t|, i requires 0 <= i < |t| =>
    Entity(Rw(t[i].id, oldId, newId),
           if t[i].id == oldId then RwRefs(t[i].refs, oldId, newId) else t[i].refs))
}
