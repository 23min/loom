// prosey.dfy — the prosey-title detection subject + the gold contract.
//
// Subject: a faithful model of aiwf's IsProseyTitle (internal/entity/entity.go).
// A title is "prosey" (rejected by `aiwf add ac`) if ANY of five triggers fire:
// over-length (>80), an embedded newline, a markdown marker (** / __ / `), a link
// bracket (`](`), or a multi-sentence boundary (a .?! followed by a space and a
// CAPITAL, occurring at least once). The load-bearing obligation is the
// MULTI-SENTENCE rule: the `>= 1` threshold and the space-and-capital precision are
// exactly where a spec written to be trivially-satisfiable blurs the boundary.
//
//   dafny verify prosey.dfy   # GoldSpec must verify (M-0005 AC-1)
//
// This file is ALSO the single source the harness slices for the shared preamble,
// the reference implementation, and the gold spec's `ensures` clauses (the
// BEGIN/END sentinels below are load-bearing — the Rust harness greps for them).
//
// Encoding note: the string helpers are RECURSIVE (not `exists`) so Dafny evaluates
// them on concrete literal witnesses by ground unfolding, keeping every probe in the
// decidable regime. The `{:fuel}` attributes raise the default unrolling (1–2) high
// enough to scan the short witnesses to the end — the only place deep unrolling is
// needed is proving a "contains" predicate FALSE over a whole negative witness.
//
// Faithfulness deviation: Go's length check is `len(title) > 80` (BYTES); this models
// `|s| > 80` (CHARS). The two coincide for the ASCII witnesses used here; multibyte
// length is not load-bearing for the multi-sentence tell this subject probes.

// === BEGIN PREAMBLE ===
predicate IsSentenceMark(c: char) { c == '.' || c == '?' || c == '!' }
predicate IsUpper(c: char) { 'A' <= c <= 'Z' }

// A single char appears anywhere in s.
predicate {:fuel 12, 12} ContainsChar(s: string, c: char)
{
  |s| >= 1 && (s[0] == c || ContainsChar(s[1..], c))
}

// An adjacent pair (a, b) appears anywhere in s.
predicate {:fuel 12, 12} ContainsPair(s: string, a: char, b: char)
{
  |s| >= 2 && ((s[0] == a && s[1] == b) || ContainsPair(s[1..], a, b))
}

// At least one sentence boundary: a sentence-mark, then a space, then a capital.
// Mirrors the Go rune-walk window `for i := 0; i < len-2; i++` (needs 3 runes).
predicate {:fuel 12, 12} HasSentenceBoundary(s: string)
{
  |s| >= 3 && ((IsSentenceMark(s[0]) && s[1] == ' ' && IsUpper(s[2])) || HasSentenceBoundary(s[1..]))
}

// A sentence-mark followed by a space, IGNORING the capital requirement. The
// reference does NOT use this — it is the capital-blind boundary the `mms_nocap`
// mutant substitutes to break the `ms_needs_capital` precision obligation. Declared
// in the shared preamble so the mutant bank (which carries only an IsProsey body)
// can reference it; unused by the reference impl and every other mutant.
predicate {:fuel 12, 12} HasMarkSpace(s: string)
{
  |s| >= 2 && ((IsSentenceMark(s[0]) && s[1] == ' ') || HasMarkSpace(s[1..]))
}
// === END PREAMBLE ===

// === BEGIN REFERENCE IMPL ===
// THE REFERENCE IMPLEMENTATION (correct) — transcribed from IsProseyTitle:
//   empty → false; then OR of: len>80, newline, markdown marker, link bracket,
//   multi-sentence boundary.
predicate IsProsey(s: string)
{
  if s == "" then false
  else if |s| > 80 then true
  else if ContainsChar(s, '\n') || ContainsChar(s, '\r') then true
  else if ContainsPair(s, '*', '*') || ContainsPair(s, '_', '_') || ContainsChar(s, '`') then true
  else if ContainsPair(s, ']', '(') then true
  else HasSentenceBoundary(s)
}
// === END REFERENCE IMPL ===

// the gold spec, as a lemma over IsProsey; `dafny verify` checks it.
lemma GoldSpec()
// === BEGIN GOLD SPEC ENSURES ===
  // (over_length) ANY title past 80 chars is prosey. A `forall` (not a witness):
  // the length branch short-circuits before any recursive scan, so this is cheap to
  // prove for the reference and fails fast — symbolically, with no 81-char literal to
  // churn over — against the length-dropping mutant. It is also the more faithful
  // statement of the check.
  ensures forall s: string :: |s| > 80 ==> IsProsey(s)
  // The remaining checks need the reference's recursive scan, which Dafny can only
  // ground-evaluate on a concrete literal — so each is a MINIMAL witness (3–6 chars).
  // Short keeps the all-clauses-false scan (needed to prove a negative, or a positive
  // false under its dropping mutant) shallow and fast; realism is irrelevant — the
  // witness only has to trigger exactly its own check and nothing else.
  // (newline) an embedded newline is prosey
  ensures IsProsey("a\nb")
  // (markdown) a markdown marker is prosey
  ensures IsProsey("a**b")
  // (link_bracket) a markdown link is prosey
  ensures IsProsey("a](b")
  // (ms_present) a single sentence boundary makes it prosey  <-- the tell (threshold >= 1)
  ensures IsProsey("Go. Up")
  // (ms_needs_capital) period+space+LOWERCASE is NOT a boundary, so not prosey  <-- the tell (precision)
  ensures !IsProsey("Go. up")
// === END GOLD SPEC ENSURES ===
{ }
