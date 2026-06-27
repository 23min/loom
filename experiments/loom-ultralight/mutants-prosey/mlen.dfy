// mutants-prosey/mlen.dfy — "drop-over-length": the `|s| > 80` check is removed, so
// a long-but-otherwise-clean title is no longer prosey (breaks over_length; killed by
// the `forall s :: |s| > 80 ==> IsProsey(s)` obligation, which now fails for a long
// trigger-free string). Clause-isolated: every other witness is short, so the length
// check never fired for them — newline/markdown/link/multi-sentence intact.
predicate IsProsey(s: string)
{
  if s == "" then false
  else if ContainsChar(s, '\n') || ContainsChar(s, '\r') then true
  else if ContainsPair(s, '*', '*') || ContainsPair(s, '_', '_') || ContainsChar(s, '`') then true
  else if ContainsPair(s, ']', '(') then true
  else HasSentenceBoundary(s)
}
