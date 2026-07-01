// RUNG 3 — composite ids: CLAIMS (umbrella-author, blind), verified against the
// MODEL in rung3-model.dfy (impl-modeler, blind).  Run: dafny verify rung3-claims.dfy
// Expected: 1 verified, 4 errors — all four claims fail; see ../canonicalize/gap-report.md.
include "rung3-model.dfy"

predicate Spec_ContainsSlash(s: string) { exists i :: 0 <= i < |s| && s[i] == '/' }

// Claim 1 (structural / recursive): canonicalize distributes over the split.
// FAILS — real over-generalization: the grammar allows only ONE level (M/AC), so the
// distributive law is false on a composite parent (see gap-report § tractability).
lemma Claim1_CompositeStructural(parent: string, sub: string)
  requires |parent| > 0
  requires |sub| > 0
  requires !Spec_ContainsSlash(sub)
  ensures canonicalize(parent + "/" + sub) == canonicalize(parent) + "/" + canonicalize(sub)
{ }

// Claim 2 (weaker structural): a composite stays composite.
// FAILS — but TRUE: blind-unprovable without a body-aware reveal (the tractability wall).
lemma Claim2_PreservesCompositeShape(parent: string, sub: string)
  requires |parent| > 0
  requires |sub| > 0
  requires !Spec_ContainsSlash(sub)
  ensures Spec_ContainsSlash(canonicalize(parent + "/" + sub))
{ }

// Witnesses — the human's two examples, verbatim. BOTH FAIL: the code returns them
// unchanged (M-7, M-14 are below the M \d{3,} accept floor; AC is never padded).
lemma {:fuel canonicalize,6} {:fuel Impl_ParseComposite,6} {:fuel Impl_IndexOfSlash,12} {:fuel Impl_AllDigits,12}
Witness_Example1_bothPadded()
  ensures canonicalize("M-7/AC-3") == "M-0007/AC-0003"
{ }

lemma {:fuel canonicalize,6} {:fuel Impl_ParseComposite,6} {:fuel Impl_IndexOfSlash,12} {:fuel Impl_AllDigits,12}
Witness_Example2_subUnchanged()
  ensures canonicalize("M-14/AC-1") == "M-0014/AC-1"
{ }
