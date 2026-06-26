// mutants-prosey/mnl.dfy — "drop-newline": the embedded-newline check is removed, so
// a title with a \n (and no other trigger) is no longer prosey (breaks newline;
// killed by IsProsey("Line one\nLine two")). Clause-isolated: no other witness
// contains a newline, so length/markdown/link/multi-sentence are unaffected.
predicate IsProsey(s: string)
{
  if s == "" then false
  else if |s| > 80 then true
  else if ContainsPair(s, '*', '*') || ContainsPair(s, '_', '_') || ContainsChar(s, '`') then true
  else if ContainsPair(s, ']', '(') then true
  else HasSentenceBoundary(s)
}
