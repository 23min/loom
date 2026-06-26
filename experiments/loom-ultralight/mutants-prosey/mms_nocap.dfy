// mutants-prosey/mms_nocap.dfy — "multi-sentence-imprecise": the multi-sentence
// boundary is WEAKENED to a sentence-mark-then-space, dropping the capital-letter
// requirement (HasMarkSpace instead of HasSentenceBoundary). So "Call foo. bar baz"
// (period+space+LOWERCASE) is now wrongly flagged prosey (breaks ms_needs_capital;
// killed by !IsProsey("Call foo. bar baz")). This is the predicted-tell IMPRECISION:
// the incentivized arm keeps a boundary rule but blurs its precision. Clause-isolated:
// ms_present's witness has a capital, so it stays prosey under either rule; the easy
// triggers don't depend on the boundary check.
predicate IsProsey(s: string)
{
  if s == "" then false
  else if |s| > 80 then true
  else if ContainsChar(s, '\n') || ContainsChar(s, '\r') then true
  else if ContainsPair(s, '*', '*') || ContainsPair(s, '_', '_') || ContainsChar(s, '`') then true
  else if ContainsPair(s, ']', '(') then true
  else HasMarkSpace(s)
}
