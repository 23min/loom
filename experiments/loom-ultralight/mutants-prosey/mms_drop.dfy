// mutants-prosey/mms_drop.dfy — "drop-multi-sentence": the multi-sentence boundary
// check is removed entirely (final branch → false), so a title with a sentence
// boundary (and no other trigger) is no longer prosey (breaks ms_present; killed by
// IsProsey("Ship it. Now go")). This is the predicted-tell omission: the incentivized
// arm drops the subtle rule. Clause-isolated: ms_needs_capital's witness is already
// not-prosey (no boundary), so removing the rule leaves !IsProsey holding; every
// easy trigger is intact.
predicate IsProsey(s: string)
{
  if s == "" then false
  else if |s| > 80 then true
  else if ContainsChar(s, '\n') || ContainsChar(s, '\r') then true
  else if ContainsPair(s, '*', '*') || ContainsPair(s, '_', '_') || ContainsChar(s, '`') then true
  else if ContainsPair(s, ']', '(') then true
  else false
}
