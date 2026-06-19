---
id: ADR-0004
title: No actor runtime in v0
status: proposed
---

# ADR-0004 — No actor runtime in v0

> **Date:** 2026-05-22 · **Deciders:** project initial author; open for review.
> **Related:** `PLAN.md` §5.1; architecture paper §6 (the substrate the architecture envisions).

---

## Context

The architecture paper [`docs/research/verifiable-umbrella-paper-v2.md`](research/verifiable-umbrella-paper-v2.md) §6 describes a runtime model in which Loom-verified components run as supervised actors. The actor model is part of the architecture's full vision: supervised concurrent processes, message-passing isolation, failure detection and restart, the kind of fault-tolerance Erlang/Elixir popularized.

The v0 plan must decide whether to include any portion of this runtime model in the initial release.

The candidates considered are:

1. Full actor runtime with supervisors (the paper's vision).
2. Minimal actor runtime — message-passing between processes, no supervision.
3. No actor model — generate plain functions and modules.
4. Optional/opt-in actor model — `does` blocks can declare themselves as actors, otherwise plain.

The decision interacts with the target-language choice (ADR-0003): Python's threading and async ecosystems offer raw materials but no out-of-the-box supervised-actor framework. Adopting an actor model would mean either (a) integrating with an existing framework like `pykka`, `thespian`, or `dramatiq`, (b) writing a small Loom-specific actor runtime, or (c) generating BEAM-language code (Erlang or Elixir) for a subset of modules — a major scope expansion.

---

## Decision

**No actor runtime in v0.** Loom generates plain Python functions and modules. The architecture paper's runtime vision is deferred to post-v0 work.

The decision is documented prominently in `PLAN.md` §5.1 as out-of-scope content. Re-opening the decision requires a follow-up ADR and re-opening of the plan.

---

## Considered alternatives

### Option 1: Full actor runtime with supervisors

**For.**

- Matches the architecture paper's full vision. v0 would demonstrate the architecture more completely.
- Some claims that are natural in an actor context (eventual consistency, supervised restart) cannot be expressed without actors.
- Future work would build on actor primitives that are already present.

**Against.**

- Substantial additional implementation: supervisor trees, message dispatch, failure-detection mechanisms, distributed-actor questions.
- The verification model for actor systems is significantly harder. Dafny does not directly support actor semantics; encoding them would be a research project on its own.
- Adds a substantial dependency surface (a chosen actor framework, its runtime, its operational characteristics).
- The thesis under test in v0 is the three-layer architecture (prose / umbrella / implementation) with bidirectional refinement. Actors are a downstream concern; including them dilutes the v0 demonstration.
- If the actor implementation is bad in v0, the architecture's reputation may suffer for reasons unrelated to its core thesis.

### Option 2: Minimal actor runtime — message-passing only

**For.**

- Less work than full supervision but provides the concurrent-actor flavor.
- A small Loom-specific runtime can be ~500 lines of Python.
- Lets v0 demonstrate at least one actor example.

**Against.**

- "Minimal actors without supervision" is not what the architecture paper describes; v0 would be neither one thing nor the other.
- The verification model question remains. Even message-passing without supervision requires modeling concurrent state in the verifier.
- The implementation effort is non-trivial even at "minimal" scope.

### Option 3: No actor model — plain functions and modules

**For.**

- Smallest implementation effort. The codegen for plain functions is straightforward (per `docs/verification-internals.md` §3).
- Verifier model is straightforward: pure functions, sequential composition.
- The v0 thesis (three-layer architecture, bidirectional refinement) is unaffected; the example corpus can demonstrate it without actors.
- Operations that would be natural as actors (long-running stateful processes) are not in v0's example corpus.
- Post-v0 work on actors can build on a stable, well-tested non-actor foundation.

**Against.**

- v0 does not validate the architecture's full claim. The runtime portion of the paper is not exercised.
- Some classes of system (chatbots, real-time data processors, distributed services) cannot be naturally expressed in v0 because they need actors. The example corpus is constrained.
- The "we'll add it later" story may be perceived as v0 being incomplete.

### Option 4: Optional / opt-in actor model

**For.**

- Lets users who want actors get them, while keeping the default plain.
- The Loom syntax could allow `does {actor: behavior(...) -> Behavior {...}}` for actor modules and plain `does` for non-actor modules.

**Against.**

- The cost is in the *option*, not in the usage. Even if no example uses actors, the codegen must support generating either. The runtime would still need to exist for the cases that opt in.
- "Optional" features in v0 systems often end up neither well-tested (because they're optional) nor well-removed (because they're claimed-supported). Worst of both worlds.
- Conceptually unclean: actors and non-actors interact in ways (concurrency boundaries, message passing) that the umbrella's verification model would need to handle in v0. That work is the same as full actor support.

---

## Consequences

### Positive

- The v0 scope is bounded and achievable. The pipeline (parse → check → verify → codegen → execute) can be implemented and demonstrated cleanly.
- The thesis under test (three-layer architecture with bidirectional refinement) is exercised by the example corpus without dilution.
- Verification stays within Dafny's comfort zone (pure functions, immutable data). The translation is mechanical and the discharge is reliable.
- Generated Python code is plain and reviewable.

### Negative

- The architecture paper's full vision is not validated by v0. Reviewers comparing v0 to the paper will see the gap.
- Some interesting examples (anything stateful or concurrent) cannot be in v0's example corpus.
- The "we deferred actors" decision may need to be defended repeatedly until the post-v0 actor work begins.
- The eventual actor work will be a significant undertaking; deferring does not eliminate it.

### Neutral

- The plan's §5 commitment includes other deferred items (multi-user, multi-target, capability tracking, runtime view, visual editor, fine-tuned LLM). Actors are one item in that list; the precedent for "v0 is a research prototype, not a production system" is well-established by these other deferrals.

---

## Re-opening conditions

This decision should be re-opened if any of:

- v0 is delivered and the next major scope discussion is about actor support.
- A specific example becomes a v0 success criterion that requires actor semantics. (None do, in the current plan §6.)
- A contributor with deep actor-runtime experience joins and proposes including actor work in v0. The bar is high: the contributor would have to commit to the work, the verifier story would need to be sketched, and the example corpus would need to be redesigned to demonstrate actors without weakening the demonstration of the core architecture.

Otherwise, the decision stands.

---

## Implementation notes

- The codegen target (per ADR-0003 and `docs/verification-internals.md` §3) produces plain functions, no actor decorations, no message-passing scaffolding.
- The Loom syntax (per `docs/language-reference.md`) does not include actor-related forms in v0. Reserved syntactic positions are noted in §10 of that document for potential future use.
- The architecture paper's §6 discussion of supervised actors and capability profiles is preserved in `docs/research/` as the source-of-truth for the post-v0 vision; the v0 plan does not contradict it, only defers it.

---

## References

- `PLAN.md` §5.1 (actor runtime out of scope), §5.3 (multi-target also deferred), §5.4 (capabilities also deferred).
- Architecture paper §6 — the runtime vision being deferred.
- ADR-0003 — target language is Python; an actor model would constrain that choice further.
- Erlang/OTP — the canonical supervised-actor reference, kept in view for the post-v0 work.
