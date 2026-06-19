# Containment, Not Solution

> On reliable systems around probabilistic workers.

## 1. The frame

Some part of a system's output is being authored by something probabilistic — a machine-learned model, a stochastic search procedure, a human under time pressure. The output is intended to be acted on. What is the shape of this problem, and what can be honestly said about its solubility?

This essay is about the structure of the question, separated from any specific tool, language, or substrate. It argues three things.

First, the regress of "who checks the checker" is bounded, not infinite. Every working verification arrangement terminates in a trust root, and the question is not whether the trust root exists but whether it is cheap to defend.

Second, almost every architecture for reliable probabilistic authorship places a human at some residual checkpoint, and the human is the load-bearing weakness. Its three failure modes — capacity, competence, incentive — have different tractabilities, and conflating them is one of the most common design errors.

Third, the probabilistic character of the authoring system is permanent. But this is the wrong thing to fight against. The right project is systems engineering around an unreliable worker, of which we have centuries of practice in other domains. The unreliability of the worker is not the bottleneck on whether the system can be made reliable; the design of the system around the worker is.

## 2. Solved versus contained

Most working engineering does not solve problems. It contains them.

Cryptography is not unbreakable; it is contained by sizing key spaces and rotating. Operating systems are not exploit-free; they are contained by sandboxing and least privilege. Aviation is not crash-free; it is contained by redundant systems, checklists, and incident review. The discipline in each case is: where is the residual risk, can it be seen, can it be kept from compounding silently.

This is a different orientation than the word *reliable* invites. Outsiders hear "make it reliable" and imagine an endpoint. Practitioners who actually build reliable systems are continuous; they treat residual risk as a permanent feature of the world and design to make it locatable.

When evaluating any proposal for reliable use of probabilistic authorship, the first question is whether its implicit goal is solution or containment. Proposals aimed at solution will fail; proposals aimed at containment can succeed. The two are easy to confuse because they make similar promises in the short run and diverge only when stressed.

## 3. Trust roots and bounded regress

The objection that any verifier needs a verifier needs a verifier — that we are condemned to infinite regress — feels devastating. It is not.

Every working verification arrangement has a *trust root*: something it stops checking. Compilers trust the bootstrap. Mathematics trusts its axioms. Empirical science trusts measurement. We do not eliminate trust roots; we choose them so that they are small, stable, narrow, and external to the system being checked.

A good trust root is:

- **Small enough to audit.** Code or assumptions a person can read in finite time.
- **Stable enough to age.** Not changing with every release of the thing it checks.
- **Narrow enough to test.** Behavior reducible to mechanical regression.
- **External enough not to be co-opted.** Independent of the system whose output it is checking, so that the worker cannot game the checker by adapting its output.

The regress terminates when these conditions hold. In practice it terminates in two or three steps, not in infinity. The mathematicians stopped at axioms and got on with it.

The substantive question is therefore not whether the regress ends but whether the trust roots in any specific arrangement are cheap to defend. If the chosen termination point is itself large, opaque, or produced by the same probabilistic process that authored the output, then the regress is effectively unbounded — not because regress is infinite in principle but because the chosen termination is no better than what it terminates. Self-checking by the same probabilistic system fails this test by construction. Self-checking by a different probabilistic system trained on the same distribution mostly fails it too. The trust root has to come from somewhere structurally different from the worker.

Choose trust roots well and the problem becomes finite. Choose them poorly and the appearance of verification masks an unbounded regress.

## 4. Independence of failure modes

The criterion that makes a trust root cheap to defend reduces, on inspection, to a single property: *its failures do not correlate with the worker's*.

This is why the four characteristics in §3 — small, stable, narrow, external — all matter. Each is a way of buying independence. A small checker can be audited for the failure modes the worker has; a stable one does not drift toward the worker's shape; a narrow one is testable against the worker's specific failure cases; an external one is built from a different substrate, so its mistakes have a different distribution. The shared underlying property is that when the worker is wrong, the checker should not also be wrong in the same way.

This sharpens what counts as redundancy and what does not. Adding more reviewers does not improve reliability if the reviewers share failure modes — eight copies of the same blind spot is one blind spot, not eight. Replicated probabilistic workers checking each other typically share failure modes by construction; they were trained on similar distributions, exposed to similar examples, and tend to be wrong on the same inputs. Adding more of them produces the appearance of redundancy without the substance.

The substantive test for any verification arrangement is therefore not "how many checks are there" but "how independent are their failures." An arrangement with one mechanical check whose errors are uncorrelated with the worker's is stronger than an arrangement with eight probabilistic checks whose errors all correlate. Designs that count checks rather than measuring independence produce reliability theatre — visible mechanism, invisible blind spots.

The practical consequence is that independence is the metric to optimise when arranging verification. Anything else — number of reviewers, depth of review, prestige of checker — is a proxy that holds only when independence does. Once the independence assumption fails, the proxies become misleading without warning.

## 5. The human as the residual checkpoint

Almost every architecture for reliable probabilistic authorship places a human at some residual checkpoint — reviewing a summary, approving a claim, signing off on a gate. The human is where the regress finally terminates in trust. This is the load-bearing weakness, and its three failure modes are not equally tractable.

**Capacity.** Humans do not have time to review every artifact carefully. This is tooling-solvable. Keep the artifact small. Surface only what has changed. Direct attention to the highest-leverage parts. Code review at scale already does this; reviewers do not read every line, they read the diff and apply judgment. Capacity-bound review is a solved problem in adjacent disciplines; the work is to import the practices, not to invent new ones.

**Competence.** Humans may not understand the artifact they are signing off on — the language is unfamiliar, the formalism is opaque, the abstraction is unintuitive. This is the historically fatal failure mode for high-rigor disciplines, and it is the one most likely to silently undo the rest of an otherwise sound architecture. It is partially addressable in two ways. The shape of the artifact can favor things humans already know — natural language, diagrams, tables, languages with existing communities — over inventing notation that the reviewer must learn before signing. And the same probabilistic worker that authored the artifact can be used to *explain* it in familiar terms, where the explanation is checked independently against the formalism. Explaining is a different task than authoring; the worker's failure modes on the two are different, which is what makes this composition non-circular. The composition only holds, however, if the human reviews the artifact through some channel that is not itself worker-mediated; if the human signs off on the explanation alone without engaging with the underlying artifact, the apparent two-stage review collapses to one source and the independence is illusory.

The competence failure mode is the one designers most commonly under-budget for. Beautiful architectures fail at this layer routinely because the artifact the human is asked to sign is in fact unreadable to the human, and so the sign-off becomes ceremonial.

**Incentive.** Humans rubber-stamp things to ship faster. This is not technically solvable. No tool fixes it. The most a tool can do is make the rubber-stamp visible and durable — record who approved what, under what evidence, when — so that incidents are traceable and the surrounding culture has artifacts to work with. The tool gives the organization materials to enforce honest review; it cannot enforce review itself.

The trap is to design the technical pieces and assume the human picks up the slack. That assumption is the most reliable predictor of failure for any architecture that depends on human-in-the-loop review. Architectures that survive contact with practice make the human's job small, concentrated, and well-instrumented, and remain honest about the residual risk that humans approve without engaging.

## 6. Why probabilistic authorship is permanent

A common hope is that the probabilistic worker will improve enough — through more training, better calibration, more self-checking — that we will no longer need anything outside it. This hope is structural confusion.

Any predictor that learns from a finite sample of a continuous, ambiguous task distribution has nonzero error rate. This is not a maturity problem with current systems; it is an information-theoretic property of prediction. There will never be a probabilistic authoring system whose outputs can be relied on, in the strong sense, without external check. Not in this generation, not in any future generation. The unreliability is not a bug being fixed; it is a feature of what these systems are.

But the framing "will the probabilistic worker become reliable enough" is the wrong question, and unpacking why is the most important move in this essay.

We do not rely on humans being mistake-free. Surgeons make mistakes. Civil engineers approve flawed designs. Financial analysts miscalculate. We handle this by *systems engineering around the unreliable worker*: checklists, redundancy, independent verification, sandboxing, blast-radius containment, post-incident review. The worker remains unreliable; the system around the worker becomes reliable.

Probabilistic authoring systems are workers. The reliability question is about the system they are embedded in, not about the worker itself. The systems-engineering tradition for human workers is centuries old; the equivalent tradition for probabilistic ones is in its first decade. The difficulty of the work ahead is real; the impossibility some people read into it is not.

Asking "will the worker become reliable enough to trust" is like asking "will surgeons become accurate enough to operate without checklists." It is the wrong shape of question. The right question is: *what is the smallest possible surface area of probabilistic output that we rely on without external check, and how do we keep it small?*

## 7. Locatability as the metric

If solution is unavailable and containment is the goal, what is the metric?

This essay proposes *locatability*: the degree to which residual risk in the system has a known, named, inspectable home. A system in which probabilistic mistakes can land anywhere has poor locatability — risk is diffuse, invisible, impossible to monitor. A system in which probabilistic mistakes are funneled into a small number of named artifacts that are explicitly reviewed has good locatability — risk is concentrated, visible, tractable.

Locatability is not a single number. It is a posture. It manifests as:

- Small, named artifacts that carry the load-bearing claims, separable from the rest of the system.
- Explicit visibility of what is being relied on without external check — uncovered ground treated as first-class output, not as silent assumption.
- Durable trails of who approved what under what evidence, so that incidents teach the system.
- Tooling that directs human attention to high-leverage review and away from ceremonial sign-off.

Most current practice has poor locatability. The risk is everywhere, the artifacts are large, the uncovered ground is implicit, the human role is ceremonial. The improvement available from current practice to better locatability is large even though the absolute reliability does not approach "solved." This gap — between the practical improvement available and the theoretical impossibility of solution — is the space in which honest work on this problem lives.

A useful test for any proposal is to ask: *after this system is in place, where would the next failure most likely come from, and would I see it before it caused damage?* A system with good locatability has answers to both halves. A system with poor locatability has answers to neither.

## 8. The two traps

There are two symmetric traps to avoid, and architectures fail by falling into one or the other with depressing regularity.

The first is believing the problem can be finished. It cannot. Probabilistic workers will not become reliable enough that they need no external check. Trust roots will not collapse to zero. Humans will not stop being the residual weakness. Architectures that promise solution are selling something they cannot deliver, and they erode the discipline of containment by making the goal sound like an endpoint that can be reached and then stopped working on.

The second is believing that because the problem cannot be finished, the work is not worth doing. This is the same error in reverse. Containment is not a consolation prize. It is the actual nature of every reliable system humans have ever built. The work of moving residual risk from "anywhere in the system" to "specific named places where humans can focus attention" is not a fallback from solving the problem — it is solving the problem, in the only sense the problem admits.

The honest orientation is neither solution nor despair. It is *risk relocation as a permanent practice*: choose trust roots well, make the human's job small and instrumented, accept that the worker remains probabilistic, and judge the system by the locatability of its residual risk rather than by any promise of reliability that nothing of this shape can deliver.

A reliable system around an unreliable worker is achievable. A worker reliable enough not to need a system is not. The discipline is to keep these two propositions clearly distinguished, and to design for the first without ever pretending to be designing for the second.
