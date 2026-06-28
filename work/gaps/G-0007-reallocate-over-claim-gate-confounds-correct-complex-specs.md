---
id: G-0007
title: reallocate over-claim gate confounds correct complex specs
status: open
discovered_in: M-0011
---
## What's missing

The `reallocate` over-claim §6 dimension (`1 − valid/extracted`) counts a spec as invalid
unless the validity gate can decide it's satisfiable by the reference impl. `M-0011`'s N=1
smoke on the `M-0012` sound gate (3 models × 2 arms) returned **3/6 `unexecutable`** — correct,
thorough disinterested specs marked invalid — from **three distinct causes** the hybrid gate
does not cover:

1. **Extraction overrun (`extract_spec_ensures`).** The extractor terminates the `ensures`
   region only at a line whose trimmed text starts with `{`. When the model's lemma body
   brace is not at line-start (e.g. the `opus-4.8` disinterested spec closed the lemma with a
   bare `}`), the extractor runs PAST the lemma and the closing ` ``` ` fence into the prose,
   so the assembled `.dfy` is unparseable → `unexecutable`. A pre-existing harness bug
   (mis-parses even otherwise-simple specs); the clear fix is to also terminate at a line
   starting with `}` or ` ``` `.
2. **Model-defined helpers not captured.** The harness assembles preamble + reference impl +
   the extracted `ensures`. A spec that defines and calls a helper (the `haiku-4.5`
   incentivized spec defined `function IndexOfId`) references an undefined symbol in the
   assembled program → resolution error → `unexecutable`.
3. **Unbounded guarded id-quantifiers.** Correct specs naturally express "no spurious ids" /
   "all old ids preserved" with `forall x: Id :: HasId(t, x) ==> …` (the `sonnet-4.6`
   disinterested spec). Neither `dafny verify` (empty-body) nor `dafny run` can discharge
   these: the Go backend cannot bound `x` from the `HasId` guard (confirmed), so they are a
   genuine ghost-only residual. A sound rewrite (guard → bounded iteration over the live
   id-set) would execute them, but is a non-trivial transform.

## Why it matters

Thorough disinterested specs are exactly the ones that use unbounded id-quantifiers, helpers,
and varied formatting — so the instrument **systematically penalizes the disinterested arm**,
inflating its over-claim rate and confounding the pre-registered over-claim comparison
(predicted to favour the *incentivized* arm). An N=30 run recorded on this instrument would be
confounded — the waste `M-0012`'s smoke discipline exists to prevent. `M-0012` (per `D-0003`)
fixed one validity class (bounded existentials / iff-characterizations, now `exec-valid`); the
earlier "6/6 invalid" smoke was in fact a MIX of these causes, so `M-0012` was necessary but
not sufficient. Addressed by `M-0013` (instrument hardening) before `M-0011`'s recorded run.
