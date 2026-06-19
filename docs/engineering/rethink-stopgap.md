# Rethink — a stop-gap for the value gate

> **Status:** draft (interim practice)
> **Audience:** anyone building software with an LLM agent *before* Loom exists — including the authors of Loom itself.
> **Companion:** [`docs/bidirectional-refinement.md`](../reference/bidirectional-refinement.md) (the gap report this practice imitates by hand), [`docs/research/process-gates-and-value-gates.md`](../research/process-gates-and-value-gates.md) (why a *value* gate is the thing worth approximating).

---

## 1. Why this document exists

Loom is seed-stage: the architecture is specified but the code does not yet exist, so the verifier, the gap report, and `specq` are not available to lean on. In the meantime, software still gets written — including Loom's own tooling — with an LLM agent and no mechanical gate behind it.

This document records one practice that partially fills that gap. It is explicitly a **stop-gap**: a manual, fallible stand-in for the value gate Loom will eventually provide. It is not part of the architecture. When `loom verify` exists, most of what follows is subsumed by it. Until then, it is worth having written down, because it is the cheapest available defense against the failure mode Loom is being built to eliminate.

## 2. The failure mode

An LLM coding agent behaves like a greedy optimizer: at each step it takes the first adequate move from the current state, rarely the globally best one. Each individual edit is locally reasonable, so the result is *correct* — and yet globally misshapen, because the structure is the accumulated residue of a path rather than a design anyone would choose from scratch. The codebase converges on a **local optimum**: working, but carrying incidental complexity, abstractions with one caller, and defensive layers that exist only because of the order in which things were built.

This is the implementation-layer twin of the *cheating attractor* named in `PLAN.md` §2.4 — gradient incentive pushing toward whatever clears the bar most cheaply. Loom attacks that attractor at the claim layer with `specq` and at the implementation layer with the verifier. With neither available yet, `rethink` is the hand-rolled substitute for the second.

## 3. The practice

Periodically — before committing a non-trivial design, or whenever a unit feels accreted — re-evaluate **one bounded unit** (a file, module, function, or decision) by reconstructing it *from intent, as if the current code did not exist*. The reconstruction forces a non-local view of the problem and surfaces the difference between essential and path-dependent structure.

The naive form of this ("rebuild it from scratch, then keep it if it feels cleaner") has a fatal weakness: the judgment of "cleaner" comes from the same greedy optimizer that produced the local optimum. A from-scratch rewrite can simply be a *differently*-local optimum. The practice is only useful with a substitute for the missing verifier — an explicit **obligation gate**: before redesigning, write down what the unit owes (behavior, public interface, invariants, tests), and permit a rewrite *only if it preserves every one of them*. That list is a poor person's umbrella; "preserves every obligation" is a poor person's `loom verify`; "incidental vs. essential complexity" is, by hand, category (C) of the gap report.

## 4. Portable skill definition

Drop the following into any repo as `.claude/skills/rethink/SKILL.md` (or, for a pure prompt template with no auto-triggering, as `.claude/commands/rethink.md` with the frontmatter removed). Invoke with `/rethink <target>`.

```markdown
---
name: rethink
description: >-
  Re-evaluate the design of a specific unit (file, module, function, or
  decision) by reconstructing it from scratch, and adopt the new design only if
  it is simpler and preserves all existing behavior. Use when code works but
  feels over-complex or shaped by its edit history, before committing a
  non-trivial design, or when the user invokes /rethink.
---

# Rethink

Re-evaluate one unit's design by rebuilding it from intent, then keep whichever
design is simpler — but only when it provably preserves what the current code does.

## Scope

Operate on **one unit** named by the user (a file, module, function, or design
decision). If none is given, infer it from the just-completed work and state your
choice in one line before proceeding. Never rethink the whole codebase at once.

## Procedure

1. **Pin the obligations.** Before looking at the current structure, list what the
   unit must keep true regardless of design: observable behavior, public
   interface/signature, invariants, and the tests it must still pass. Flag any
   that are currently unstated — those are the riskiest to break.
2. **Reconstruct from intent.** Describe how you would build this from scratch to
   satisfy the underlying problem, deliberately not referencing the current
   structure while you do it. Work at the level of data model, control flow, and
   the core abstraction — not naming or micro-style.
3. **Diff essential vs. incidental.** Compare the from-scratch design to what
   exists. Name concretely what the current code carries only because of how it
   grew (path-dependent state, defensive layers, single-caller abstractions,
   unused capability) versus what is genuinely load-bearing.
4. **Decide, biased to keep.** Adopt the from-scratch design only if it is both
   simpler/clearer and preserves every obligation from step 1. Otherwise keep the
   current code. "No change warranted" is a correct and common outcome.

## Output

Report, then act:

- **Obligations** — the gate from step 1.
- **From-scratch design** — a few lines, structure only.
- **Delta** — incidental complexity found; essential parts confirmed.
- **Verdict** — `keep` or `rewrite`, with the concrete win that justifies it.
- If `rewrite`: implement it, then run the tests / re-check each obligation and
  report the result. Never declare success from the design alone.

## Rules

- Never weaken, drop, or delete an obligation — a behavior, an invariant, a test —
  to make the rewrite look better. If an obligation seems wrong, flag it for the
  user instead of silently relaxing it.
- Preserve the public interface unless the rethink is explicitly about the interface.
- Default to keep. A rethink that changes nothing is a successful audit, not a failure.
```

## 5. Honest limits

The gate is only as strong as the obligations you can name. On code with no tests and no written invariants, `rethink` still partly trusts the model's own judgment — which is the very thing the practice is trying to discipline. That residual gap is not a flaw to fix at the prompt level; it is precisely the gap Loom exists to close, with a machine-checkable umbrella in place of a hand-listed one. Read this practice, then, as both a useful interim tool and a standing reminder of why the verifier is worth building.
