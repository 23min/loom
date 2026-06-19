---
id: ADR-0018
title: Spec-implementation binding — option space (decision deferred)
status: proposed
---

# ADR-0018 — Spec↔implementation binding — option space (decision deferred)

> **Date:** 2026-06-19 · **Deciders:** project initial author; open for review.
> **Status note:** options recorded; the decision is **open**, deferred to loom-light implementation evidence. aiwf ADR status `proposed` is the closest fit for "open for decision".
> **Stage:** a **loom-light** decision (see [`docs/loom-light.md`](../loom-light.md)), deferred.
> **Related:** ADR-0017 (loom-light: no codegen; `does`-form deferred — the same question from the other side), ADR-0002 (Dafny verifier), `PLAN.md` §1.1 (siblings), §2.3 (bidirectional refinement / gap report), §4.4; `docs/loom-light.md`.

---

## Context

ADR-0017 leaves the form and role of `does` open and scopes loom-light to verification with no codegen. The implementation that satisfies an umbrella therefore lives in a **sibling** — Dafny for the verified core, or the consumer's host language for production code — and *how an umbrella is bound to its sibling* is undecided. That mechanism is this ADR's subject.

It is **not** decided here. There is no code yet, and the binding mechanism should be chosen against implementation evidence rather than up front — committing early and reversing later is the costly-pivot pattern the project avoids. This ADR records (a) the invariants any mechanism must satisfy — the genuinely-decided part — and (b) the valid options with trade-offs, so the eventual decision is made against a recorded option space rather than silence (`PLAN.md` §0).

The form of `does` (ADR-0017) and the binding mechanism are the **same question from two sides**: `does: ref ledger.dfy` *is* a binding expressed in the umbrella. They will be resolved together, during loom-light, informed by how aiwf can consume loom.

Two links are in scope, and they are **orthogonal** — a real configuration picks one mechanism for each:

- the **verified link**: claims ↔ Dafny sibling (a *proof*);
- the **host link**: claims ↔ host-language sibling (Go/TS/Rust/Python — *evidence*, not proof).

---

## Invariants (decided — these constrain every option)

1. **In loom-light, loom never generates the implementation.** It reads the umbrella and the sibling and checks the relationship (ADR-0017). No codegen round-trip.
2. **The portable claims surface carries no consumer-specific binding.** Host bindings live outside it; the dependency direction is consumer → loom, never the reverse. The surface must remain usable standalone.
3. **The verified link is a proof; the host link is evidence.** A mechanism must not let the second masquerade as the first.
4. **Identity is name + signature correspondence, at minimum.** An operation maps to an implementation by a stable name and signature, whatever locates them.
5. **The link's *meaning* is the gap report (§2.3), independent of mechanism.** The mechanism determines only how siblings are located and the check invoked — not what the relationship means.
6. **A missing or unresolvable binding degrades to a clear finding, never a silent skip.**

These are the load-bearing commitments. Everything below is open.

---

## Options — the verified link (claims ↔ Dafny)

### V1 — Convention + loom-assembled unit
Basename pairing (`ledger` claims ↔ `ledger.dfy`, same directory). Loom lowers the claims to Dafny contracts and lemmas, combines them with the bodies authored in the sibling, and verifies the assembled unit.

- **For.** Zero ceremony; `.mli`/`.ml` familiarity. Loom owns the contract lowering, so what is verified cannot silently diverge from the claims. The human authors only bodies.
- **Against.** Needs a clean split between the loom-owned contract region and the human-owned body region, plus a convention for how bodies are supplied. Re-lowering must merge, not overwrite (mitigated: loom owns only the spec side).

### V2 — Author-owned `.dfy` + correspondence check
The human writes complete, idiomatic Dafny (inline contracts + bodies). Loom verifies (i) Dafny proves it and (ii) the claims *correspond* to the `.dfy`'s contracts and lemmas. Mismatch is a finding.

- **For.** No assembly machinery — loom reads two files and compares. The `.dfy` is hand-written and idiomatic. The correspondence delta *is* the gap report, naturally.
- **Against.** The contract is expressed twice (claims and `.dfy`) — duplication, which invites drift (the very thing loom exists to kill). The correspondence check is itself security-relevant: it must catch a `.dfy` that quietly weakens a stated claim (the cheating attractor lives exactly here). A weak correspondence check is a hole.

### V3 — Dafny module refinement
Loom lowers the claims to an abstract Dafny spec module; the implementer provides `module Impl refines Spec`. Dafny's own refinement checker enforces the link.

- **For.** Uses Dafny's *native* spec/impl mechanism; the link is checked by Dafny itself, not by loom-bespoke logic.
- **Against.** Refinement is an advanced, sharper-edged Dafny feature — more brittle, less documented, steeper for contributors. Couples loom tightly to Dafny's refinement semantics (a portability risk if the verifier is ever swapped — cf. ADR-0002's `Verifier` trait).

---

## Options — the host link (claims ↔ host language)

### H1 — Consumer-owned manifest
A loom/aiwf manifest maps umbrella → code unit/symbol plus the test command for `shows`. The binding lives in the consumer's repo.

- **For.** Keeps the surface portable (invariant 2). Matches aiwf's existing contract-binding model exactly — the integration story for free. Explicit, auditable, supports code living anywhere in the consumer tree.
- **Against.** A separate file to maintain; can go stale if code moves (needs a "binding resolves?" check — invariant 6).

### H2 — In-code annotation
A host-source marker (e.g. `//loom:implements ledger.transfer`) names the umbrella operation from the code side.

- **For.** The link travels with the code on moves/renames; discoverable from the code.
- **Against.** Requires per-language annotation parsing (surface per host language). Puts loom vocabulary into consumer source — mild coupling, though only a comment.

### H3 — Convention
Matching package/function names in a designated directory.

- **For.** Zero declaration.
- **Against.** Implicit and fragile; poor fit for "code lives anywhere"; hard to audit; collides with real-world naming. Weakest of the three.

---

## Non-options (ruled out by the invariants)

- **Binding baked into the portable surface** (a host path or symbol inside the umbrella) — violates invariant 2 (portability / dependency direction).
- **Codegen-and-edit** (loom generates the body, the human edits it) — violates invariant 1 (round-trip; loom would own the implementation), and is out of loom-light scope per ADR-0017.

---

## How this gets decided (the evidence that settles it)

Deferred until loom-light produces evidence on the questions the option space turns on:

- Build the ledger example under **V1** and **V2**; compare the resulting gap report and, critically, run each against a deliberately *weakened* `.dfy` to see which better catches claim-weakening (this stresses V2's correspondence check and V1's lowering-ownership claim).
- Defer the host link until the first real downstream integration (likely aiwf): if aiwf-first, **H1** reuses its contract binding directly; reconsider **H2** only if code-side discoverability proves to matter.
- Decision triggers to watch: duplication/drift pain (against V2), assembly/merge pain (against V1), refinement brittleness (against V3).

When the evidence exists, a follow-up ADR records the choice (jointly with the `does`-form resolution of ADR-0017) and supersedes this one's "Open" status.

---

## References

- `docs/loom-light.md` — the stage this ADR belongs to.
- ADR-0017 (loom-light: no codegen; `does`-form deferred) — the same question from the other side.
- ADR-0002 (Dafny verifier) and its `Verifier` trait — relevant to V3's coupling cost.
- `PLAN.md` §1.1 (siblings), §2.3 (bidirectional refinement / gap report), §4.4.
