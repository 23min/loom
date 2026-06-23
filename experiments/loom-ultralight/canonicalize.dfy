// canonicalize.dfy — the loom-ultralight subject + the gold contract.
//
// Subject: a faithful *model* of aiwf's entity-id canonicalization invariant
// (aiwf internal/entity/canonicalize.go + ADR-0008). An id has a kind, a
// numeric value, and a width (digits written). Canonicalization left-zero-pads
// the value to a MINIMUM of 4 digits; an id already >= 4 digits is unchanged;
// the kind and the numeric value NEVER change; it is idempotent.
//
// This file is the human-runnable calibration artifact:
//   dafny verify canonicalize.dfy   # GoldSpec + Idempotent must verify (AC-1)
//
// It is ALSO the single source the harness slices for the shared preamble, the
// reference implementation, and the gold spec's `ensures` clauses (the BEGIN/END
// sentinels below are load-bearing — the Rust harness greps for them).

// === BEGIN PREAMBLE ===
const PAD: nat := 4

// digits needed to write `value` (NumDigits(0) == 1)
function NumDigits(value: nat): nat { if value < 10 then 1 else 1 + NumDigits(value / 10) }

// a parsed entity id: kind tag, numeric value, and the width it is written at
datatype Id = Id(kind: string, value: nat, width: nat)

// input is wellformed if its width can actually hold its value
predicate Wellformed(x: Id) { x.width >= NumDigits(x.value) }
// === END PREAMBLE ===

// === BEGIN REFERENCE IMPL ===
// THE REFERENCE IMPLEMENTATION (correct)
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value, if x.width >= PAD then x.width else PAD)
}
// === END REFERENCE IMPL ===

// the gold spec, as a lemma over Canonicalize; `dafny verify` checks it.
lemma GoldSpec(x: Id)
  requires Wellformed(x)
// === BEGIN GOLD SPEC ENSURES ===
  ensures Canonicalize(x).kind  == x.kind                                   // (K) kind preserved
  ensures Canonicalize(x).value == x.value                                  // (V) value preserved  <-- the deep one
  ensures Canonicalize(x).width == (if x.width >= PAD then x.width else PAD) // (W) exact canonical width
  ensures Wellformed(Canonicalize(x))                                       // (F) output wellformed
// === END GOLD SPEC ENSURES ===
{ }

lemma Idempotent(x: Id)
  requires Wellformed(x)
  ensures Canonicalize(Canonicalize(x)) == Canonicalize(x)
{ }
