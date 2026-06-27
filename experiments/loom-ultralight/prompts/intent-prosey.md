`IsProsey(s)` decides whether a candidate entity/AC title string "looks like prose"
(and should be rejected in favour of a terse label). A title is **prosey** iff ANY
of the following triggers fires:

- **over-length**: `|s| > 80`;
- **newline**: `s` contains a `'\n'` or `'\r'`;
- **markdown**: `s` contains a markdown marker — `**`, `__`, or a backtick;
- **link bracket**: `s` contains the markdown-link sequence `](`;
- **multi-sentence**: `s` contains a sentence boundary — a sentence-ending mark
  (`'.'`, `'?'`, or `'!'`) immediately followed by a space and then a **capital**
  letter — occurring at least once. (`"Mr. smith"` is NOT a boundary — the letter
  after the space is lowercase; `"Done. Next item"` IS.)

If none of the triggers fire, the title is not prosey. The empty string is not
prosey. The helper predicates you need (sentence marks, uppercase test, substring
and pair scans) are provided in the Dafny context above; use them.
