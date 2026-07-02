# Loom's Reach — an ambition and scoping note

> What loom could verify, for whom, and with what. Property *shape* over code *size*; a graded, honest, multi-substrate verifier over a push-button oracle for arbitrary code.

## 0. What this is

This is a scoping and ambition note, not a plan and not a spec. It exists to answer a question that has to be settled *before* building the thin tool the E-0004 dogfood greenlit (decision `D-0006`, qualified proceed): **what can loom actually be useful for, how would a non-expert know when to reach for it, and is what we have seen so far the limit — or a floor?**

It is grounded in real evidence. E-0004 turned loom's whole umbrella loop on real aiwf code twice: a decidable status-transition FSM (`M-0014`) and string-based id-canonicalization (`M-0015`). The first was push-button and surfaced a genuine bug; the second mapped a hard tractability boundary precisely. Those two data points are the spine of everything below. Where this note reasons past them — about other substrates, about property classes we did not test — it says so.

It sits above the PoC layer ([`loom-loop-poc.md`](../loom-loop-poc.md), [`loom-light.md`](../loom-light.md)) and the architecture layer ([`../README.md`](../README.md)): those describe *how the loop works*; this describes *what the loop is for and how far it can reach*.

## 1. The thesis

**What E-0004 hit is the real, well-mapped boundary of SMT-backed verification — not an artifact of a small experiment. But it is a boundary of property *shape*, not of code *size* or "smallness." And E-0004 exercised only the thinnest possible version of the loop. So it is a floor, not a ceiling — and whether loom is "narrow" or "broad" is largely a decision about *which loom we build*, not a fact any verifier hands us.**

The fear worth taking seriously is: *a tool useful only for a very small subset of problems, that looks like a lot of work for small things.* That fear is **correct** about one possible loom — the one that promises push-button full proof of arbitrary code. That loom is narrow and E-0004 is near its limit. The fear is **misplaced** about the loom our architecture actually describes — a graded verifier that spans full proof → bounded proof → concrete examples, honest at every rung about what it did and did not establish. That loom is broad, and E-0004 tested one corner of it.

The rest of this note makes that distinction concrete.

## 2. Why the FSM/string split was inevitable

Dafny is an *auto-active* verifier. Z3 (an SMT solver) discharges what it can automatically; a human supplies hints — lemmas, induction, `{:fuel}` — where it cannot. The line between "automatic" and "needs a human" is one of the best-understood facts in the field, and it is exactly the line E-0004 drew:

- **Push-button zone** — relational, finite, algebraic properties over structured data (datatypes, sets, maps, bounded sequences). The FSM lives here; Z3 simply *decides* it.
- **Expert zone** — universal properties over *recursive functions on unbounded data* (strings, bytes) that need an invented induction hypothesis; nonlinear arithmetic; unbounded loops needing invented invariants. Canonicalize lives here.

This split is not our discovery — it is *the* structural fact of SMT verification. The tell that matters for loom's ambition: **the real-world Dafny deployments all live in the push-button zone by deliberate targeting.** AWS's Encryption SDK and Cryptographic Material Providers are written in Dafny; the Cedar authorization language's semantics were formally verified (first in Dafny, later modeled in Lean); Microsoft Research's IronFleet verified Paxos-style distributed protocols. Every one of these verifies *relational / state-machine / algebraic* properties, with experts doing induction work only at specific hard edges. None verifies "arbitrary code." *Nothing does.* (These examples are illustrative and worth re-verifying before we lean on any specific one in a pitch.)

So "loom cannot verify arbitrary code" is not loom failing. It is loom being a verification tool.

## 3. The trap, and the way out

The narrowness fear rests on an implicit model: *loom must verify whole programs, or arbitrary code.* Drop that model.

**Value in verification never comes from proving whole programs. It comes from proving the two or three invariants that, if violated, cause the worst bugs.** You do not loom the codebase; you loom the invariants. This is how every real verification effort works, and it is why "we can't verify everything" has never stopped anyone — you were never going to.

And the invariants worth proving cluster in a zone that is *both tractable and high-stakes*. That zone is the subject of the next section — it is the real payload of this note.

## 4. The property catalogue — what loom is *for*

These are the recurring shapes of high-value, tractable properties. They are not a tiny subset; they are the load-bearing logic of authorization, money, state, and protocols in most real systems. For each: the shape, plain-language examples, why it is tractable, and — the part that makes it usable — **how you would recognize it in a system you are working on.**

### 4.1 State machines / lifecycles

*"You can't ship before payment." "Cancelled is terminal." "This event is only valid in these states."*

Order flows, payments, auth sessions, connection states, subscription lifecycles, workflow engines. This is the FSM loop from `M-0014` — and it is not a toy; it is where a huge share of business-logic bugs live. Decidable and relational, so it is squarely push-button.

**Recognize it when:** your code has a `status` / `state` / `phase` field, a set of allowed transitions (often a `switch`, a table, or a tangle of `if` guards), and rules like "you can only do X when you're in state Y." The moment someone says "wait, how did that order get *shipped* if it was never *paid*?" — that is an unenforced state-transition invariant.

### 4.2 Authorization

*"No user without role R can ever reach action A." "Deny overrides allow." "A revoked token grants nothing."*

This is exactly the class Cedar verified. Relational and largely finite, so tractable — and security-critical, which makes the universal quantifier ("*no* user, *ever*") worth far more than a sampled test.

**Recognize it when:** there is a permission check, a role/scope/capability model, a policy evaluated against a request. Any sentence of the form "someone who *isn't* allowed to do this must *never* be able to" is an authorization invariant — and "never" is a proof obligation, not a test case.

### 4.3 Structured-data invariants

*"Line items sum to the total." "No two bookings overlap." "This tree stays balanced." "No duplicates." "Every child points back to its parent."*

Constraints that must hold over a data structure after every operation. Tractable when the data is algebraic (records, lists, sets, maps, trees) rather than unbounded strings.

**Recognize it when:** you have a data structure with a "should always be true" property that is currently enforced by hope, scattered assertions, or a validation function you're not sure is complete. Invoice math, calendar/booking systems, balanced structures, graph consistency, referential integrity in memory — all here.

### 4.4 Idempotency / commutativity / ordering

*"Applying this twice equals applying it once." "These two operations commute." "The merge is order-independent."*

The heart of correctness in event-driven and distributed systems, where messages arrive twice, out of order, or concurrently. These algebraic properties are tractable for structured operations and catch bugs that are nearly impossible to find by testing (they only appear under specific interleavings).

**Recognize it when:** you have retries, at-least-once delivery, event replay, CRDT-like merges, caches, or "we process this webhook and sometimes it fires twice." Every "it should be safe to run this again" is an idempotency claim; every "it shouldn't matter which order these arrive in" is a commutativity claim.

### 4.5 Exhaustiveness / totality

*"Every case is handled." "No reachable state is stuck." "This function is defined for all inputs."*

Coverage-of-cases properties. Tractable, and they eliminate whole categories of bug (the unhandled enum variant, the deadlocked state, the input that falls through every branch).

**Recognize it when:** you have a `switch`/`match` over an enum, a set of states with transitions, a parser or dispatcher. The question "are we *sure* there's no input / state / event that nothing handles?" is a totality obligation.

Across all five: **tests sample; loom quantifies.** E-0004 loop 1 showed the difference live — the universal claim caught the no-AC → done path that the *examples* had missed. That is the thing tests structurally cannot do, and it is the whole reason to reach for loom on a property in this catalogue.

## 5. Recognition — who spots these, and how

A catalogue is only useful if someone can match their system against it. Two paths, both real:

- **The experienced engineer recognizes them directly.** Someone who knows the domain reads §4 and thinks "the order state machine, the entitlement check, and the invoice math — those three." This is the fastest path and the catalogue is written to trigger it (each class leads with the plain-language shape, then the "recognize it when").
- **The newcomer or junior asks an LLM to surface them.** Someone unfamiliar with the codebase — or new to the pattern language — can point an LLM at the code with the catalogue and ask *"which of these shapes appear here, and where?"* The LLM is well-suited to this: it reads breadth, it knows the patterns, and it returns candidates a human then judges. This turns recognition from a rare expert skill into an assisted, checkable step.

This second path matters more than it first appears. **The single biggest determinant of loom's breadth is not the verification frontier — it is recognition.** The properties loom is good at are valuable, but *seeing* that "my flaky order bug" is really "an unenforced state-transition invariant" is a skill many engineers lack. If an LLM can reliably surface loomable properties from a codebase — and if the human can cheaply confirm each candidate against the catalogue — then loom's audience is every engineer, not just those who already think in invariants. That is a product and LLM-scaffolding problem, not a Dafny problem, and it may be the highest-leverage thing loom can invest in.

## 6. How a non-expert steers a formal layer they cannot read

The worry: a non-expert has no intuition for what is provable, so loom will feel like a lot of work for uncertain payoff. The architecture's answer is that **they do not predict tractability — the gap report reports it.** They write intent and examples (E-0004 confirmed: zero Dafny read); the tool returns "proved / couldn't prove / here is a counterexample." The tool *degrades honestly* instead of failing opaquely, and that honesty is precisely what makes it safe to hand to someone who cannot read the formal layer.

Two trust rails carry a non-expert (detailed in [`loom-loop-poc.md`](../loom-loop-poc.md) §3): **examples are mechanically checked against the claims** (a claim that disagrees with your `input → expected` is caught without reading Dafny), and **the back-translation is audited against intent** (you check the LLM's English account of each claim, not the Dafny). Neither closes the gap fully; the gap report is where the residual lives, made visible.

There is a **floor that never disappears:** concrete-example checking stayed tractable *everywhere* in E-0004 — flat and recursive, both rungs. Even when the universal proof is out of reach, the non-expert always gets "your examples pass or fail against the real code." That alone beats nothing, and it never times out.

But E-0004 found the real wrinkle, and this note will not paper over it: **on strings, a category-(B) failure is not self-diagnosing** — "real gap" and "too hard to prove" look identical to the reader. That is a genuine UX hole for a non-expert, and closing it is core to making loom broad. The next section is largely about how.

## 7. Is this the limit? No — the loop we tested was the thinnest one

E-0004 deliberately used **blind subagents and zero proof automation** — the cheapest possible loop, chosen to test the idea at minimum cost. The distance between *that* and the expert-Dafny ceiling is the tool's entire engineering surface, and it is wide. Three levers, roughly in order of leverage:

1. **Auto-apply the proof playbook.** The induction hint a *blind author* cannot write, a *tool* can attempt mechanically — standard tactics (`{:induction}` attributes, fuel tuning, common lemma templates) tried automatically before giving up. Every case that moves is a category-(B) → (A) promotion, widening the push-button zone without the human doing anything.
2. **Bounded fallback when full proof fails.** When "for all strings" is out of reach, *"I checked every string up to length 12 and found no counterexample"* is enormous value for a non-expert — and it sidesteps the induction wall entirely. It also fixes the self-diagnosis hole from §6: a bounded check either finds a witness (a real gap, with a concrete example) or does not (likely fine). This turns the string cliff into a graceful slope.
3. **Choose the substrate per property shape.** The umbrella architecture — prose + examples + LLM-authored formal + gap report — is *backend-agnostic*. E-0004 exercised only the Dafny path and said nothing about the others (see §8).

So loom's reach is not "full proof where Dafny is push-button." It is a **spectrum — full proof where tractable, bounded proof where not, concrete examples always — with the gap report honestly reporting which rung each property landed on.** That spectrum is much broader than the corner E-0004 tested, and the architecture already describes it.

## 8. The substrate landscape

"This depends on Dafny" is only half true. The umbrella's formal layer can target different backends, and the right one depends on the property shape. A sketch of the landscape:

- **Dafny (auto-active + Z3).** Sweet spot: code-level contracts, data-structure invariants, algebraic properties over structured data; compiles to multiple languages. Wall: universal properties over recursive functions on unbounded data (the E-0004 string result). Best for the §4.3 and much of §4.1/§4.5 classes.
- **SMT directly (Z3 / CVC5).** Same underlying power and the same wall, with less ergonomic specification. Rarely the right *surface* for a non-expert, but it is the engine under most of the others.
- **Bounded model checkers — TLA+/TLC, Alloy.** For state-machine and protocol properties (§4.1, §4.4), these are *counterexample-driven* and need *no induction*: they explore states up to a bound and hand back a concrete trace when a property breaks. For a non-expert, "here is a 3-step sequence that violates your rule" is dramatically more useful than a failed proof. This may be a *better* substrate than Dafny for the state-machine class specifically — and E-0004 never tested it.
- **Property-based / bounded-exhaustive testing.** The mechanized form of §7's lever 2. Not proof, but honest partial evidence with concrete counterexamples, and it never hits the induction wall. A natural floor beneath any proof attempt.
- **Interactive provers — Lean, Coq, Isabelle.** Strictly more powerful (they can prove anything true), but expert-heavy: you write the proof. Wrong tool for a non-expert audience; right tool only at the far edge where the stakes justify a specialist.

The strategic point: **the substrate should follow the property shape, and loom's value proposition does not depend on winning the hardest proofs.** It depends on routing each property to a backend that can say something honest and useful about it — proof, bounded proof, or counterexample — and reporting the result at the right confidence level.

## 9. What loom is *not* — the honest limits

To keep the ambition disciplined:

- **Not a push-button prover for arbitrary code.** No such thing exists. loom targets property classes, not whole programs.
- **Not a way to eliminate the residual.** The verifier checks impl against claims; it never proves the claims *capture intent*. Examples pin the cases you thought of; back-translation pins what the LLM admits it wrote; neither proves the spec is right in a region you did not example and did not say. loom's honest claim is that it makes the residual *visible and shrinkable*, not gone ([`loom-loop-poc.md`](../loom-loop-poc.md) §7).
- **Not automatic recognition (yet).** Matching a system to §4 is assisted, not free. The LLM-surfacing path (§5) is a bet, not a proven capability.
- **Not a replacement for tests.** loom earns its keep on *universal, high-stakes* properties in the tractable zone. For everything else — the vast bulk of code — tests remain the right tool. loom should *steer* you here, not pretend everything is loomable.

## 10. Positioning and open directions

The sharp version of the whole note: **E-0004 is the limit of *one* loom — the push-button-full-proof-of-arbitrary-code one, which was never viable and never the goal. It is nowhere near the limit of the graded, honest, multi-substrate loom that spans proof → bounded-check → examples and routes by property shape.** The qualified proceed was right; the qualifier just tells us which loom to build.

Directions this note opens, for later discussion (not decided here):

- **Test the spectrum, not more Dafny-on-strings.** Take one state-machine property and one data invariant; run the loop with a bounded-check fallback and a counterexample-driven backend for the state machine. The question: does a non-expert get useful, honest results *across* the tractability line?
- **Test recognition.** Can an LLM reliably surface §4 properties from a real codebase, at a precision a human can cheaply confirm? This may bound loom's audience more than any verification result.
- **Decide the substrate-routing question.** One backend (Dafny) with a bounded fallback, or genuine per-shape routing (Dafny for data, a model checker for state machines)? This shapes the thin tool's architecture.

Each is cheap relative to the cost of building the wrong tool. The recognition question, in particular, is the one that most directly decides whether loom is broad or narrow — and it is the one E-0004 did not touch at all.
