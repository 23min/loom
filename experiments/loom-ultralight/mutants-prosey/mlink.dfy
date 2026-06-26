// mutants-prosey/mlink.dfy — "drop-link-bracket": the markdown-link check (`](`) is
// removed, so a title with a link (and no other trigger) is no longer prosey (breaks
// link_bracket; killed by IsProsey("See [docs](url)")). Clause-isolated: no other
// witness contains `](` — length/newline/markdown/multi-sentence are unaffected.
predicate IsProsey(s: string)
{
  if s == "" then false
  else if |s| > 80 then true
  else if ContainsChar(s, '\n') || ContainsChar(s, '\r') then true
  else if ContainsPair(s, '*', '*') || ContainsPair(s, '_', '_') || ContainsChar(s, '`') then true
  else HasSentenceBoundary(s)
}
