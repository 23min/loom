// RUNG 1 — E-<digits> canonicalization
// § does   : impl-modeler (blind), from real canonicalize.go @ v0.20.0
// § proves : umbrella-author (blind), from the human's intent (E-, EXACTLY 4 digits, 5+ illegal)

// ============================ MODEL (does) ============================
predicate Impl_IsDigit(c: char) { '0' <= c <= '9' }
predicate Impl_AllDigits(s: string) { forall i :: 0 <= i < |s| ==> Impl_IsDigit(s[i]) }
function Impl_DigitVal(c: char): nat { if '0' <= c <= '9' then (c as int) - ('0' as int) else 0 }
function Impl_ParseNat(s: string): nat { if |s| == 0 then 0 else Impl_ParseNat(s[..|s|-1]) * 10 + Impl_DigitVal(s[|s|-1]) }
function Impl_DigitChar(d: nat): char requires d < 10 { ((d + ('0' as int)) as char) }
function Impl_NatToString(n: nat): string decreases n { if n < 10 then [Impl_DigitChar(n)] else Impl_NatToString(n / 10) + [Impl_DigitChar(n % 10)] }
function Impl_Zeros(k: nat): string { if k == 0 then "" else ['0'] + Impl_Zeros(k - 1) }
function Impl_Pad4(n: nat): string { var s := Impl_NatToString(n); if |s| >= 4 then s else Impl_Zeros(4 - |s|) + s }

function canonicalize(id: string): string {
  if |id| == 0 then id
  else if |id| >= 2 && id[0] == 'E' && id[1] == '-' then
    var num := id[2..];
    if |num| == 0 then id
    else if !Impl_AllDigits(num) || |num| < 2 then id
    else if |num| >= 4 then id
    else "E-" + Impl_Pad4(Impl_ParseNat(num))
  else id
}

// ============================ CLAIMS (proves) ============================
predicate Spec_isDigit(c: char) { '0' <= c <= '9' }
predicate Spec_allDigits(s: string) { forall i :: 0 <= i < |s| ==> Spec_isDigit(s[i]) }
predicate Spec_startsWithEDash(s: string) { |s| >= 2 && s[0] == 'E' && s[1] == '-' }
function Spec_body(id: string): string { if |id| >= 2 then id[2..] else "" }
function Spec_digitCount(id: string): nat { |Spec_body(id)| }
function Spec_charVal(c: char): nat { if Spec_isDigit(c) then (c as int - '0' as int) else 0 }
function Spec_numValue(s: string): nat { if |s| == 0 then 0 else Spec_numValue(s[..|s|-1]) * 10 + Spec_charVal(s[|s|-1]) }
predicate Spec_legal(id: string) { Spec_startsWithEDash(id) && Spec_allDigits(Spec_body(id)) && 1 <= Spec_digitCount(id) <= 4 }
predicate Spec_isCanonical(id: string) { Spec_startsWithEDash(id) && Spec_allDigits(Spec_body(id)) && Spec_digitCount(id) == 4 }

function Spec_zeros(n: nat): string { if n == 0 then "" else ['0'] + Spec_zeros(n-1) }
lemma Spec_ZerosValueZero(z: string) requires forall i :: 0 <= i < |z| ==> z[i] == '0' ensures Spec_numValue(z) == 0 { if |z| == 0 { } else { Spec_ZerosValueZero(z[..|z| - 1]); } }
lemma Spec_ZeroPadValue(z: string, s: string) requires forall i :: 0 <= i < |z| ==> z[i] == '0' ensures Spec_numValue(z + s) == Spec_numValue(s) {
  if |s| == 0 { assert z + s == z; Spec_ZerosValueZero(z); }
  else { assert (z + s)[..|z + s| - 1] == z + s[..|s| - 1]; assert (z + s)[|z + s| - 1] == s[|s| - 1]; Spec_ZeroPadValue(z, s[..|s| - 1]); }
}

lemma Claim1_CanonicalShape(id: string) requires Spec_legal(id) ensures Spec_isCanonical(canonicalize(id)) { }
lemma Claim2_PrefixPreserved(id: string) requires Spec_legal(id) ensures Spec_startsWithEDash(canonicalize(id)) { }
lemma Claim3_ExactlyFourDigits(id: string) requires Spec_legal(id) ensures Spec_digitCount(canonicalize(id)) == 4 { }
lemma Claim4_ValuePreserved(id: string) requires Spec_legal(id) ensures Spec_numValue(Spec_body(canonicalize(id))) == Spec_numValue(Spec_body(id)) { }
lemma Claim5_AlreadyCanonicalUnchanged(id: string) requires Spec_legal(id) requires Spec_isCanonical(id) ensures canonicalize(id) == id { }

lemma {:fuel Impl_ParseNat,6} {:fuel Impl_NatToString,6} {:fuel Impl_Zeros,6} {:fuel Impl_AllDigits,6} {:fuel canonicalize,4}
ClaimExamples()
  ensures canonicalize("E-7")    == "E-0007"   // human intent
  ensures canonicalize("E-42")   == "E-0042"
  ensures canonicalize("E-123")  == "E-0123"
  ensures canonicalize("E-0")    == "E-0000"   // human intent
  ensures canonicalize("E-0001") == "E-0001"
  ensures canonicalize("E-1234") == "E-1234"
{ }
