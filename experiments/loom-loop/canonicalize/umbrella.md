# Umbrella — id canonicalization

*A loom umbrella (five registers, [language-reference.md](../../../docs/reference/language-reference.md) §4), authored across two rungs of a tractability ladder. The formal `does`/`proves` bodies live in the verifiable Dafny — [`rung1.dfy`](rung1.dfy) (flat `E-<digits>`) and [`rung3-model.dfy`](rung3-model.dfy) + [`rung3-claims.dfy`](rung3-claims.dfy) (composite `M/AC`). Findings, the tractability verdict, and the corrected understanding are in [`gap-report.md`](gap-report.md).*

## Rung 1 — flat `E-<digits>`

### `intent` *(human — as authored; **later found wrong**, see gap-report § Value)*

The canonical form is `E-NNNN` — `E`, `-`, **exactly 4 digits**. Fewer than 4 → front-pad with `0`s
(value preserved); already 4 → unchanged; **5+ digits → illegal.** *(This conflated "emit" with
"accept" — the real rule is "emit wide, accept narrow"; the loop caught it. See gap-report.)*

### `shows` *(human)* — full in `rung1.dfy`
`E-7 → E-0007` · `E-42 → E-0042` · `E-0001 → E-0001` · `E-12345 → ILLEGAL`

### `does` / `proves` — formal bodies in `rung1.dfy`
`does` = the real `E`-canonicalize modeled at raw `seq<char>`. `proves` = c1 canonical shape · c2
prefix preserved · c3 exactly-4-digits · c4 value preserved · c5 already-canonical unchanged.

### Back-translation *(LLM `summarize`)*
c1 a legal id canonicalizes to `E-` + exactly 4 digits · c2 the `E-` prefix is preserved · c3 the
output has exactly 4 digits · c4 the numeric value is preserved · c5 an already-canonical id is
unchanged.

## Rung 3 — composite `M/AC`

### `intent` *(human — as authored; **later found wrong**, and internally inconsistent)*

A composite id is a parent and a sub joined by `/` (`M-14/AC-1`). Canonicalize the parent
(recursively) and the sub — front-padding each number to 4 digits — and rejoin with `/`.

### `shows` *(human)* — the two examples **disagree on the sub-id** (a flagged inconsistency)
`M-7/AC-3 → M-0007/AC-0003` (sub padded) · `M-14/AC-1 → M-0014/AC-1` (sub unchanged)

### `does` / `proves` — formal bodies in `rung3-model.dfy` + `rung3-claims.dfy`
`does` = the real composite recursion (split on `/`, recurse on parent, sub verbatim), modeled and
verified. `proves` = Claim1 structural distribution · Claim2 composite-shape preserved · the two
example witnesses.

### Back-translation *(LLM `summarize`)*
Claim1 canonicalizing a composite distributes over the split (parent recurses, sub independent) ·
Claim2 a composite stays composite · the witnesses assert the two human examples.

### `gap` *(the honest limitation, both rungs)*
The load-bearing gap is **not** in the umbrella's content but in the *medium*: on strings, the
umbrella's `proves` do not discharge push-button against a blind `does` — see gap-report §
Tractability. This is a property of Dafny-on-strings, surfaced not hidden.
