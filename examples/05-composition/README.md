# 05 — Composition (two umbrellas)

This example demonstrates **cross-umbrella composition**: how a parent umbrella
states *system-level* guarantees in terms of what its children prove, and where
the discipline currently stops.

It is the worked example for [`docs/compositional-correctness.md`](../../docs/reference/compositional-correctness.md),
which explains the design in full. Read that doc for the analysis; this README
is just the map.

## The three files

| File | Role | Proves |
|---|---|---|
| [`ledger.lm`](ledger.lm) | child A — money | `conservation`, `no_overdrafts` |
| [`audit.lm`](audit.lm) | child B — audit log | `append_only`, `records_the_entry`, `length_grows_by_one` |
| [`bank.lm`](bank.lm) | parent — the system | `system_conserves_money`; **summarizes** `funds_conserved_and_audited` (A) and `audit_is_complete` (gap) |

## What it shows

1. **Composition that discharges.** `bank.funds_conserved_and_audited`
   (in the `summarizes` register) follows from `ledger.conservation`,
   `audit.records_the_entry`, and `audit.length_grows_by_one` — the parent
   assumes the child claims rather than re-proving them.

2. **The load-bearing gap.** `bank.audit_is_complete` — "the ledger and the
   audit log can never silently drift apart" — is the property you would most
   want against an agent that quietly edits one path and forgets the other.
   v0 cannot express it (it is a property of the operation *trace*, not of one
   pure call), so it is admitted as an explicit `gap` and surfaces as
   category-(B). The point of Loom is that this gap is **loud and named**, not
   silent.

3. **Gap propagation / trust roots.** The (A) result is sound only because the
   children are themselves fully verified. If a child claim it depends on were
   category-(B), the parent claim would degrade to (B) too.

## Status

The `summarizes` register and cross-umbrella discharge are **post-v0** (see
`docs/compositional-correctness.md` §6 and `docs/bidirectional-refinement.md`
§9). These `.lm` files are specifications of the intended feature, not yet
runnable through `loom verify`. List-literal syntax (`[...]`) and `Log.insert`
are provisional v0 surface details; the proofs depend only on `.length` and
`.contains`.
