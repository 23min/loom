// mutants/M19.dfy — "shrink-wide": shrinks an already-canonical id to its minimum
// wellformed width instead of leaving it unchanged (killed by EXACT width, and
// only that). Stays wellformed (width >= NumDigits(value)) and >= PAD, so only a
// spec that pins width to max(x.width, PAD) catches the lost digits.
function Canonicalize(x: Id): Id {
  Id(x.kind, x.value, if x.width >= PAD then (if NumDigits(x.value) >= PAD then NumDigits(x.value) else PAD) else PAD)
}
