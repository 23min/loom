// mutants-prosey/mmd.dfy — "drop-markdown": the markdown-marker check (** / __ / `)
// is removed, so a title with bold/code markup (and no other trigger) is no longer
// prosey (breaks markdown; killed by IsProsey("Use **bold** here")). Clause-isolated:
// no other witness contains a markdown marker — length/newline/link/multi-sentence
// are unaffected.
predicate IsProsey(s: string)
{
  if s == "" then false
  else if |s| > 80 then true
  else if ContainsChar(s, '\n') || ContainsChar(s, '\r') then true
  else if ContainsPair(s, ']', '(') then true
  else HasSentenceBoundary(s)
}
