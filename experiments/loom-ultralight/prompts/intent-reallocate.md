A planning tree is a sequence of entities. Each entity has an `id` and a sequence
of `refs` — the ids of other entities it points to (its cross-references). Ids are
unique across the tree.

`Reallocate(t, oldId, newId)` renames one entity and repairs the tree around it: it
must return the tree in which the entity whose id is `oldId` now has id `newId`, and
**every** reference to `oldId` — anywhere in any entity's `refs` — has been rewritten
to `newId`. It is called only when `oldId` is present, `newId` is absent (a fresh id),
and the tree's ids are unique.

The contract is the three structural invariants reallocation must preserve:

- **No orphan**: after reallocation, no entity is left holding `oldId` — neither as
  its own id nor anywhere it is referenced.
- **Uniqueness**: ids remain unique — the rename introduces no collision.
- **Complete rewrite**: every cross-reference to `oldId` becomes `newId`, in every
  entity, not only in the entity being renamed. An entity elsewhere in the tree that
  *referenced* `oldId` must now reference `newId`.

The complete cross-reference rewrite is as much part of the contract as the rename
itself: a reallocation that renames the entity but leaves a dangling reference to the
old id behind is wrong.
