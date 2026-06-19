# loom-light — a thin, verification-first stage toward the umbrella architecture

> **Status:** draft (a plan for a *stage*, not the destination).
> **Relationship to `PLAN.md`:** `PLAN.md` describes the destination ("grand-loom" — the full umbrella architecture: a real `.lm` language, code generation, LLM operations, compositional verification, `specq`). **This document describes loom-light, an earlier stage on the path to that destination** — and possibly itself preceded by an even smaller validation step (see §1).
> **Not "v0".** The stages are a *ladder*, not a single first version, so this is deliberately not labelled "v0". `PLAN.md`'s title says "Loom v0" of the *full* realization; the rungs toward it are named, not numbered.

---

## 0. How to read this document

This is the plan for **loom-light**: the smallest thing that is still recognizably Loom — a value-gate over code correctness — rather than a demo. It is written to be honest about what it *defers* and what it leaves *open*, because its whole reason to exist is to validate the load-bearing hypothesis cheaply before the expensive parts (the language, codegen, LLM operations) are built.

Where this document records a decision, it points at the ADR that holds it. Where it leaves something open, it says so and names where it will be decided. Nothing here changes `PLAN.md`; the destination is intentionally left intact.

---

## 1. The ladder (the central framing)

loom-light is one rung of a deliberate, **additive** progression. The point of staging is anti-pivot: validate the hypothesis that carries the whole project — *that a value-gate plus weak-claim detection actually works* — before committing to a language and a code generator. Each rung adds capability; **no rung reverses a decision of the rung below it.** That is what makes this evolution, not a pivot.

| Stage | What it is | Surface | Status |
|---|---|---|---|
| **loom-ultralight** (a PoC) | The fastest possible test of the core hypothesis: does the value-gate catch weak, LLM-authored claims? Possibly **no separate language at all** — claims written in **markdown** (or inline native Dafny), the vacuity check as the one thing proven. Throwaway-acceptable. | markdown / native Dafny | **undecided** — we may or may not do this |
| **loom-light** | *This document.* A real thin tool: a claims-first surface, verify via Dafny, the vacuity check, structured findings, subprocess integration with aiwf. **No code generation.** | claims-first (form TBD) | planned |
| **grand-loom** (`PLAN.md`) | The destination: full `.lm` language, codegen, `distill`/`generate`/`summarize`, compositional verification, full `specq`. | the `.lm` language | the destination |

Two things follow from the ladder:

- **The surface is itself staged.** loom-ultralight might use plain markdown; loom-light might introduce a minimal claims surface; grand-loom is the full `.lm` language. Whether loom-light needs its own language *at all*, or can ride markdown + native Dafny longer, is **open** (§5).
- **loom-light is not necessarily the first thing built.** If loom-ultralight happens, it comes first and is allowed to be throwaway. loom-light is the first thing built *to last*.

---

## 2. What loom-light is

A single pipeline:

```
claims + implementation
   → lower claims to Dafny
   → run the verifier
   → gap report (claimed vs proved)        ┐
   → vacuity check (mutate claims, re-run)  ├─→ structured findings
   → (claimed-but-unproved, weak claims)   ┘
```

Its only hard dependency is the verifier (Dafny + Z3). It has zero workflow footprint of its own.

**The differentiator is not "runs a verifier."** Running a verifier is a one-line CI step the formal-methods crowd already has. The thing worth building — and worth *validating first* — is **catching weak, LLM-authored claims that pass anyway**: the vacuity / mutation-kill-rate check. Most of the literature frames weak specs as a *capability* failure (the model tried and couldn't); the live problem under optimization pressure is *endogenous* weakening (the same agent authors the claim and is graded on passing it). The vacuity check is the front door, not an add-on. (The mutation *technique* itself is not new — see §9, *Prior art and positioning*; what is under-served is the *endogenous* framing.)

---

## 3. What loom-light is NOT (deferred to grand-loom, or simply out)

- **No code generation.** loom-light emits no executable code; there is no `loom generate → Python`. ADR-0017. (This is what makes it reachable from any host language.)
- **Not necessarily its own language.** The full `.lm` surface is a grand-loom feature; loom-light may use a reduced surface or none (§1, §5).
- **No load-bearing LLM operations.** `distill`/`generate`/`summarize` are grand-loom. loom-light may use the LLM as a thin assist, but the architecture does not depend on it.
- **No compositional / cross-umbrella verification.** Single-umbrella only; composition is grand-loom (`docs/compositional-correctness.md`).
- **No runtime, actors, multi-user, or capability enforcement** — same exclusions as `PLAN.md` §5.

A repo with zero loom claims behaves exactly as it did without loom. The gate is **opt-in per component.**

---

## 4. Decisions already made

| Decision | Summary | Where |
|---|---|---|
| **Implementation language: Rust** | The destination is a compiler (parser, AST, lowering); Rust serves that, and switching languages mid-ladder is the costly pivot we avoid. The subprocess boundary (below) removed Go's only unique advantage (library-linking into aiwf). The honest Go counter — better LLM fluency, matches aiwf — was weighed and does not justify a future rewrite. | ADR-0001 (reaffirmed against this scope) |
| **Integration: subprocess** | aiwf consumes loom by shelling out and reading JSON findings + exit codes. loom's language is therefore invisible to aiwf. | this doc |
| **Repo: loom is its own repo** | aiwf is a *consumer*, not a host (compose-don't-absorb). Co-developed against aiwf as the first consumer; monorepo and submodule were rejected as less reversible. | this doc |
| **No target codegen** | loom-light generates no executable code. | ADR-0017 |
| **Dependency direction** | aiwf → loom, never the reverse. The claims surface stays portable; no consumer-specific paths inside it. | ADR-0017, ADR-0018 (invariants) |

---

## 5. Decisions deferred to when work starts

These are deliberately **not** decided up front; they will be settled during loom-light from implementation evidence — especially *what the verifier needs* and *how aiwf can actually consume loom*.

- **The form and role of `does`** — reference to a Dafny sibling / reference to host code / prose intent / inline verified body / omitted / polymorphic over these. (ADR-0017.)
- **The spec↔implementation binding mechanism** — V1/V2/V3 for the verified link, H1/H2/H3 for the host link. The same question as the `does`-form, from the other side; decided jointly. (ADR-0018.)
- **The surface** — markdown vs a minimal `.lm` vs native Dafny. Tightly coupled to whether loom-ultralight happens first (a markdown PoC would answer a lot of this).

---

## 6. The honest boundary (scope of value)

Verification flows through Dafny, so **only what is expressible in / lowerable-to Dafny is *proved*** — regardless of the production language. In practice:

- The **crisp-invariant core** — ledgers, parsers, protocols, state machines, allocators, authorization logic — is written in Dafny and proved.
- The **glue** — IO, UI, host-language service code — stays in the host language and is *not* proved by loom; it is covered by tests and review (the evidence layer).

This is the 25-year formal-methods boundary — high yield on sharp invariants, low yield on glue — stated plainly rather than hidden. It is why the gate is opt-in per component, and it is the reason "used mostly by downstream consumers" must be read with care: the *guarantee* reaches a downstream consumer only where they are willing to express a crisp core in Dafny. The broader reach is the vacuity check and the findings, not a proof of arbitrary host code.

---

## 7. Relationship to aiwf (the tie, done right)

aiwf is loom's **first and most capable consumer, not its host.** loom emits structured findings; aiwf lifts them into its findings / wrap-time triage / provenance / pre-push chokepoint — the *containment loop* that handles weak-but-passing claims by humans, deliberately, with a record. A standalone user gets the same findings without the governance apparatus.

The advantages of "tying" loom to aiwf are real but they are **consumer-side** advantages — they accrue to *aiwf consuming loom*, not to loom living inside aiwf. They are captured by making aiwf the privileged consumer plus co-development, without the structural cost of co-location. (The aiwf-side framing of this is `../aiwf/docs/pocv3/plans/loom-light-plan.md`, in the aiwf repo.)

---

## 8. What would validate loom-light (the hypothesis test)

The load-bearing question is whether the differentiator is real. The test — the natural job of a loom-ultralight PoC — is the **endogenous-gaming experiment**:

1. Ask a model to author a claim for a component **when it is only specifying** (disinterested) → record claim strength (mutation kill-rate).
2. Ask the **same** model to author the claim **when it is also graded on making its implementation pass** (incentivized) → record claim strength.
3. **The gap is the result.** Strong-when-disinterested + weak-when-incentivized is exactly what the accuracy-framed literature cannot see, and the vacuity check is what catches it. If the gap does not appear, that is also a real result — and it tells you the differentiator is weaker than hoped *before* you over-invest.

The concrete, runnable design — anchored on a **real aiwf invariant** (entity-id canonicalization) rather than a toy, with all artifacts authored so the human only installs-and-runs — is written up in [`loom-ultralight.md`](loom-ultralight.md). Running it is reason enough to do the loom-ultralight rung before loom-light proper.

---

## 9. Prior art and positioning

loom-light's mechanisms are, with one exception, **already published and empirically evaluated.** This is stated up front on purpose: the project stands on that work rather than claiming to invent it, and its honest contribution is narrower than "a new verification technique."

**The technique lane is closed.**

- The **vacuity check** — mutate the claims, re-verify, treat a surviving mutant as a weak claim — is **[MutDafny](https://arxiv.org/abs/2511.15403)** (32 Dafny mutation operators mined from bugfix commits, evaluated on 794 real programs) and **[IronSpec](https://www.usenix.org/system/files/osdi24-goldweber.pdf)** (Specification Testing Proofs + mutation, OSDI '24). loom-light should **reuse MutDafny's operators, not reinvent them.**
- The **correspondence check** (ADR-0018 option V2 — claims ⟷ the author's `.dfy`) is essentially **[CLOVER](https://arxiv.org/pdf/2310.17807)** (closed-loop consistency among code/spec/doc; ~87% accept on correct, 0 false-positives on adversarial-incorrect). Note CLOVER's checker is itself LLM-mediated — a probabilistic checker of a probabilistic author, which collides with the independence requirement in [`docs/containment-not-solution.md`](research/containment-not-solution.md) §4. Carry that into the binding decision (ADR-0018).
- The **Dafny-as-verified-intermediate, host-code-separate** shape (the claims-only surface) is a published 2025 direction — **[Dafny as a Verification-Aware Intermediate Language](https://arxiv.org/html/2501.06283)**.
- The **weak-postcondition problem** is a recognized, benchmarked subfield (**[nl2postcondition](https://dl.acm.org/doi/10.1145/3660791)**, CodeSpecBench, SpecGen), and **lower-to-a-verifier** is the decades-old intermediate-verification-language pattern (Why3, Boogie, Viper).

**The one open lane is the framing, not the mechanism.** The spec-quality literature treats weak specs as a *capability* failure (the model tried and couldn't). The alignment literature documents specification gaming and reward-tampering (**[gaming in reasoning models](https://arxiv.org/abs/2502.13295)**, **[Sycophancy to Subterfuge](https://arxiv.org/pdf/2406.10162)**) but in *task completion*, not spec-authoring. **Neither studies the case loom-light targets:** the same agent authors the claim *and* is graded on passing it, so weakening is *endogenous and incentivized*. That intersection is under-served — and it is a **framing + experiment** contribution (§8), not a technique.

**So, honestly:**

- loom-light is **not** a novel verification method. Measured as research novelty, the technique lane does not stand up — MutDafny / IronSpec / CLOVER own it.
- loom-light **is** (a) the *productization* of validated techniques into an opt-in, workflow-integrated value-gate with findings, triage, provenance, and a chokepoint — none of the prior-art tools are productized into a governance loop; and (b) the *endogenous-gaming experiment*, which is the citable delta **if** it reproduces.
- The moat is therefore **integration + framing**, which is thinner than a method moat. This sharpens the adoption-ceiling risk: good packaging removes friction, it does not create demand. The loom-ultralight PoC's job (§8) is to find out — cheaply — whether even the framing contribution is real, before loom-light is built.

---

## 10. Open questions

- **Do we build loom-ultralight (the PoC) first, and in what form?** Markdown claims + native Dafny is the leading shape; undecided.
- The deferred ADR-0017 / ADR-0018 questions (`does`-form, binding).
- Whether loom-light needs its own surface language at all, or rides markdown / native Dafny.
- Where the verifier flakiness (Z3 timeouts) bites the *vacuity* gate specifically — a surviving mutant must be distinguishable from a timed-out one (killed / survived / inconclusive), or the kill-rate signal is corrupted.

---

## 11. References

- `PLAN.md` — the grand-loom destination this stage evolves toward.
- ADR-0001 (Rust), ADR-0002 (Dafny), **ADR-0017** (no codegen; `does`-form deferred), **ADR-0018** (binding option space).
- `docs/containment-not-solution.md` — the locatability / containment posture loom-light embodies.
- `docs/rethink-stopgap.md` — the hand-rolled value-gate practice loom-light is meant to mechanize.
- `docs/compositional-correctness.md` — cross-umbrella verification (grand-loom, not loom-light).
- `../aiwf/docs/pocv3/plans/loom-light-plan.md` — the aiwf-side proposal (lives in the aiwf repo).

### Prior art (external — see §9)

- [MutDafny](https://arxiv.org/abs/2511.15403) — mutation-based assessment of Dafny specifications.
- [IronSpec](https://www.usenix.org/system/files/osdi24-goldweber.pdf) — increasing the reliability of formal specifications (OSDI '24).
- [CLOVER](https://arxiv.org/pdf/2310.17807) — closed-loop verifiable code generation (consistency checking).
- [nl2postcondition](https://dl.acm.org/doi/10.1145/3660791) — LLMs turning NL intent into formal postconditions.
- [Dafny as a Verification-Aware Intermediate Language](https://arxiv.org/html/2501.06283) — Dafny as the verified intermediate for codegen.
- [Demonstrating specification gaming in reasoning models](https://arxiv.org/abs/2502.13295) and [Sycophancy to Subterfuge](https://arxiv.org/pdf/2406.10162) — the alignment-side gaming literature (task completion, not spec authoring).
