// reallocate.dfy — the loom-ultralight id-reallocation subject + the gold contract.
//
// Subject: a faithful *model* of aiwf's `reallocate` invariant (rename an entity
// id and rewrite every cross-reference to it, per the CLAUDE.md "On an id
// collision, run `aiwf reallocate`" rule). A tree is a sequence of entities, each
// with an id and a sequence of ids it references. Reallocation renames the entity
// whose id is `oldId` to `newId` and rewrites every `oldId` reference to `newId`,
// everywhere — preserving id-uniqueness and leaving no orphaned `oldId`.
//
// This file is the human-runnable calibration artifact:
//   dafny verify reallocate.dfy   # GoldSpec must verify (M-0009 AC-1)
//
// It is ALSO the single source the harness slices for the shared preamble, the
// reference implementation, and the gold spec's `ensures` clauses (the BEGIN/END
// sentinels below are load-bearing — the Rust harness greps for them). The gold
// ensures are stated so Dafny discharges them with an EMPTY lemma body (the
// harness wraps them in `lemma Spec(...) { }`), and so each clause is a single
// expression reusable verbatim as a strength-probe obligation goal.

// === BEGIN PREAMBLE ===
// An entity identifier. `int` keeps the model inside Z3's decidable regime; the
// invariant is about id *equality and rewriting*, never id arithmetic.
type Id = int

// An entity: its own id, and the ids it references (cross-references / parents).
datatype Entity = Entity(id: Id, refs: seq<Id>)

// A planning tree is a sequence of entities.
type Tree = seq<Entity>

// `x` is present as some entity's id.
predicate HasId(t: Tree, x: Id) {
  exists i :: 0 <= i < |t| && t[i].id == x
}

// Ids are unique across the tree (no two entities share an id).
predicate Valid(t: Tree) {
  forall i, j :: 0 <= i < |t| && 0 <= j < |t| && i != j ==> t[i].id != t[j].id
}

// Rewrite a single id: `oldId` becomes `newId`, everything else is unchanged.
function Rw(r: Id, oldId: Id, newId: Id): Id { if r == oldId then newId else r }

// Rewrite every id in a reference sequence.
function RwRefs(rs: seq<Id>, oldId: Id, newId: Id): seq<Id> {
  seq(|rs|, i requires 0 <= i < |rs| => Rw(rs[i], oldId, newId))
}
// === END PREAMBLE ===

// === BEGIN REFERENCE IMPL ===
// THE REFERENCE IMPLEMENTATION (correct): relabel each entity's id and rewrite
// every reference, pointwise across the tree.
function Reallocate(t: Tree, oldId: Id, newId: Id): Tree {
  seq(|t|, i requires 0 <= i < |t| =>
    Entity(Rw(t[i].id, oldId, newId), RwRefs(t[i].refs, oldId, newId)))
}
// === END REFERENCE IMPL ===

// the gold spec, as a lemma over Reallocate; `dafny verify` checks it.
//
// The contract is the COMPLETE pointwise pin of reallocation: the renamed entity
// becomes newId (R), every other id is unchanged (F), and every reference is
// rewritten (C). The three clauses are mutually INDEPENDENT — each is violable on
// its own (the mutant bank has an impl per clause) — and together they uniquely
// determine the output, so the gold is a *complete* contract: an impl that renames
// the target to the wrong id, or alters an unrelated entity, is rejected. The two
// structural invariants reallocation is known for — no orphaned old id, and
// preserved id-uniqueness — are *consequences* of the pin, proven in
// `StructuralInvariantsFollow` below and deliberately NOT sliced as obligations:
// stated alongside the pin they would be redundant (the pin entails them). C — the
// complete cross-reference rewrite — is the predicted "tell"; R and F are the control.
lemma GoldSpec(t: Tree, oldId: Id, newId: Id)
  requires oldId != newId
  requires Valid(t)
  requires HasId(t, oldId)
  requires !HasId(t, newId)
// === BEGIN GOLD SPEC ENSURES ===
  ensures forall i :: 0 <= i < |t| && t[i].id == oldId ==> Reallocate(t, oldId, newId)[i].id == newId           // (R) the renamed entity becomes newId  <-- control
  ensures forall i :: 0 <= i < |t| && t[i].id != oldId ==> Reallocate(t, oldId, newId)[i].id == t[i].id         // (F) every other id is unchanged        <-- control
  ensures forall i :: 0 <= i < |t| ==> Reallocate(t, oldId, newId)[i].refs == RwRefs(t[i].refs, oldId, newId)   // (C) every reference is rewritten       <-- the tell
// === END GOLD SPEC ENSURES ===
{ }

// The two structural invariants reallocation preserves — no orphaned old id, and
// preserved id-uniqueness — FOLLOW from the complete pin {R, F} above. Proven here
// as human-reader theorems (not sliced by the harness): stating them as obligations
// alongside the pin would be redundant, since the pin entails them.
lemma StructuralInvariantsFollow(t: Tree, oldId: Id, newId: Id)
  requires oldId != newId
  requires Valid(t)
  requires HasId(t, oldId)
  requires !HasId(t, newId)
  ensures !HasId(Reallocate(t, oldId, newId), oldId)  // no orphaned oldId
  ensures Valid(Reallocate(t, oldId, newId))          // id-uniqueness preserved
{ }
