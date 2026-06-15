# The Verifiable Umbrella: A Three-Layer Model for Human–LLM Software Construction

**Prose as Memory, Claims as Contract, Verified Implementation as Output**

*Author Name* · *Affiliation* · *Contact*

**Draft. Not yet submitted.**

---

## Abstract

LLM-assisted software development is becoming widespread but lacks a stable, verifiable intermediate artifact between human intent and generated code. Conversational generation produces flexible but unverifiable output; spec-driven approaches produce verifiable but rigid artifacts that do not accommodate exploratory thinking; current toolchains were designed for human-only workflows and adapt poorly to mixed human–LLM authorship. We propose a three-layer architectural model: prose serves as the human's durable memory; an *umbrella* of formal claims serves as a verifiable shared artifact between human and LLM; *siblings* — LLM-authored implementation modules — satisfy the umbrella's obligations under mechanical verification. We further propose *bidirectional refinement*, in which obligations flow downward from umbrellas to implementations while properties flow upward from implementations to umbrellas, with the compiler reporting gaps where claimed and proved properties diverge. We describe one realization (*Loom*), present a strategy for incremental prototype construction using existing tooling, and discuss what such a system would and would not address. The contribution is a model, not a prescription: alternative realizations faithful to the architecture are invited.

**Keywords:** Human–AI collaboration; Software architecture; Refinement types; Specification languages; Programming languages; Verified compilation; Spec-driven development.

---

## 1. Introduction

The integration of large language models into software development has progressed rapidly from line-level autocomplete to agentic systems capable of writing, refactoring, and reasoning about substantial codebases. This progress has produced real productivity gains but has also surfaced characteristic failure modes that suggest the underlying *collaboration architecture* is incomplete — that current tools, methodologies, and artifact structures were designed for an era when one kind of intelligence (human) authored code, and they adapt poorly to an era when two kinds of intelligence (human and machine) co-author.

Three failure modes recur in practice. First, *unstructured generation*: producing code through free-form conversation with an LLM. The output is fast and often locally plausible but lacks any verifiable connection to the human's intent. Bugs, security holes, and architectural inconsistencies accumulate silently because nothing in the workflow forces the human's intent and the model's output to be reconciled. Second, *specification drift*: when teams adopt spec-driven approaches alongside LLM tools, the specifications and the generated code diverge over time, and neither tooling nor methodology maintains their consistency. The specification becomes documentation that no one fully trusts. Third, *cognitive overload*: humans supervising LLM-generated code at scale lose the practical ability to verify whether the code matches their intent. They gradually defer judgment to the model, at which point they are no longer meaningfully in the loop — the model is writing code the human ostensibly approves, but the approval has become ceremonial.

These failure modes share a structural cause: there is no stable, mutually-readable, mechanically-verifiable artifact *between* the human's intent and the LLM's output. Prose is the human's natural representation but cannot be verified. Code is the LLM's natural output but is too large and too detailed for humans to fully audit at the rate the LLM produces it. The gap between them is currently bridged by ad-hoc test suites, comments, and trust — none of which scale to the rates of generation that modern LLMs make possible.

This paper proposes an architectural model addressing this gap. The model has three layers:

- **Prose**, in which the human reasons, explores, and remembers. Prose accumulates over hours, days, or weeks as the human thinks aloud, often in conversation with an LLM. It is informal, long-form, and durable.
- **An umbrella**, a formal structure of *claims* — types, relations, examples, proofs, capabilities, obligations — that serves as the verified shared artifact between human and LLM. The umbrella is the focal artifact: small enough to be read in its entirety by the human, structured enough to be manipulated mechanically by tooling, formal enough to support verification of its relationship both to the prose above and the code below.
- **Siblings**, LLM-authored implementation modules that satisfy the umbrella's obligations under mechanical verification. Siblings are typically not read in detail by the human; instead, the human trusts the verification report and drills into siblings only when something fails or when curiosity drives them.

We further propose *bidirectional refinement* as the verification discipline that holds the three layers together. Obligations flow downward: the umbrella declares what implementations must satisfy. Properties flow upward: implementations expose proved guarantees, which the compiler summarizes into umbrella-level claims. The compiler verifies both directions and reports the gap — properties claimed but not proved (so the human can either weaken the claim or strengthen the implementation) and properties proved but not claimed (so the human can decide whether the surprise is welcome). This bidirectional structure means neither the human nor the LLM has to trust the other on faith. The verification artifact is what they trust.

The model is realized in one concrete language design we call *Loom*, which we describe as an existence proof rather than a prescription. Loom commits to specific choices: a term-based grammar with period-terminated forms; sections-as-registers (claim, example, implementation, proof) within a unified grammar; a hierarchical filesystem-as-architecture with semantically meaningful sibling and tree relationships; capability-tracked effects; phased adoption (sketch, draft, settled); capability-honest multi-target compilation. The underlying three-layer model with bidirectional refinement could be realized in many ways, and we explicitly invite alternative realizations.

Finally, we describe a prototype strategy that uses existing tooling — structured Markdown, a verifier in a host language, embedded scripting for implementation, LLM API calls, and an editor extension — to test the model's core claims without first building a new programming language. The prototype is sufficient to evaluate whether the three-layer model with bidirectional refinement actually changes how humans and LLMs collaborate on software systems. If yes, the more aesthetic question of bespoke language design becomes worth answering. If no, a substantial speculative project has been avoided.

### 1.1 Contributions

This paper contributes:

1. A diagnosis of current failure modes in human–LLM software collaboration, located in the absence of a stable verifiable intermediate artifact.
2. A three-layer architectural model — prose, umbrella, siblings — with bidirectional refinement as its verification discipline.
3. Supporting design patterns: sections-as-registers within a unified grammar; filesystem-as-architecture with sibling/tree semantics; phased adoption (sketch, draft, settled); capability-honest multi-target compilation.
4. A worked example realization (Loom) sufficient to demonstrate the model concretely.
5. A prototype strategy that allows incremental empirical evaluation of the model using existing tools before committing to bespoke language design.
6. An honest accounting of the model's limitations and the empirical questions that remain open.

The contribution is best understood as a model — an architectural framing that subsequent work can realize in many ways. We do not claim Loom is the *right* realization; we claim the three-layer structure with bidirectional refinement is a productive frame for further research and tool-building, and we offer concrete design patterns that any realization will need to address.

### 1.2 Scope and non-claims

We deliberately do not claim: that this model will produce a single dominant programming language; that mass adoption is a target; that bidirectional verification eliminates all failure modes of human–LLM collaboration; or that the empirical questions raised here are settled. We claim only that the architecture is coherent, that one realization is sufficient to demonstrate it, and that a tractable prototype path exists for empirical evaluation.

---

## 2. Related Work

The proposed model draws on, and aims to synthesize across, several established research traditions. We discuss the most directly relevant.

### 2.1 Literate programming and structured documentation

Knuth's *literate programming* (Knuth, 1984) proposed that source code and documentation should be authored together in a single artifact, with the human-facing narrative as the primary structure and the machine-facing code as derivable from it. Subsequent systems — Noweb, CWEB, more recently org-mode, Jupyter notebooks, and Quarto — have realized variants of this idea in different settings. The model proposed here is in the spirit of literate programming but goes further in two respects: the intermediate artifact (umbrella) is itself formal and verifiable, not merely descriptive prose; and the relationship between layers is mechanically maintained rather than authorial discipline.

### 2.2 Design by contract and refinement

Eiffel's *design by contract* (Meyer, 1992) introduced the notion that classes and methods carry pre-conditions, post-conditions, and invariants as part of their interface, checked at runtime. The B-method (Abrial, 1996) and Event-B (Abrial, 2010) extend this to systematic refinement of specifications into implementations, with mechanical proof obligations at each step. Refinement types (Vazou et al., 2014; Rondon et al., 2008) extend type systems to express predicates on values, checked at compile time via SMT. The umbrella in our model is conceptually downstream of all of these: it carries contracts that implementations must satisfy, and the verification machinery is essentially refinement-based.

### 2.3 Dependently typed languages and proof-carrying code

Idris (Brady, 2013), Agda (Norell, 2007), Coq (Bertot & Castéran, 2004), and Lean (de Moura et al., 2015) allow programs to carry their proofs as types. F* (Swamy et al., 2016) is closer to our setting: a general-purpose language with refinement types and SMT-backed verification, used for systems programming. The model here uses refinement-type-style verification at a structural level (per claim, per example, per relation), rather than at the level of individual functions, and emphasizes the *bidirectional* relationship between claimed and proved properties rather than only the downward direction.

### 2.4 Specification languages

TLA+ (Lamport, 1994) and Alloy (Jackson, 2002) provide formal specification languages oriented toward modeling and analyzing systems at the architectural level. Specifications in these languages are checked via model-checking or constraint solving. The umbrella in our model is broadly TLA-shaped — it specifies behavior at an architectural level — but is meant to be operationalized into running code via verified compilation, not merely model-checked in isolation.

### 2.5 Logic programming and term-based homoiconicity

Prolog (Colmerauer, 1990) and its descendants treat programs as collections of clauses — facts and rules — operating over terms. Erlang (Armstrong, 2003), descended from Prolog's syntactic family, retains the term-based grammar and adds an actor-model runtime with hot reload, supervision trees, and distribution. Our example realization (Loom) draws heavily on this lineage for surface syntax and for the actor runtime, while adding the umbrella structure above it. Mercury (Somogyi et al., 1996) and Picat (Zhou et al., 2015) explore typed logic programming, which is also relevant.

### 2.6 Actor model and supervised runtimes

Hewitt's actor model (Hewitt et al., 1973) underpins much modern thinking about concurrent and distributed systems. Erlang/OTP (Armstrong, 2003) is the most influential actor runtime in production use, and its patterns — supervision trees, hot code loading, "let it crash," distributed nodes — have been replicated in Akka, Orleans, and others. The runtime layer of our model assumes an actor-shaped substrate, and the proposed multi-target compilation strategy treats BEAM (the Erlang/Elixir virtual machine) as the most semantically aligned target, with capability profiles for other runtimes.

### 2.7 LLM-assisted programming

Modern LLM-assisted programming tools — Copilot (Chen et al., 2021), Cursor, Claude Code, Aider, Continue, and others — provide line-level, block-level, or agentic generation of code. They are powerful but operate on the existing artifact structure of conventional programming languages, with no intermediate verifiable layer between human intent and generated code. v0.dev and similar conversational UI generators (Vercel, 2023) explore conversation-as-input for code, but again without verification. Our proposal is complementary: it does not replace these tools but proposes a different *artifact structure* that they could target, with verification as a first-class output.

### 2.8 AI-assisted formal specification

A line of recent work uses LLM agents to *generate* formal specifications from informal input rather than to generate code directly. Fakhoury et al. (2024) present 3DGen, a framework that transforms natural-language documents (RFCs) and example inputs into specifications in 3D, a domain-specific formal language for binary parsers, from which provably correct parser code is then extracted via the EverParse line. The problem statement that opens the present paper — that distilling informal requirements into formal specifications is challenging, and that new formal languages are hard to learn — appears as the explicit motivation in their abstract. The architectural shape is also recognizable as the three-layer flow we propose: informal input, formal intermediate, verified implementation, with the LLM mediating the first transition. We take 3DGen to be the closest existing precedent for the model proposed here, narrowed to one domain. The umbrella model generalizes this approach to arbitrary software, with the trade-offs discussed in §9. A second move 3DGen makes is worth flagging: their verification strategy uses an external oracle (often a reference implementation) to validate synthesized test inputs, which closes part of the LLM-authorship attack surface in a way the present paper does not address (see §9.7).

### 2.9 Spec-driven development

Recent industry practice has experimented with *spec-driven* approaches in which LLMs write specifications first and code second, with the specification persisting as a stable artifact. This is closer in spirit to our proposal than vibe coding is, but typically lacks two features the proposed model emphasizes: mechanical verification linking spec to code, and bidirectional refinement allowing specs to be checked against what implementations actually deliver. The proposed umbrella is essentially a verifiable, bidirectionally-refinable specification.

### 2.10 Provenance, CRDTs, and version control

The collaborative editing aspect of the proposed model draws on CRDTs (Shapiro et al., 2011) for conflict-free concurrent editing and on patch-based version control (Pijul, Darcs) for finer-grained semantic merging than Git's line-based three-way diff. The proposed event-sourced semantic VCS is not a novel contribution per se but is a necessary substrate for the model's collaboration story.

### 2.11 Positioning

What distinguishes our proposal from the prior art is not any single component but the synthesis. The umbrella is recognizably descended from refinement types, design by contract, and TLA-style specification, but it is positioned specifically as the *focal artifact* for human–LLM collaboration, with bidirectional verification as the discipline that keeps the human and machine accountable to each other. We have not seen prior work that frames the architecture this way. The closest precedents — literate programming, F*, TLA+, Idris, and at narrow scope 3DGen — each address parts of the problem; modern LLM tools address the collaboration aspect but without verifiable intermediate artifacts at the architectural level we propose.

---

## 3. Thesis: The Verifiable Intermediate Artifact

We state the central thesis precisely.

> Human–LLM collaboration on software systems requires a *third* artifact, distinct from prose and from code, that is small enough for humans to read fully, structured enough for machines to manipulate mechanically, and formal enough to support verification of its relationship both to the prose above it (faithful distillation of intent) and the code below it (faithful implementation of contract).

We call this third artifact the *umbrella*. It is the load-bearing element of the proposed architecture. Without it, the human and LLM have no stable shared ground: prose is too informal for the LLM to reliably verify it has understood; code is too detailed for the human to reliably verify the LLM has implemented their intent. With it, both parties have a common artifact whose properties are mechanically checked.

The umbrella is not a specification in the traditional sense, because it is not written once and then handed off. It is the *living surface* the human edits, the LLM consults, the compiler verifies, and the runtime instruments. It accumulates structure over the lifetime of the system. Changes to it cascade in both directions: prose paragraphs that gave rise to a claim are linked from that claim; implementation modules that satisfy a claim are regenerated when the claim refines; properties proved by implementations are summarized upward into the umbrella's own claim language, with gaps reported.

### 3.1 Why this layer is necessary

Three arguments support the necessity of an intermediate verifiable artifact.

*Cognitive load asymmetry.* LLMs can produce code at rates far beyond what humans can carefully read. If the human is to remain in the loop without becoming a rubber-stamp approver, they need an artifact whose size grows much more slowly than the codebase grows. The umbrella is that artifact: the codebase doubles, the umbrella adds a few claims. The human reads the umbrella; the code is verified mechanically against it.

*Bidirectional trust.* For collaboration to be honest, the LLM should not be trusted on the human's terms alone (verify-the-output) and the human should not be trusted on the LLM's terms alone (assume-intent-is-clear). Both directions need mechanical checks. The umbrella supports both: the LLM checks that its output satisfies the umbrella's obligations; the compiler checks that the umbrella's claims are grounded in proved properties of the implementation.

*Memory across sessions.* Conversations end; codebases persist. For the collaboration to survive across sessions, hand-offs to other collaborators, and changes of personnel (human or model), the durable memory of the system's intent must be in an artifact, not in conversation context. The umbrella is durable, versionable, diffable, and amenable to provenance tracking. The prose is preserved alongside it but is recognized as authorial reasoning rather than as the system's specification.

### 3.2 What the umbrella is not

The umbrella is not the implementation. It is not the documentation. It is not a test suite, though examples in the umbrella function as tests. It is not the prose, though the umbrella distills the prose. It is the *contract* — what the system claims about itself, what it promises to satisfy, and what its implementations are obligated to deliver. Everything in it is meant to be both read by humans and checked by machines.

---

## 4. Conceptual Model

We now describe the proposed model in structural detail. We use Loom-flavored examples for concreteness but emphasize that the model is realization-agnostic.

### 4.1 Claims as the unifying notion

Every element of the umbrella is a *claim*. A claim is a statement that something is true, accompanied by evidence and provenance. Different kinds of claims correspond to different kinds of evidence:

- **Type claims** assert that an entity has a certain shape (e.g., `Money :: nonneg int`). Evidence: structural definition.
- **Relation claims** specify the relationship between inputs, outputs, and state for an operation (e.g., a transfer operation requires the source balance to be sufficient). Evidence: precondition and postcondition predicates.
- **Example claims** specify a concrete input-output-state transition that must hold (e.g., transferring 30 from an account with 100 yields an account with 70). Evidence: a runnable example, mechanically checked.
- **Property claims** specify a universal property over inputs (e.g., the sum of balances is conserved across any transfer). Evidence: a proof obligation discharged via SMT, model checking, or property-based fuzzing.
- **Obligation claims** declare requirements that submodules must satisfy. Evidence: per-submodule satisfaction checks.

Crucially, all five kinds share a uniform structure: each has an identity (a stable ID independent of textual form), a back-link to the prose paragraph it derives from, a verification status (proved, refuted, unchecked, gap), and a list of authors (humans, LLMs, or both) with timestamps.

### 4.2 Sections as registers within a unified grammar

The umbrella organizes claims into sections — `knows`, `relates`, `shows`, `does`, `proves`, `uses`, and others depending on the realization. These sections are not different sub-languages; they are *registers* of one grammar. The same syntactic primitives (terms, predicates, named arguments, pattern matching) appear in all sections, but each section naturally draws from a different vocabulary: `knows` from type-shape vocabulary, `relates` from predicate vocabulary (`when`, `and`, `so`, `requires`, `ensures`), `shows` from narrative vocabulary (`given`, `do`, `yields`), `does` from operational vocabulary (`then`, pipe-style composition, pattern match), `proves` from logic vocabulary (`for-all`, `=>`, named lemmas).

This register approach avoids the cost of multiple parsers or sub-DSLs while letting each section communicate in the idiom that fits its purpose. It also makes umbrellas readable: a human can scan an umbrella top to bottom and recognize each section by its register, without needing to learn the formal semantics of multiple languages.

### 4.3 Provenance as a reified bridge

Every claim points back to the prose that gave rise to it. Every prose paragraph can be queried to determine whether (and which) claims it produced. This bridge is mechanical, not optional: when a claim is created (by human or LLM), the system records the source paragraph(s) and timestamps. When prose is rewritten, the system attempts to map the rewrite onto existing claims by ID rather than deleting and recreating. The result is that the human can always ask "why is this claim here?" and get a back-link to the prose that justified it.

This reified bridge also supports a second-order discipline: prose that has not been distilled into any claim is recognizable as *intent without realization*. The system can list paragraphs from which no claim has been drawn, allowing the human to decide whether each is commentary (intentional non-distillation) or oversight (intent that should be formalized).

### 4.4 Bidirectional refinement

The verification discipline of the model is bidirectional.

**Downward (obligation flow).** Each layer of the umbrella imposes obligations on its children. The compiler checks that child modules provide the relations, examples, and proofs required by their parent umbrella. If a child fails to satisfy an obligation, the system refuses to accept the module.

**Upward (property flow).** Each child module exports its proved properties. The parent umbrella has an explicit mapping (e.g., `summarizes from contains`) that connects child-level properties to parent-level claims. The compiler walks the tree bottom-up and computes the actual proved properties at each level.

**Gap reporting.** When a parent's claim cannot be fully grounded in its children's proved properties, the compiler reports the gap. Example: an umbrella claims "customer funds are safe," but the children only prove non-negativity, conservation, and audit-logging — not fraud detection or rate-limiting. The gap report makes this explicit, allowing the human to either add modules that close the gap or weaken the claim to match what is actually delivered.

This bidirectional structure is the discipline that prevents the system from drifting. It also enables what we call *post-verification*: the human can ask the system to summarize what the implementation actually delivers, in higher-level terms, and compare that summary to their original prose intent. The system cannot judge whether the summary matches the intent — that remains the human's role — but it can produce the summary mechanically and surface what was not delivered, giving the human a meaningful artifact to judge against.

### 4.5 Hierarchy: filesystem as architecture

The model proposes that the filesystem structure of an umbrella-organized codebase is semantically significant, not merely organizational. A directory is itself a module (often via a designated file such as `_.lm`) holding shared types, invariants, and obligations for everything beneath it. Files within a directory are *siblings*, sharing the parent's context and able to reference each other's exported claims without explicit imports. Cross-directory references require explicit `uses` blocks with integration tests that verify the inter-module contract.

This geometry produces an architecture visible by inspection: browsing the filesystem reveals the system's domain structure, with sibling proximity indicating tight coupling and tree distance indicating loose coupling. Moving a file becomes a semantic operation (with tooling support); refactoring a domain becomes moving a directory; integration tests live at every meaningful boundary because every meaningful boundary is a directory or `uses` block.

Recursive composition is natural: umbrellas can have umbrellas, with a *verifiability gradient* — claims become more formal as one descends the tree and more prose-like as one ascends. The top of the tree might assert qualitative properties that the compiler cannot directly verify; the immediate-children umbrellas decompose those into more specific claims that lower-level modules prove mechanically. Bidirectional refinement bridges the gradient: lower-level proofs are summarized upward and matched against higher-level claims, with gaps reported.

### 4.6 Phases: progressive formality

The model proposes a *phase* per module to reduce the ceremony cost during exploration. Three phases:

- **Sketch.** Minimal ceremony. Types inferred. `uses` blocks reduced to names. Examples optional. Verification is best-effort. Suitable for exploratory work and prototyping.
- **Draft.** The system has observed usage and proposes contracts (e.g., "I notice this function always returns non-negative; should that be a refinement type?"). Verification is partial. The user reviews proposals and either accepts or refines.
- **Settled.** Full contracts declared. All examples mandatory. All proof obligations discharged or explicitly marked as `gaps`. Changes to settled modules are breaking changes that propagate downstream as such.

Phases are not lifecycle stages — they are properties of modules that can move in either direction. A previously settled module can be moved back to draft for major rework; a sketch module can be promoted to settled when its design stabilizes. The phase system lets the language be permissive when exploration is needed and strict when stability is needed, without forcing the user to choose one mode globally.

### 4.7 Capability-tracked effects

Each module declares the effects it uses (e.g., `@net`, `@db`, `@clock`). These declarations are tracked through the type system and verified at module boundaries. A module that needs the network must declare so; a module without the declaration cannot call network functions, and the compiler enforces this statically.

For human–LLM collaboration this is particularly valuable: an LLM generating an implementation cannot accidentally introduce a side-effect that the umbrella did not authorize. If the umbrella says a module is pure, the implementation must be pure; if the umbrella allows network access, the implementation must declare and use it explicitly. Capability tracking prevents a whole class of "the LLM added something I didn't ask for" failure modes.

### 4.8 Multi-target compilation with capability profiles

The model is target-agnostic but capability-honest. A *target profile* declares which features a given runtime supports: actors, hot reload, supervision, distribution, durable workflows, dataflow primitives, soft real-time, hard real-time, foreign-library access. A module declares which features it needs. The compiler matches module needs against target profiles and refuses (or warns, or degrades) when there is a mismatch.

This allows multi-target deployment without pretending all targets are equivalent. An umbrella can declare that one sibling compiles to a BEAM-equivalent runtime (for actor-shaped, fault-tolerant work), another to native code (for performance), another to a Python runtime (for library access), with cross-runtime communication via a stable typed message protocol. The user reasons in umbrella terms; the compiler handles the heterogeneity.

### 4.9 Live runtime view

The runtime, when present, is instrumented at construct granularity: every named flow stage, actor mailbox, workflow step, and capability use is observable by default. The compiler's knowledge of structure (because flows, actors, and workflows are language constructs rather than library calls) means observability does not need to be retrofitted. The user inspects the running system in *umbrella terms* ("the `transfer` actor has three messages in queue, the `audit` flow is running at 1,200 events/sec"), not in target-runtime terms ("GenServer 247 has mailbox depth 3").

### 4.10 The role of the LLM

In the proposed architecture, the LLM operates at two boundaries: between prose and umbrella (distillation: prose paragraphs become umbrella claims), and between umbrella and siblings (generation: umbrella obligations are realized as implementation modules). At both boundaries the LLM proposes; the compiler verifies; the human reviews. The LLM is treated as a first-class author — its contributions are recorded with provenance — but never as a final authority. Verification is the gate.

This positions the LLM where it is genuinely strong (interpretation of natural language, structured code generation, pattern completion) and the human where they are genuinely strong (intent, judgment, "that is not what I meant"). The umbrella is the medium of their collaboration; the compiler is the discipline that keeps it honest.

---

## 5. Loom: One Realization

We describe Loom as a concrete realization of the proposed model. Loom commits to specific design choices that are not the only choices possible. We present them to demonstrate that the model can be realized cleanly, not to argue they are uniquely correct.

### 5.1 Surface syntax

Loom uses a term-based, period-terminated grammar in the lineage of Prolog and Erlang. The unit of structure is the *term*: an atom, a number, a string, or a name applied to arguments (e.g., `transfer(alice, bob, 30)`). Operators are sugar for binary terms. Sections are introduced by keywords (`knows`, `relates`, `shows`, `does`, `proves`, `uses`, `receives`, `keeps`); periods terminate top-level forms; commas join clauses within forms; indentation is for readability, not for parsing.

This grammar is homoiconic in the term-based sense: code is data (terms), and metaprogramming manipulates code as terms rather than as text. Loom rejects braces and explicit block delimiters, on the grounds that periods plus indentation provide sufficient structure. It rejects assignment operators (`:=`); state updates are expressed as values returned by handlers, with explicit `state with field: value` syntax for partial updates. Records are terms with named arguments (`point(x: 1, y: 2)`), not braced literals.

### 5.2 An example module

A small module illustrating the section structure:

```
module money/transfer.

uses
  std/effects(@ledger),
  std/time(now).

knows
  Money   :: nonneg int,
  Account :: atom,
  Ledger  :: map(Account, Money).

relates
  transfer(From, To, Amount) shifts Ledger
    when  From /= To,
          balance(From) >= Amount
    so    balance'(From) = balance(From) - Amount,
          balance'(To)   = balance(To)   + Amount.

shows
  ex normal:
    given  ledger(alice: 100, bob: 50),
    do     transfer(alice, bob, 30),
    yields ledger(alice: 70,  bob: 80).

  ex overdraft:
    given  ledger(alice: 10),
    do     transfer(alice, bob, 30),
    yields error(insufficient_funds).

does
  transfer(From, To, N) =
    adjust(From, -N) then adjust(To, +N).

  adjust(Acc, Delta) =
    ledger with Acc: balance(Acc) + Delta.

proves
  conservation:
    for-all L T,
      sum(L before T) = sum(L after T).
```

Each section uses the vocabulary that fits its purpose. `knows` reads like a glossary; `relates` reads like a contract with primed variables for after-state; `shows` reads like a script; `does` reads operationally; `proves` reads mathematically. The same parser builds the same tree underneath all of them.

### 5.3 Actors, workflows, and dataflow

Loom provides first-class constructs for the three patterns most relevant to systems work: actors (message-passing state holders), workflows (durable processes with saga-style compensation), and dataflow (typed streams composed into named stages). Each is its own kind of module with section-as-register vocabulary tuned to its purpose. Actors have `has`, `receives`, `keeps`, `on idle` sections. Workflows have `step`, `compensate`, `policy` sections. Flows have `subscribe`, `where`, `via`, `emit` operators composed via `<-` binding and `|>` piping.

The unifying observation is that all three are *typed channels feeding state machines* — actors process messages, workflows process step completions, flows process stream items. The runtime treats them as the same underlying shape, which is what makes hot reload, instrumentation, and bidirectional verification uniform across them.

### 5.4 The umbrella module

Directories in Loom are themselves modules. A `_.lm` file (or designated convention) holds the umbrella for everything beneath. The umbrella declares shared types, invariants, obligations on contained siblings, top-level flow topology, and bidirectional verification links:

```
module money @umbrella.

knows
  Money   :: nonneg int,
  Account :: atom,
  Ledger  :: map(Account, Money).

keeps
  no_overdrafts:
    for-all Acc, eventually balance(Acc) >= 0.
  conservation:
    for-all T : Transfer,
      sum(balances before T) = sum(balances after T).

contains
  transfer
    must show happy_path, overdraft_rejected, self_transfer_rejected.
  ledger
    must provide balance/1, adjust/2.
  audit
    must emit audit_log :: Stream<AuditEntry>.

summarizes from contains
  customer_funds_safe <=
    transfer.proves(no_negative_local),
    ledger.proves(monotonic_under_recovery).

gaps
  customer_funds_safe:
    -- fraud detection not represented.
    -- rate limiting not represented.
```

The `summarizes from contains` block maps child-level proved properties to umbrella-level claims; the `gaps` block enumerates claims that cannot be fully grounded. This is the bidirectional refinement made concrete.

### 5.5 The runtime substrate

Loom's reference runtime is BEAM (the Erlang/Elixir virtual machine). The choice is pragmatic: BEAM already provides production-grade actors, hot reload, supervision, distribution, and observability — exactly what the model assumes at the runtime layer. Generating Erlang source from Loom siblings and compiling via `erlc` lets us inherit the BEAM ecosystem (OTP, telemetry, distributed Erlang, Mnesia, Phoenix) without rebuilding it.

Crucially, BEAM is *invisible* to the Loom user. The Loom toolchain hides it: stack traces are translated back to Loom locations via source maps; runtime errors are mapped through a finite, reviewed taxonomy to Loom-level errors; live system inspection presents BEAM telemetry in Loom vocabulary. The user lives entirely within Loom semantics; the implementation happens to deliver them via BEAM.

Other targets — WASM components, .NET/Akka, Python/Ray, native Rust, real-time Rust — are supported via additional backends, each declaring its capability profile. A module that requires `hot_reload` will not compile to native Rust; the compiler refuses and explains why. This is the capability-honest multi-targeting from §4.8 made concrete.

### 5.6 What Loom commits to and what it leaves open

Loom is a specific set of design choices: term-based grammar, period termination, sections-as-registers, filesystem-as-architecture, no assignment operator, BEAM as reference runtime. Alternative realizations could differ on any of these without abandoning the underlying model. A realization with block-and-brace syntax, or with explicit assignment, or with a JVM-first runtime, or with a flat namespace instead of hierarchical filesystem, would still be a faithful realization of the three-layer model with bidirectional refinement, provided it preserves:

- The three-layer structure (prose, umbrella, siblings) as distinct, mutually-readable artifacts.
- Claims as the unifying notion in the umbrella, with stable IDs and provenance back to prose.
- Bidirectional verification with explicit gap reporting.
- Mechanical verification of sibling implementations against umbrella obligations.
- Capability tracking for effects.
- Some form of hierarchy or composition allowing umbrellas of umbrellas.

We expect alternative realizations to emerge, and we welcome them. The model is the contribution; Loom is one demonstration.

---

## 6. Prototype Strategy

Building a new programming language from scratch is a multi-year endeavor. Empirically validating the proposed model does not require it. We describe a prototype strategy that uses existing tooling to test the model's central claims, deferring bespoke language design until the model itself is validated.

### 6.1 The minimum viable stack

The prototype consists of five components, each implementable with existing tooling:

**1. Artifact format.** Structured Markdown with YAML-fenced claim blocks. Prose lives in plain Markdown sections. Claims live in `yaml` code blocks with conventions for sections (`knows`, `relates`, `shows`, etc.). Each claim has an explicit ID, prose-link, status, and content. The format is ugly compared to bespoke syntax but is parseable by any language, readable by humans, diffable in Git, and shippable in a weekend.

**2. Verifier.** A command-line tool (we propose Rust, for the type-safety and ecosystem) that parses the Markdown+YAML format, extracts claims, and verifies them. For the implementation language inside `does` blocks, we propose embedding an existing scripting language (Rhai or Starlark) rather than writing a new interpreter. Examples in `shows` blocks are evaluated by running the embedded interpreter against the given inputs and comparing outputs. Property tests in `proves` blocks use a standard property-based testing library. Refinement types, where needed, are discharged via Z3 (callable from Rust). The verifier emits a structured report: per-claim status, gaps, regressions.

**3. LLM integration.** Three LLM-mediated commands in the same CLI:
- `distill`: takes a prose document, prompts the model to extract umbrella claims, returns structured YAML for human review.
- `generate <sibling>`: takes the umbrella, prompts the model to fill in `does` blocks, runs the verifier, feeds errors back for retry.
- `summarize <umbrella>`: walks the tree bottom-up, asks the model to summarize what each module's proved properties imply at higher levels, produces the gap report.

All three are LLM API calls with structured input/output. No bespoke infrastructure is needed beyond an HTTP client.

**4. Editor.** A VSCode extension that renders claim blocks with inline verification status, exposes the three LLM commands as editor commands, shows hover information including claim IDs and provenance, and surfaces the umbrella structure in a webview panel. The extension calls the Rust CLI for verification and a chosen LLM provider for generation.

**5. Storage and collaboration (initial).** Git, with a discipline of one-claim-per-file or one-section-per-file to reduce merge conflict surface. Multi-user collaboration with claim-level conflict resolution is deferred to a later phase.

### 6.2 Sequencing

We propose six phases, each yielding a useful intermediate artifact:

**Phase 1 (≈ 1 month): Format and verifier core.** Define the Markdown+YAML format. Build the Rust CLI. Embed Rhai for `does` blocks. Implement example-checking and basic type-checking. Validate by hand-writing several modules and running them through the verifier.

**Phase 2 (≈ 2 months): LLM integration.** Implement `distill`, `generate`, `summarize`. Iterate on prompt design until the proposed claims/summaries are usefully accurate. Validate by running the full prose → umbrella → siblings → verification loop on small but real systems.

**Phase 3 (≈ 2 months): VSCode extension.** Inline verification, claim navigation, LLM commands, umbrella webview. Validate by attempting to do real work in the extension and tracking pain points.

**Phase 4 (≈ 3 months): Bidirectional refinement.** Implement `summarizes from contains` and the gap report. Validate by constructing systems with deliberate gaps and confirming the report identifies them; validate also that the LLM-generated summaries are conservative (preferring "I could not ground this" over false confirmation).

**Phase 5 (≈ 3 months): Provenance and collaboration substrate.** Stable claim IDs, prose-paragraph links, event-sourced change log. Initial multi-user collaboration via CRDT-style merging at the claim level.

**Phase 6 (open-ended): Runtime codegen.** Generate Erlang source from siblings; run on BEAM; close the loop from umbrella to running system. At this point the question of whether to invest in a bespoke language surface (Loom-syntax-proper) becomes empirically informed.

The total elapsed time to a working end-to-end prototype is approximately 11 months for a single developer. The first three phases yield a useful authoring system even without the runtime; the runtime is the last step, not the first.

### 6.3 What the prototype validates

The prototype is sufficient to evaluate four central empirical claims of the model:

1. **The umbrella is small.** Across multiple systems, the umbrella size grows much more slowly than the implementation size. (Measurable: claim count vs. lines of generated code over time.)
2. **Bidirectional refinement catches drift.** Mismatches between claimed and proved properties are correctly identified by the compiler. (Measurable: gap reports against ground-truth specifications.)
3. **LLM-generated siblings can satisfy verified umbrellas.** With current-generation models, the regeneration loop converges on verified implementations within a small number of iterations for non-trivial modules. (Measurable: iterations to passing verification, per module.)
4. **Human cognitive load is reduced.** Users supervising LLM-generated code via the umbrella report substantively lower load than supervising the same code without it. (Measurable: subjective task load surveys, error rates in human review.)

If these four claims hold empirically, the model is validated and bespoke language design becomes worth pursuing. If they do not hold, the model has been falsified at low cost.

### 6.4 What the prototype does not address

The prototype does not address: runtime performance characteristics that depend on a real target (BEAM, native, etc.); large-scale multi-user collaboration; non-textual artifacts (UI mockups, configuration files); systems requiring formal proof beyond what SMT can discharge; systems where the bottom layer is untyped foreign code (the verification boundary terminates at the foreign-call interface).

These are real limitations of the prototype, not of the model. Some can be addressed in later phases (runtime, collaboration); others reflect intrinsic boundaries of what mechanical verification can do.

---

## 7. Evaluation Framework

We propose an evaluation framework for empirical work on the model. Two kinds of evaluation are relevant: *artifact-level* (does the system produce correct, useful output?) and *workflow-level* (does the system change how humans work?).

### 7.1 Artifact-level evaluation

For artifact-level evaluation, we propose comparing systems built using the umbrella model to systems built using two baselines: (1) conventional LLM-assisted programming (Cursor, Copilot, or equivalent) without an intermediate verified artifact; (2) spec-driven LLM programming without bidirectional verification (specifications written first but not mechanically maintained relative to code).

Metrics:

- **Verification coverage**: fraction of system behavior covered by examples, refinement types, or proofs.
- **Specification-implementation drift**: rate at which the specification (or umbrella) and the implementation diverge over time, in systems with active development.
- **Gap accuracy**: precision and recall of the bidirectional gap report against ground-truth specifications.
- **Regeneration stability**: average extent of implementation change in response to small specification changes (the *small input, small output* property).
- **Defect density**: bugs per thousand lines of generated code, after verification has passed.

### 7.2 Workflow-level evaluation

For workflow-level evaluation, we propose user studies in which experienced developers attempt comparable tasks using the umbrella model versus the baselines.

Metrics:

- **Time to working system**: elapsed time from task description to verified implementation.
- **Cognitive load**: NASA-TLX or equivalent subjective load measures, administered during and after the task.
- **Code review fidelity**: in scenarios where developers review LLM-generated code, the rate at which they correctly identify intentional defects, comparing review-with-umbrella to review-without.
- **Intent preservation**: how well the final implementation matches the developer's stated initial intent, judged by independent reviewers.
- **Cross-session resumability**: when developers return to a system after a break, how quickly they regain working context, comparing umbrella-mediated to prose-only handoffs.

### 7.3 Limitations of evaluation

We acknowledge significant evaluation challenges. The proposed model is a workflow intervention as much as an artifact intervention, and workflow effects often emerge only at scale or over time. Short studies will underestimate the model's benefits (or surface costs); long studies are expensive and confound many variables. We propose that initial evaluations should focus on artifact-level metrics, where ground truth is more accessible, with workflow-level evaluations deferred until the artifact-level claims are validated.

We also acknowledge that the model's most distinctive feature — bidirectional refinement with gap reporting — is hardest to evaluate, because its value lies in what it prevents (silent drift, unrecognized gaps) rather than what it produces. We expect this to be the most difficult and most important locus of empirical work.

---

## 8. Discussion

### 8.1 What the model addresses

The proposed model directly addresses the three failure modes identified in §1:

- **Unstructured generation.** The umbrella provides an artifact that the LLM must satisfy and that the human can read. Generation cannot succeed without satisfying the umbrella; review cannot fail to engage with the umbrella because the umbrella is the focal artifact.
- **Specification drift.** Bidirectional verification mechanically maintains the relationship between specification (umbrella) and implementation (siblings). Drift surfaces immediately as failed verification or as gap reports.
- **Cognitive overload.** The umbrella is small and grows slowly. Code review happens at the umbrella level; sibling-level review is on demand. The human's cognitive surface stays bounded even as the codebase grows.

The model also enables capabilities that current architectures support poorly: durable cross-session memory (umbrella as durable artifact); provenance of intent (claim-to-prose links); honest accounting of what the system actually guarantees (gap reports); multi-target deployment with explicit capability honesty.

### 8.2 What the model does not address

The model does not address several real concerns:

- **Intent capture.** The model assumes the human can write prose that, distilled, captures their intent. Garbage prose still produces garbage umbrellas. The model improves the verification of intent-to-implementation, not the elicitation of intent itself.
- **LLM reliability.** The model relies on the LLM generating implementations that can pass verification within a small number of iterations. With current-generation models this is plausible but not guaranteed; with weaker models the regeneration loop may not converge. The model degrades gracefully — failed verification simply requires more iterations or human intervention — but it does not eliminate the need for capable models.
- **Truly novel design.** Bidirectional verification keeps the system honest about *what is claimed and what is proved*, but the originality of the claims is the human's responsibility. The model is a discipline, not a creativity engine.
- **External integration.** Systems that integrate with foreign code (existing libraries in other languages, external services, hardware) terminate the verification chain at the foreign boundary. The model accommodates this via explicit `relies-on` declarations at boundaries but cannot extend verification beyond them.

### 8.3 Generality of the model

We argue that the three-layer structure with bidirectional refinement is generalizable beyond Loom and beyond the specific design patterns we have proposed. The model could be realized:

- With different surface syntaxes (block-and-brace, S-expression, indentation-only, etc.).
- With different runtime targets as primary (BEAM, JVM, native, WASM).
- With different verification backends (SMT, model-checking, type-theoretic proof, property-based testing).
- With different collaboration substrates (Git, CRDT, blockchain-anchored, others).
- With different LLM integration patterns (interactive, batch, pair-with-model-as-author).

What remains invariant across realizations: prose as the human's reasoning surface, umbrella as the verified intermediate, siblings as verified implementation, bidirectional refinement as the discipline.

### 8.4 The role of Loom

Loom in this paper plays the role of an existence proof: a demonstration that the model can be realized concretely without internal contradiction. We have not claimed Loom is optimal. We have aimed to make Loom *coherent* — internally consistent, externally implementable, aesthetically considered — to give the model a face that potential implementers can react to. Reactions in either direction (adoption, critique, alternative realization) are productive. The worst outcome would be that the model remains so abstract that no one can engage with it; Loom is the engagement surface.

---

## 9. Limitations

We have noted several limitations in passing. We collect and elaborate them here.

### 9.1 Trust in LLM-generated verification artifacts

The model relies on the LLM not only to generate implementations but also to propose claims, summaries, and gap reports. If the LLM is sloppy, the verification artifacts themselves are unreliable. The mitigation is that humans review LLM proposals before acceptance and that the verifier mechanically checks claims rather than trusting them. But mechanical checking has its own limits: an LLM-generated claim like "this function is pure" can be mechanically verified, but a claim like "this function correctly implements the user's intent" cannot. The human remains the final arbiter for intent-matching, and the volume of LLM proposals may exceed the human's review capacity.

### 9.2 Computational and storage cost

Bidirectional verification across a hierarchy is computationally expensive. Each claim is a small theorem; each upward summarization is a proof obligation; each regeneration cycle involves model calls, verification runs, and propagation analysis. For large systems this cost is non-trivial. The cost is partially mitigated by incremental verification (only re-check what changed) and by phase-aware verification (sketch-phase modules verify lightly, settled-phase modules verify deeply), but it does not disappear. Event-sourced provenance adds storage overhead — the full history of every claim is durable — which compounds over time.

### 9.3 Corpus and tooling for a new language

A realization that introduces a new surface syntax (like Loom-proper) suffers from the standard new-language adoption problem: there is no training corpus, no IDE plugin ecosystem, no Stack Overflow. The prototype strategy in §6 mitigates this by deferring bespoke syntax — early evaluation uses structured Markdown, which inherits the existing Markdown corpus and tooling. The corpus problem only bites when a realization moves to bespoke syntax, and at that point the realization will need to seed its own corpus (often by being the canonical output format of an LLM-based authoring tool).

### 9.4 Conceptual demands on users

The model asks users to think in claims, registers, bidirectional refinement, capability profiles, and verifiability gradients. This is more conceptual machinery than typical programming, and not all users will find the trade worth it. Users who already enjoy formal methods (refinement types, TLA+, dependent types) will find this congenial; users who prefer minimal-ceremony approaches may not. The model is not for everyone; we do not claim otherwise.

### 9.5 What bidirectional verification does not catch

Bidirectional verification catches drift between claimed and proved properties. It does not catch:

- Errors in the prose that produced the claims (garbage in, structured garbage out).
- Errors in the claim formulation that pass verification but do not match intent.
- Behaviors emerging from the interaction of multiple correctly-implemented modules (composition errors below the umbrella's level of abstraction).
- Runtime behaviors that are not amenable to static or example-based verification (race conditions in rare timings, behaviors depending on external state).

These remain real risks. The model raises the floor of what is reliably caught; it does not eliminate the need for testing, monitoring, and human judgment.

### 9.6 The dependency on capable verification backends

The mechanical-verification story relies on SMT, type-theoretic proof, and property fuzzing being adequate to the verification needs of typical systems. For many domains (data transformation, business logic, protocol implementation) this is the case. For domains that require deeper proof (cryptographic correctness, real-time guarantees, hardware verification) it is not. The model accommodates richer verification via plugin backends, but it does not make hard verification easy.

### 9.7 Verification under LLM authorship of specifications

The model treats the umbrella as the trusted artifact and verifies the implementation against it. When the umbrella is itself LLM-authored — as the §6 prototype contemplates via the `distill` operation — the verification chain bottoms out in human review of claims, and the cheating dynamics shift from the implementation to the authorship boundary. Specifically: the LLM may propose claims it can trivially satisfy (`for-all x, true`, or properties with preconditions narrowed to vacuity); it may declare hard cases as gaps rather than prove them, moving the system's effective level of guarantee down to where it can be carried; it may select `shows` examples that exercise only easy paths, leaving boundary cases unrepresented; it may declare effects or capabilities that move work into layers where verification is weaker. The bidirectional gap report makes some of these visible — an umbrella whose gaps grow as the implementation matures is recognizable as a regression in spec strength — but the gap report itself depends on the human catching what the LLM has elided. 3DGen (§2.8) addresses part of this attack surface via an external oracle that provides a verification target independent of the LLM-authored spec; the model proposed here has no analogous oracle in the general case. Mechanisms for catching weak specs — mutation testing on claims, cross-register coverage rules, statistical engagement-with-domain measures — are an open research direction we treat in a separate paper.

---

## 10. Future Work

We identify several lines of future work.

### 10.1 Empirical evaluation

The most important next step is empirical evaluation along the lines proposed in §7. Implementing the prototype described in §6 and running it against real systems is necessary to validate (or falsify) the central claims of the model. We hope that this paper will encourage such work.

### 10.2 Alternative realizations

We have offered Loom as one realization but expect that alternatives will be valuable. Realizations targeting JVM, WASM, or Python ecosystems would test the model's generality. Realizations with different surface syntaxes (block-based, S-expression, visual/structural) would surface which design patterns are essential and which are aesthetic. Realizations integrated with existing IDEs or notebooks (rather than bespoke editors) would test the model's compatibility with established workflows.

### 10.3 Collaboration at scale

The model's claim-level collaboration substrate is sketched but not deeply developed in this paper. Real multi-user editing with concurrent modification of claims, conflict resolution, branching, and merging requires substantial design work, possibly drawing on Pijul's patch theory and CRDT research. We consider this a critical area for follow-up.

### 10.4 Integration with formal methods

The proposed verification machinery uses SMT and property-based fuzzing as defaults but does not deeply integrate with the established formal-methods stack (Lean, Coq, Isabelle, F*, Dafny). A realization that allows the heaviest proof obligations to be discharged via these tools, while keeping lighter obligations within the local verifier, would extend the model's reach to domains requiring stronger guarantees.

### 10.5 Verifying LLM-authored specifications

The bidirectional refinement story in §4.4 assumes the umbrella's claims are sincere — that the human, possibly assisted by an LLM, has written claims that reflect the desired system behavior. Empirical work on the §6 prototype is likely to find that claims, when LLM-proposed, exhibit characteristic weakening dynamics: vacuous quantifications, gap-as-escape, narrow examples (§9.7). Defending against these is a distinct research direction with its own threat model, defenses, and evaluation methodology. The piece most likely to be novel — mutation testing applied to specifications under LLM authorship — does not appear in the verification literature to our knowledge. We sketch the direction here and develop it in a separate paper.

### 10.6 LLM training for umbrella manipulation

LLMs trained generally are competent at producing umbrellas in the proposed format but not optimized for it. Fine-tuning or instruction-tuning on umbrella-shaped artifacts would likely improve the distill, generate, and summarize operations substantially. The training corpus could be bootstrapped from the model's own output, curated by human reviewers.

### 10.7 Application beyond software

The three-layer structure (prose, umbrella, verified output) is potentially applicable beyond software systems. Domains with analogous needs — legal contract drafting, regulatory compliance, scientific protocol specification, engineering design documentation — share the underlying problem (human intent, formal intermediate, verified artifact) and may benefit from analogous tooling. We do not pursue this direction here but flag it as an open question.

---

## 11. Conclusion

We have proposed an architectural model for human–LLM collaboration on software systems, organized around a verifiable intermediate artifact between human prose and machine-generated code. The model is three-layered: prose as the human's durable memory; an umbrella of formal claims as the verified shared artifact; siblings as LLM-authored implementation modules satisfying the umbrella's obligations. The verification discipline is bidirectional: obligations flow downward, properties flow upward, and the compiler reports gaps where claimed and proved properties diverge.

We have described one realization (Loom) sufficient to demonstrate the model concretely, and a prototype strategy using existing tooling that allows incremental evaluation without first committing to bespoke language design. We have discussed the model's strengths, limitations, and the empirical questions it raises.

The contribution is best understood as an architectural framing — a way of asking what artifacts and disciplines a serious human–LLM collaboration on software should be organized around. We have proposed answers; we expect future work to test, refute, and refine them. Loom is one demonstration; other realizations are invited. The model is the contribution; the realizations are how the contribution becomes useful.

We close with what we believe is the deepest reason this question is worth pursuing now. The current moment in software development is characterized by enormous capability mismatch between human and machine. Machines generate code faster than humans can review it; humans express intent in forms machines cannot reliably verify. The gap is bridged today by trust, which scales poorly and fails silently. The proposed model replaces trust with verifiable artifact, and replaces ad-hoc collaboration with a discipline whose rules are mechanically enforced. This is not the only possible response to the current moment, but we believe it is a serious one — and a model that subsequent work, building or critiquing, can engage with productively.

---

## References

Abrial, J.-R. (1996). *The B-Book: Assigning Programs to Meanings*. Cambridge University Press.

Abrial, J.-R. (2010). *Modeling in Event-B: System and Software Engineering*. Cambridge University Press.

Armstrong, J. (2003). *Making reliable distributed systems in the presence of software errors*. PhD thesis, Royal Institute of Technology, Stockholm.

Bertot, Y., & Castéran, P. (2004). *Interactive Theorem Proving and Program Development: Coq'Art: The Calculus of Inductive Constructions*. Springer.

Brady, E. (2013). Idris, a general-purpose dependently typed programming language: Design and implementation. *Journal of Functional Programming*, 23(5), 552–593.

Chen, M., Tworek, J., Jun, H., et al. (2021). Evaluating large language models trained on code. *arXiv:2107.03374*.

Colmerauer, A. (1990). An introduction to Prolog III. *Communications of the ACM*, 33(7), 69–90.

de Moura, L., Kong, S., Avigad, J., van Doorn, F., & von Raumer, J. (2015). The Lean theorem prover. In *International Conference on Automated Deduction* (pp. 378–388). Springer.

Fakhoury, S., et al. (2024). 3DGen: AI-Assisted Generation of Provably Correct Binary Format Parsers. *arXiv:2404.10362*.

Hewitt, C., Bishop, P., & Steiger, R. (1973). A universal modular ACTOR formalism for artificial intelligence. In *Proceedings of the 3rd International Joint Conference on Artificial Intelligence*.

Hoare, C. A. R. (1969). An axiomatic basis for computer programming. *Communications of the ACM*, 12(10), 576–580.

Jackson, D. (2002). Alloy: a lightweight object modelling notation. *ACM Transactions on Software Engineering and Methodology*, 11(2), 256–290.

Knuth, D. E. (1984). Literate programming. *The Computer Journal*, 27(2), 97–111.

Lamport, L. (1994). The temporal logic of actions. *ACM Transactions on Programming Languages and Systems*, 16(3), 872–923.

Meyer, B. (1992). Applying "design by contract". *Computer*, 25(10), 40–51.

Norell, U. (2007). *Towards a practical programming language based on dependent type theory*. PhD thesis, Chalmers University of Technology.

Pierce, B. C. (2002). *Types and Programming Languages*. MIT Press.

Plotkin, G. D. (1981). *A structural approach to operational semantics*. Technical report DAIMI FN-19, Aarhus University.

Rondon, P. M., Kawaguchi, M., & Jhala, R. (2008). Liquid types. In *Proceedings of PLDI 2008*, 159–169.

Shapiro, M., Preguiça, N., Baquero, C., & Zawirski, M. (2011). Conflict-free replicated data types. In *Symposium on Self-Stabilizing Systems* (pp. 386–400). Springer.

Somogyi, Z., Henderson, F., & Conway, T. (1996). The execution algorithm of Mercury, an efficient purely declarative logic programming language. *The Journal of Logic Programming*, 29(1–3), 17–64.

Swamy, N., Hriţcu, C., Keller, C., et al. (2016). Dependent types and multi-monadic effects in F*. In *Proceedings of POPL 2016*, 256–270.

Vazou, N., Seidel, E. L., Jhala, R., Vytiniotis, D., & Peyton Jones, S. (2014). Refinement types for Haskell. In *Proceedings of ICFP 2014*, 269–282.

Zhou, N.-F., Kjellerstrand, H., & Fruhman, J. (2015). *Constraint Solving and Planning with Picat*. Springer.

---

*Draft. Comments and corrections welcome.*

*This paper proposes a model. It does not report on a built system at scale. Readers should calibrate accordingly.*
