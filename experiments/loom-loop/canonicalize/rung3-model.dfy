// RUNG 3 — IMPL MODEL of aiwf Canonicalize for COMPOSITE ids (string level).
// Faithful to internal/entity/canonicalize.go + entity.go @ aiwf v0.20.0.
//
// Shared interface (DO NOT change signature):
//   function canonicalize(id: string): string
// composite ids look like "M-014/AC-1".

// ---------------------------------------------------------------------------
// character / digit helpers
// ---------------------------------------------------------------------------

predicate Impl_IsDigit(c: char) { '0' <= c <= '9' }

predicate Impl_AllDigits(s: string) {
  forall i :: 0 <= i < |s| ==> Impl_IsDigit(s[i])
}

// ---------------------------------------------------------------------------
// per-kind grammars the recursion needs
//
// Milestone parent grammar  (entity.go:197)  ^M-\d{3,}$
// AC sub-id grammar         (entity.go:227)  AC-\d+   (>=1 digit, NO floor)
// ---------------------------------------------------------------------------

predicate Impl_IsMilestoneID(s: string) {
  |s| >= 5                    // "M-" + at least 3 digits
  && s[0] == 'M' && s[1] == '-'
  && Impl_AllDigits(s[2..])
}

predicate Impl_IsACID(s: string) {
  |s| >= 4                    // "AC-" + at least 1 digit
  && s[0] == 'A' && s[1] == 'C' && s[2] == '-'
  && Impl_AllDigits(s[3..])
}

// ---------------------------------------------------------------------------
// flat per-kind padding for the milestone parent (canonicalize.go:44-72).
//
// CanonicalPad = 4 (entity.go:19). For a matching M id:
//   num := s[2..]
//   if len(num) >= 4      -> return s unchanged
//   else                  -> return fmt.Sprintf("M-%04d", Atoi(num))
//
// The pad branch is reached only when len(num) < 4 AND s matched ^M-\d{3,}$,
// so len(num) == 3 exactly. For a 3-digit all-digit string with value v
// (0..999): num == %03d(v) uniquely, and %04d(v) == "0" + %03d(v) == "0"+num.
// So `fmt.Sprintf("M-%04d", Atoi(num))` == "M-0" + num for every reachable
// input. We model that exact string result (no Atoi/Sprintf needed).
// ---------------------------------------------------------------------------

function Impl_bareCanonMilestone(s: string): string
  requires Impl_IsMilestoneID(s)
{
  var num := s[2..];
  if |num| >= 4 then s        // already canonical or wider: unchanged
  else "M-0" + num            // |num| == 3; %04d re-pad == prepend one '0'
}

// ---------------------------------------------------------------------------
// ParseCompositeID (entity.go:227,239)  ^(M-\d{3,})/(AC-\d+)$
//
// A valid composite has EXACTLY one '/': neither the parent (M-\d{3,}) nor the
// sub (AC-\d+) may contain '/', so the regex's single literal '/' is the only
// '/' in the string, hence necessarily the FIRST one. Scanning the first '/'
// and grammar-checking both sides is therefore exactly faithful.
// ---------------------------------------------------------------------------

datatype Impl_Option<T> = None | Some(value: T)

function Impl_IndexOfSlash(s: string, i: nat): int
  requires i <= |s|
  decreases |s| - i
  ensures Impl_IndexOfSlash(s, i) == -1 || i <= Impl_IndexOfSlash(s, i) < |s|
{
  if i == |s| then -1
  else if s[i] == '/' then i
  else Impl_IndexOfSlash(s, i + 1)
}

function Impl_ParseComposite(id: string): Impl_Option<(string, string)>
  // parent is a strict prefix ending before '/', so it is strictly shorter —
  // this postcondition is what discharges canonicalize's termination.
  ensures Impl_ParseComposite(id).Some? ==>
            |Impl_ParseComposite(id).value.0| < |id|
{
  var k := Impl_IndexOfSlash(id, 0);
  if k < 0 then None
  else
    var parent := id[..k];
    var sub := id[k + 1..];
    if Impl_IsMilestoneID(parent) && Impl_IsACID(sub)
    then Some((parent, sub))
    else None
}

// ---------------------------------------------------------------------------
// Canonicalize (canonicalize.go:32-74)
//
//   if id == "" { return id }
//   if parent, sub, ok := ParseCompositeID(id); ok {
//       canonParent := Canonicalize(parent)          // RECURSE on parent
//       if canonParent == parent { return id }
//       return canonParent + "/" + sub               // sub left ALONE
//   }
//   ... bare per-kind path (here: milestone) ...
//   return id
// ---------------------------------------------------------------------------

function canonicalize(id: string): string
  decreases |id|
{
  if id == "" then id
  else
    var pc := Impl_ParseComposite(id);
    if pc.Some? then
      var parent := pc.value.0;
      var sub := pc.value.1;
      // |parent| < |id| (Impl_ParseComposite ensures) => decreases holds
      var canonParent := canonicalize(parent);
      if canonParent == parent then id            // unchanged parent: return original
      else canonParent + "/" + sub                // rejoin; sub verbatim
    else if Impl_IsMilestoneID(id) then
      Impl_bareCanonMilestone(id)                 // bare milestone re-pad
    else
      id                                          // everything else: verbatim
}

// ---------------------------------------------------------------------------
// Executable regression check: pin the exact real-code outputs (mirrors
// canonicalize_test.go). `dafny verify` proves these lemmas -> the model
// agrees with the Go test vectors on every listed input.
// ---------------------------------------------------------------------------

lemma Impl_Regression()
  ensures canonicalize("") == ""
  ensures canonicalize("hello") == "hello"
  // bare milestone
  ensures canonicalize("M-007") == "M-0007"
  ensures canonicalize("M-0007") == "M-0007"
  ensures canonicalize("M-22") == "M-22"        // below \d{3,} floor: verbatim
  ensures canonicalize("M-99999") == "M-99999"  // wider than canonical: verbatim
  // composite: recurse on parent, AC sub left ALONE (never padded)
  ensures canonicalize("M-007/AC-1") == "M-0007/AC-1"
  ensures canonicalize("M-007/AC-12") == "M-0007/AC-12"   // AC-12 unchanged
  ensures canonicalize("M-0007/AC-1") == "M-0007/AC-1"    // canonical parent
  ensures canonicalize("M-22/AC-1") == "M-22/AC-1"        // below-floor parent -> verbatim
  // the prompt's example ids (parents below the \d{3,} floor) -> verbatim
  ensures canonicalize("M-7/AC-3") == "M-7/AC-3"
  ensures canonicalize("M-14/AC-1") == "M-14/AC-1"
{
}
