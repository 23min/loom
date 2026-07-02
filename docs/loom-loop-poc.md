# loom-loop — the whole umbrella loop, dogfooded on real aiwf code

> **Status:** design / direction. A deliberate **pivot back** from the "light" ladder rungs to the *whole loop* at small scale — start simple, evolve.
> **One line:** a human who *cannot* write formal specs authors **prose + examples**; an LLM authors the **formal umbrella**; a **verifier + gap report** close the loop; we watch how far it gets on **real aiwf code** — and whether the human ever has to read a line of Dafny.
> **Relationship to the ladder:** this is what **loom-ultralight was supposed to be** — the cheapest validation of the load-bearing idea — but done *whole* instead of stripped. It precedes loom-light (`loom-light.md`); it builds toward the umbrella architecture (`README.md`), not away from it.
> **Supersedes** `loom-completeness-poc.md` as the next move. That plan was reviewed by four independent reviewers and found to test the wrong thing in the wrong order (toy subjects, the textbook formal-methods cell, no human, the gap report's actual value engineered out). Their critique is what motivated this pivot; §9 records how this doc answers it.

---

## 0. Why this exists (the pivot)

Three experiments (E-0001/E-0002/E-0003) returned NO-GO-adjacent results and left a feeling that "loom keeps failing." That feeling is an artifact of **measuring the wrong thing**. All three tested a single adversarial side-question — *does an LLM weaken a spec when graded on passing it?* (gaming) — and the answer (it mostly doesn't) is **good** for the architecture, not bad. The umbrella concept itself was never on trial.

To make the gaming question cheap and fully automatable, the "light" rungs stripped the loop down to: *LLM writes bare Dafny `ensures`, mutation-check its strength.* That removed exactly the parts that **are** loom:

- the **prose** intent section,
- the **examples** (the non-expert's lever),
- the **human** in the loop,
- the **gap report** itself (the load-bearing visible artifact),
- and **real code** (the experiments used decidable toy *models*).

What remained was loom with its soul removed. The rungs were too light not in *cost* but in *content*. The correct notion of minimal is **minimal-but-whole** — the smallest thing that still has every section and a human reading it — not **minimal-but-stripped**.

This doc is the minimal-but-whole loop.

## 1. The principle

The umbrella has **sections** because the work is **split**. The human and the LLM each author the part they can:

| Section | Author | The human's relationship to it |
|---|---|---|
| **Intent** (prose) | **human** | writes it — their domain, their language |
| **Examples** (concrete cases) | **human** | writes it — `input → expected`, readable, checkable |
| **Claims** (formal Dafny) | **LLM** | **never reads it** — it is infrastructure |
| **Back-translation** (claims restated in English) | **LLM** | **audits it** against Intent |
| **Gap report** (A/B/C) | **verifier** | **reads + decides** — accept / reject / push |

"I can't write the formal spec" is not a problem with loom; it is the **premise** of loom. The human authors two sections and audits two. They never touch the formal one.

## 2. The artifact — the umbrella, worked on real id-canonicalization

```
UMBRELLA — entity-id canonicalization
══════════════════════════════════════════════════════════════
§ INTENT          ← human (prose)
  An id has a kind (E, M, ADR…), a numeric value, and a width.
  Canonicalize pads the value with leading zeros to a per-kind
  minimum (4 for most). It never changes the kind or the value —
  only display width. Already-wide ids are unchanged. Idempotent.

§ EXAMPLES        ← human (concrete; you can read these back)
  E-7        → E-0007
  ADR-12     → ADR-0012
  E-0001     → E-0001            (already ≥4, unchanged)
  E-123456   → E-123456          (wider than min, preserved)
  M-1/AC-2   → M-0001/AC-0002    (composite — the recursion case)

§ CLAIMS          ← LLM (formal Dafny, under the hood — never read by you)
  ensures result.kind  == x.kind
  ensures result.value == x.value
  ensures result.width == max(x.width, minWidth(x.kind))
  ensures wellformed(result)
  ensures canon(canon(x)) == canon(x)

§ BACK-TRANSLATION ← LLM (English; you audit vs INTENT)
  "kind never altered" · "value never altered"
  "width = larger of original and the kind's minimum"
  "canonicalizing twice = once"

§ GAP REPORT      ← verifier (A/B/C; you read + decide)
  (A) proved:    kind, value, wellformed, idempotent      ✓
  (B) UNPROVED:  width — TIMEOUT on the composite recursion ← signal
  (C) unclaimed: "value preserved even for malformed widths" — promote?
```

The human authored **Intent** and **Examples**; audited **Back-translation** and the **Gap report**. The formal **Claims** were the LLM's, and stay under the hood.

**Umbrella convention** (established in loop 1). An umbrella follows the five-register `.lm` skeleton — `knows` · `relates` · `shows` · `does` · `proves` · `gap` ([`language-reference.md`](reference/language-reference.md) §4) — not ad-hoc sections. Framing: the **umbrella is the whole markdown document**; the `module` is its **formal spine**. The prose **Intent** (layer 1) and the **back-translation** (the `summarize` op's output) stay as markdown adjuncts — no doc-comment walls. Process/metadata (which loop, blind-subagent provenance, the Dafny-lowering caveat) lives in the **gap report**, not the umbrella. loom's semantics map exactly: `proves` is checked against `does`, and the failures *are* the gap report. Worked real example: [`experiments/loom-loop/milestone-fsm/umbrella.md`](../experiments/loom-loop/milestone-fsm/umbrella.md). (The sketch above predates the convention and keeps its old labels for illustration.)

## 3. The loop

```
   YOU: intent + examples
        │
        ▼
   LLM: formal claims  +  back-translation  +  a Dafny model of the REAL impl
        │
        ▼
   VERIFIER:  impl vs claims   AND   claims vs YOUR examples
        │
        ▼
   GAP REPORT (A / B / C, in English)
        │
        ▼
   YOU: audit back-translation, read gaps → accept / reject / add an example / push
        │
        └──────────────────── iterate ────────────────────┘
```

Two trust rails let a non-expert steer a formal layer they cannot read:

1. **Examples are mechanically checked against the claims** — a claim that disagrees with your `input → expected` is caught without you reading Dafny. (loom's `shows` register; the non-expert's anchor *from below*.)
2. **Back-translation is audited against intent** — you check the LLM's English account of each claim, not the Dafny. Paired with examples (which can't lie), this catches unfaithful formalization.

Neither closes the gap fully (§7). The **gap report** is where the residual lives, made visible.

## 4. What we are actually trying to find out

Four questions, observed — **not** scored against a pre-registered threshold (see §4.1):

- **Tractability** — does a *real* impl verify, or does the gap report fill with category-(B) timeouts? (the `width`-on-recursion gap in §2 is the canary.) *This is the load-bearing unknown the toy experiments dodged.*
- **Faithfulness** — do the LLM's claims agree with your examples (mechanical) and your intent (back-translation audit)? When they don't, is the disagreement *visible*?
- **Value** — did a gap or a category-(C) finding tell you something **true and useful you didn't already know** — a missed case, a hidden invariant, a real bug?
- **Effort** — iterations, wall-clock, and the load-bearing one: **did you have to read any Dafny?** Target: no.

### 4.1 This is a feasibility dogfood, not a confirmatory experiment

Deliberately, this PoC **does not pre-register a pass/fail threshold.** We do not yet know whether the loop runs on real code at all, or which numbers would matter — so inventing a metric to clear would be premature, and (after a post-hoc PROCEED that was twice honestly falsified) constructing a fresh metric whose GO keeps the project alive is exactly the motivated-reasoning trap the reviewers named. The honest output here is **"here is what happened when we actually turned the loop on real code, warts and all"** — observations, transcripts, the actual gap reports, the points where it broke. Pre-registration and thresholds come *later*, once we know the loop is feasible and what is worth measuring. We are now looking under the dark, not polishing the streetlight.

## 5. Start simple, evolve — the loop ladder

Each loop adds **one** source of realism. Do not vary two at once.

1. **Loop 1 — the status-transition FSM** (`internal/entity/transition.go`). Discrete, enumerable, no string parsing — Dafny-friendly, and real aiwf logic the author knows cold. **Goal:** prove the loop *completes* end to end; learn the ergonomics, faithfulness, and whether the gap report says anything useful. Highest chance of a clean full turn.
2. **Loop 2 — real canonicalization** (`internal/entity/canonicalize.go`). Strings, per-kind widths, composite-id recursion. **Goal:** stress **tractability** on purpose, having already validated the loop itself. (The §2 `width`-timeout gap is the expected failure surface — and finding it is a *result*, not a defeat.)
3. **Loop 3+ — a stateful / multi-step invariant** (e.g. the allocator, or a small lifecycle property). **Goal:** push past pure functions toward the messiness real loom must survive.

Stop and reassess after each. A loop that *fails honestly* (drowns in timeouts; the LLM can't faithfully formalize; the gap report is noise) is a real, cheap finding — the kind the toy program could never produce.

## 6. What success and failure look like (qualitative)

- **Encouraging:** the loop completes on Loop 1 with the author never reading Dafny; the back-translation faithfully matches intent; at least one gap or category-(C) finding is *true and non-obvious*; Loop 2 verifies, or fails in a *characterizable* way (a named tractability limit, not chaos).
- **Discouraging:** the author cannot steer without reading the formal layer; the LLM's claims pass the examples yet are subtly unfaithful in ways the human can't catch; the gap report is dominated by timeouts even on Loop 1; or every "gap" is either already-obvious or spurious. Any of these is a cheap, honest signal to rethink — far cheaper than building loom-light first.

## 7. The irreducible residual (state it plainly)

The verifier checks **impl against claims**; it never proves the **claims capture intent**. Examples pin the cases you thought of; back-translation pins what the LLM admits it wrote; neither proves the spec is *right* in a region you didn't example and didn't say. That residual never reaches zero. **loom's honest claim is only that it makes the residual *visible and shrinkable*, not gone** — the gap report is the surface of "here is what I did and did not establish." A PoC that pretends to eliminate the residual is lying; this one surfaces it.

## 8. Scope — what this is NOT (yet)

- **Not the `.lm` language.** The umbrella's sections are realized as prose / examples / LLM-authored-Dafny / gap-report. A dedicated readable claims surface is a *later* evolution, not a prerequisite (`loom-light.md` §5 leaves it open).
- **Not a tool build.** Loop 1 can be turned **by hand + LLM + a thin Dafny shell-out** (reuse the existing harness plumbing — *not* its certified per-subject validity gate, which is reallocate-specific). Build only what the next loop forces.
- **Not codegen, multi-user, or composition** (`loom-light.md` §3 exclusions stand).
- **Not yet a human study.** The first author is the originator dogfooding. A real human+LLM-team study is a later rung.

## 9. How this answers the four-reviewer critique of the completeness PoC

- **Capability ceiling** (red-team) — no longer load-bearing: we are not measuring an omission-rate-on-toys; we run the whole loop on real code where the model genuinely may struggle, and that struggle is the finding.
- **"Cheap re-pointing" cost lie** (red-team) — dropped. This reuses only the Dafny shell-out plumbing, not the certified gate; it is honest that real code is new work.
- **Measuring around loom's value** (loom-architecture) — fixed: the gap report's *visibility* and the *faithfulness* of the formal layer are the object of study, not the textbook "verification beats sampled tests" cell.
- **Wrong sequence** (strategy) — fixed: this *is* the tractability smoke on real code **plus** the human-in-the-loop, which the reviewers said dominates the toy PoC on both branches.
- **Motivated reasoning** (red-team + strategy) — fixed by §4.1: an honest feasibility dogfood with no constructed pass-threshold, not a fresh metric engineered to yield a GO.

## 10. Next step

Turn **Loop 1** by hand: the author writes the **Intent** and a handful of **Examples** for the status-transition FSM; the LLM authors the **Claims** + **back-translation** and a Dafny model of the real transition logic; run the verifier; read the gap report together. Observe the four questions in §4. Decide Loop 2 from what actually happened — not from a plan.
