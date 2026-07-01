# Gap Report — id canonicalization (Loop 2)

`M-0015` / `E-0004` · **the audit trail (E3) of a two-rung tractability ladder on real string code.**

- **Verifier:** Dafny 4.9.0 / Z3 · **Subject:** real aiwf `canonicalize.go` @ v0.20.0 (121 lines) ·
  **Model + claims:** authored by two blind subagents (neither saw the other), so each rung is a
  genuine confrontation.
- **Rung 1 (flat `E-<digits>`)** — [`rung1.dfy`](rung1.dfy): **21 verified, 5 errors** (G1).
- **Rung 3 (composite `M/AC`)** — [`rung3-claims.dfy`](rung3-claims.dfy) over
  [`rung3-model.dfy`](rung3-model.dfy): **1 verified, 4 errors**; the model self-verifies 8/0 (G1).
- Rung 2 (six flat formats) deliberately skipped — same axis as rung 1, no new tractability signal.

## The two findings

Loop 2 produced **two** distinct results: a load-bearing **tractability** verdict, and a **value**
demonstration.

---

## § Tractability — the load-bearing verdict

Across a flat rung and a recursive rung, one thing held consistently, and it is the answer E-0004
was built to get:

**Modeling is tractable. Concrete-example checking is tractable. Blind universal-property
*discharge* is the wall — and a blind `(B)`-failure on strings stops being self-diagnosing.**

- **Modeling — tractable, flat *and* recursive.** The impl-modeler produced faithful raw-`seq<char>`
  Dafny for `E`-padding (rung 1) *and* for the composite recursion (rung 3, verified first try;
  termination via `decreases |id|` helped only by a one-line in-range-index postcondition, no
  timeouts, no heavy hints). The hypothesis that *recursion* would break modeling was **disproved**
  — running rung 3 is what established this.
- **Concrete examples — tractable.** Ground-evaluable witnesses discharge with fuel and give clean,
  unambiguous per-example verdicts (this is how both rungs surfaced their real gaps below).
- **Universal claims — the wall.** Rung 1 `Claim4` (value preserved) is *true* but does not
  discharge blind — it needs an induction hint that references the impl's structure, which a blind
  author cannot write. Rung 3 `Claim1`/`Claim2` fail the same way. **Crucially, the verifier reports
  the identical `"postcondition could not be proved"` whether a claim is *false* (a real gap —
  rung 1 `Claim1`/`Claim3`, rung 3 witnesses) or *true-but-blind-unprovable* (a tractability limit —
  rung 1 `Claim4`, rung 3 `Claim2`).**

On the FSM (Loop 1) every `(B)` was a decidable, self-diagnosing real gap. On strings — flat or
recursive — **you cannot mechanically tell a discovered discrepancy from a tractability limit
without body-aware work.** That degrades the loop's core differentiator (blind claims vs blind
model → mechanical, self-diagnosing verdict) on any string-heavy code. It is not about data
complexity (recursion didn't worsen it); it is specifically blind universal-proof over `seq<char>`.

## § Value — the gap report earned its keep (the code was the correct side)

Both rungs surfaced **real, non-obvious intent-vs-code divergences** via the concrete witnesses —
and, on independent check (a separate session), the operator **accepted the code and corrected
their intent.** The unifying cause: the operator's intent conflated **"emit"** with **"accept."**
The real rule (`allocate.go:14-18`) is **"emit wide, accept narrow"**: allocation/render always emit
canonical 4-wide (`aiwf add → M-0221`), but parsers *tolerate* narrower legacy widths, per-kind:

- `E- : ^E-\d{2,}$` (2+) — so `E-7` (1 digit) is **not a valid E id** → left untouched (not
  `E-0007`). *(rung 1 witness `E-7 → E-0007` failed.)*
- `M- : ^M-\d{3,}$` (3+) — so `M-7`, `M-14` are **below the floor** → not valid milestones → returned
  verbatim (not `M-0007`, `M-0014`). *(rung 3 both witnesses failed.)*
- `AC-\d+` — **no width rule**; the AC sub is a namespaced sub-element, never padded. *(rung 3 ex1
  `AC-3 → AC-0003` failed.)*
- 5+ digits are **valid and kept** (wide is fine), not illegal. `Canonicalize` re-pads only
  valid-but-narrow ids (`E-22 → E-0022`, `M-007 → M-0007`) on lookup.

**Intent-vs-intent, too:** the two rung-3 examples (`AC-3 → AC-0003` vs `AC-1 → AC-1`) are jointly
unsatisfiable under any deterministic sub-rule — the loop surfaced the operator's *own intent* as
internally inconsistent, a distinct kind of finding.

Set against Loop 1 (`M-0014`), where the divergence was a **code** issue the operator filed upstream,
Loop 2's divergences were **intent** errors the operator accepted — the two together are a clean
demonstration of loom's *bidirectional* discipline: the gap report makes divergence visible and
hands the human the call on which side is wrong.

## The four observations (M-0015 / AC-4)

- **Tractability (headline):** see § Tractability. Modeling + concrete-checking tractable (flat and
  recursive); blind universal-property discharge is the wall; a string `(B)`-failure is not
  self-diagnosing. Precise, and found on a 121-line function.
- **Faithfulness:** high — the impl-modeler grounded both models in `file:line`, ran a vacuity check
  (flipping AC-padding correctly broke a lemma), and cross-checked the composite model against the
  Go test vectors; the umbrella-author flagged the example inconsistency and its own silences.
- **Value:** demonstrated — real intent-vs-code divergences the operator independently confirmed and
  accepted (the emit/accept conflation), plus an intent-vs-intent inconsistency.
- **Effort:** the human wrote prose intent + a handful of examples and read English gap reports.
  **Zero Dafny, zero Go.** Per rung: two blind subagents + one assemble-and-verify.

## Follow-ups

- No aiwf gap: on independent review the operator judged the **code correct** and the **intent**
  mistaken (contrast `M-0014`, which surfaced a candidate aiwf gap). This is the honest, strongest
  form of the value finding.
- Feeds `E-0004`'s terminal decision: loom-with-Dafny reaches string-heavy real code for *modeling
  and concrete checks*, but its *universal-property* verification — its edge over tests — is not
  push-button there.
