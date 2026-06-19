# Loom v0 — Plan

> **Status:** seed document
> **Purpose:** seed the repository with a complete picture of what is to be built, what is optional, what is deferred, and what decisions remain open. Intended to be chopped into epics, milestones, ADRs, and gaps. Not a specification; specifications live in `docs/`.
> **Not a roadmap:** there are no time estimates here. Sequencing is discussed in §6 but is intent rather than commitment.

---

## 0. How to read this document

This plan describes a v0 realization of the Loom architecture proposed in [`docs/research/verifiable-umbrella-paper-v2.md`](docs/research/verifiable-umbrella-paper-v2.md). The companion paper [`docs/research/spec-quality-under-llm-authorship.md`](docs/research/spec-quality-under-llm-authorship.md) develops the spec quality reporter (`specq`) that supports the architecture against weak LLM-authored specifications. The essay [`docs/research/process-gates-and-value-gates.md`](docs/research/process-gates-and-value-gates.md) offers useful context on the gate-design space Loom operates in, but Loom's argument does not depend on it.

Every section that involves a choice presents the choice in for/against form so that downstream ADRs can be written against a record of considered alternatives, not against silence.

The conventions for this document:

- **Architectural commitments** (§2) are things we are committing to as the project's identity. Changing them changes what Loom is.
- **Components** (§4) are units of work. Each component has a goal, a default choice (the one most likely to survive ADR), arguments for and against, alternatives, and the decision that needs to be made.
- **Out of scope** (§5) is content. These items will not be in v0 and should not be added without re-opening this plan.
- **Open decisions** (§9) is the list of ADRs to be written. Each cross-references the §4 component it belongs to.

This document does not prescribe the order of work. The intent is that each component becomes an epic, and the open-decisions list seeds the ADR backlog.

---

## 1. Vision

### 1.1 What Loom is

Loom is one realization of the Verifiable Umbrella architecture: a three-layer model for software construction in which (a) human prose captures intent in durable form, (b) an *umbrella* of structured formal claims sits as the verified intermediate artifact between prose and code, and (c) *siblings* of LLM-authored implementation modules satisfy the umbrella's obligations and are verified against them. The umbrella is small enough for a human to read fully; the implementation is detailed enough to be machine-verified against the umbrella's claims.

The discipline is *bidirectional*: obligations flow downward from prose through umbrella to implementation; properties flow upward from what the implementation proves through what the umbrella claims to what the prose asserts. Gaps between claimed and proved properties are reified as a *gap report*, which is the load-bearing visible artifact of the discipline.

### 1.2 What Loom is not

Loom is not a new programming language with a complete standard library. It is a language for *writing umbrellas and verifying implementations against them*, with a code generator that emits a target language (initially Python) for execution. The target language is where production code lives; Loom is where the claims and the relationship to those claims live.

Loom v0 is not:

- A complete formal-methods stack. It compiles to Dafny (or F*; see §4.3) and inherits the verifier's semantics.
- A multi-user collaboration platform. v0 is single-user, single-machine.
- A runtime substrate. v0 generates plain target-language modules without a supervised actor runtime.
- A production-grade tool. v0 is a research prototype intended to validate the architecture, not to be deployed.

### 1.3 Why now

Two converging conditions make the work timely. First, large language models are now competent at producing structured formal artifacts when given clear schemas. Second, the verification community's tools (Dafny, F*, Liquid Haskell, Lean) have matured to the point where they can be invoked as backends without the project committing to inventing its own verification semantics. The combination makes a working prototype tractable in a way it was not five years ago.

Loom is worth attempting even given the known cheating dynamics of LLM-mediated systems because its gates operate over the *value* of artifacts (does this implementation satisfy this claim?) rather than over the *process* of producing them (was this test written before this implementation?). Value gates are robust against process-faking in a way process gates are not: the LLM cannot produce an implementation that "looks like" it satisfies a claim without actually satisfying it under the verifier's check. The structural argument is developed further in the process-gates and value-gates essay (see §10).

---

## 2. Architectural commitments

The following are not open for negotiation in v0. Changing them changes what Loom is.

### 2.1 The umbrella as the verified intermediate artifact

The umbrella is the focal artifact. Everything in v0 serves the umbrella's role: it is what the human reads, what the LLM writes, what the verifier checks, what the codegen targets. The umbrella has five registers — `knows`, `relates`, `shows`, `does`, `proves` — described in [`docs/reference/claims-reference.md`](docs/reference/claims-reference.md). The registers are not negotiable in v0; cross-register coverage is part of the architecture's discipline.

### 2.2 Three layers

Prose, umbrella, implementation. The umbrella is between prose and implementation, not aligned with either. Tools that conflate any two of these (prose ↔ umbrella; umbrella ↔ implementation) are not Loom.

### 2.3 Bidirectional refinement

The verifier produces a *gap report*: a comparison of what the umbrella claims to what the verifier has proved. The gap report is part of the output, not an internal artifact. Implementations that pass verification but leave the umbrella's claims partially unproved have *gaps*, and gaps are first-class. [`docs/reference/bidirectional-refinement.md`](docs/reference/bidirectional-refinement.md) is the canonical reference.

### 2.4 The cheating attractor at the claim-authorship layer

Loom inherits the threat model from the companion paper: when the LLM authors umbrellas, claim weakening, gap-as-escape, example narrowing, and definitional erosion are the attack surface. v0 includes the spec quality reporter (`specq`) as part of the standard pipeline, not as an optional add-on. The defense is not optional because the threat is structural.

### 2.5 The substrate is git

v0 stores all artifacts as files in a git repository, with provenance carried in commit messages and trailers. There is no event log, no hash chain, no monotonic counter. The rationale is concrete: Loom's artifacts — umbrellas, implementations, gap reports, quality reports — are file-shaped; LLMs author files well; git is the standard substrate for versioned files with provenance. Inventing additional verification infrastructure on top of git would add mechanism without addressing the actual risk, which is at the claim-authorship layer (§2.4), not at the substrate layer.

### 2.6 Composition with existing verifiers

v0 does not invent verification semantics. The compiler emits artifacts in a target verifier's language (Dafny or F*; see §4.3) and reads the verifier's results. This is a deliberate choice in favor of leveraging mature tooling over building from scratch. The cost is coupling to the chosen verifier's semantics; the benefit is reaching a working prototype without inventing refinement-type theory.

---

## 3. Repository structure

The intended directory tree is presented in [`README.md`](README.md). The high-level layout:

- `crates/` — the Rust workspace containing the compiler, checker, verifier orchestrator, LLM operations, and specq. Each component is its own crate.
- `tree-sitter-loom/` — the tree-sitter grammar, used both for editor highlighting and (potentially) as the canonical parser.
- `examples/` — `.lm` source files demonstrating the language. The examples are part of the project's correctness story; they are continuously verified by CI.
- `docs/` — language reference, claims reference, verification internals, bidirectional refinement, LLM operations, spec quality, ADRs, and research.
- `docs/research/` — the four background documents (umbrella paper, spec quality paper, two essays).
- `docs/adr/` — architecture decision records.
- `tools/` — bootstrap scripts for development environments.

The rationale for the Cargo-workspace layout is that the components have clear boundaries (parsing, checking, compilation, verification orchestration, LLM operations, specq) and the workspace lets them be developed and tested independently while sharing dependencies and binary distribution. The alternative — one monolithic crate — was rejected because it makes the dependency graph between components implicit, and one of the project's principles is that the components' boundaries are clear.

---

## 4. Components

Each subsection is a candidate epic.

### 4.1 Surface language and parser (`crates/loom-syntax`)

**Goal.** Parse `.lm` files into a typed AST. Produce structured errors with span information. Produce a reasonably readable concrete-syntax form for re-emission (used by formatters and by LLM operations that need to rewrite parts of an umbrella in place).

**Default choice (recommended).** Hand-written parser in Rust using the `chumsky` parser-combinator library. Tree-sitter grammar maintained in parallel for editor support.

**For:**
- `chumsky` gives ergonomic combinator syntax with good error recovery. Mature, used by Tao and Erg.
- Hand-written parser stays close to the AST, gives total control over error messages, no codegen step in the build.
- Tree-sitter in parallel gives editor highlighting for free without committing to tree-sitter as the canonical parser.

**Against:**
- Two parsers means two places to update when the grammar changes. Risk of drift between the chumsky parser and the tree-sitter grammar.
- `chumsky` is well-regarded but has fewer production users than tree-sitter or pest.
- Tree-sitter as the canonical parser would mean one source of truth; but tree-sitter's error recovery is not as ergonomic for static analysis.

**Alternatives considered:**
- *Tree-sitter as canonical parser.* Single source of truth, less code, but tree-sitter's parse trees are less convenient for downstream analysis and error reporting is shaped by tree-sitter's recovery rather than the project's needs.
- *lalrpop.* Generated LR parser. Mature but error messages are harder to make excellent.
- *pest.* PEG-based, simple, but less expressive than chumsky for context-sensitive grammars.
- *Hand-rolled recursive descent.* No external dependency, full control. Significantly more code to write and maintain.

**Decision needed:** ADR-0005 — parser approach (chumsky vs tree-sitter-canonical vs lalrpop vs pest vs hand-rolled).

**Scope notes:**
- The full grammar is sketched in [`docs/reference/language-reference.md`](docs/reference/language-reference.md). The grammar may change as v0 evolves; the parser should be structured to accommodate changes without ground-up rewrites.
- Source positions must be preserved through parsing for diagnostic quality.

### 4.2 Static checker and cross-register coverage (`crates/loom-check`)

**Goal.** After parsing, check that the umbrella is internally well-formed: types referenced in `relates` are defined in `knows`; examples in `shows` typecheck against operations in `relates`; properties in `proves` reference operations from `relates`; cross-register coverage rules (every type used, every operation has at least one example and at least one property) are satisfied.

Anti-pattern detection (predicates that only mention inputs; refinement predicates that match every value; etc.) belongs in this crate, behind feature flags.

**Default choice.** Pure Rust check pass operating on the AST from `loom-syntax`. Produces structured diagnostics with severity levels (error, warning, lint).

**For:**
- Catches a large class of weak specs before invoking the verifier (which is slow), per the §5 defense taxonomy of the spec quality paper.
- Anti-pattern detection here is cheaper than mutation testing in `specq`; catches the lazy cheats.
- Diagnostics layered consistently with the parser's errors.

**Against:**
- Coverage rules can produce false positives in edge cases (e.g., a `knows`-only utility type used only by other types). Need an escape hatch (intentional-suppression syntax).
- Lint-level rules can be noisy if not carefully tuned.

**Alternatives considered:**
- *Skip the check pass; let verification catch all errors.* Rejected because verification is expensive and gives diagnostics in terms of the verifier's semantics, not Loom's.
- *Do checks as part of verification.* Conceptually possible but mixes layers; harder to evolve checking independently of verifier choice.

**Decision needed:** ADR-0006 — set of lint rules in v0 and their default severities.

**Scope notes:**
- Specific anti-patterns to implement are listed in [`docs/reference/spec-quality.md`](docs/reference/spec-quality.md).
- Suppression mechanism (`#[allow(unused_type)]`-style annotations on declarations) is part of v0.

### 4.3 Verification backend (`crates/loom-compile-dafny` initially)

**Goal.** Translate an umbrella's claims (`relates`, `proves`, `shows`) into the verifier's specification language; invoke the verifier; parse results; report claim-by-claim verification status with diagnostics where verification fails.

**Default choice (strong recommendation, but ADR-significant).** Dafny.

**For Dafny:**
- Refinement types, pre/postconditions, ghost code, automatic SMT discharge via Z3.
- Excellent tooling: VS Code extension, documentation, error messages.
- Active development by Amazon Web Services with stable releases.
- Easier learning curve than F*; broader user base.
- Microsoft Research lineage (Boogie, Z3) means well-understood semantics.

**Against Dafny:**
- Limits the expressiveness of `proves` to what Dafny's specification language supports. Higher-order claims, complex ghost state, or domain-specific reasoning may not translate cleanly.
- Couples Loom's semantics to Dafny's. Future divergence is a refactor.
- Method-centric design — Dafny is built around methods with pre/post; mapping Loom's register structure to Dafny's structure has some friction.

**For F\*:**
- Refinement types with effects; closer in spirit to Loom's architectural ambition (capability tracking, multiple targets).
- Strong dependent-type system; can express more.
- Research-active community; can engage with the verification literature directly.

**Against F\*:**
- Steeper learning curve. F* error messages are notorious.
- Smaller user community; less stable tooling than Dafny.
- Build setup is more involved (OCaml toolchain, several dependencies).

**For direct-to-Z3 (no intermediate verifier):**
- No upstream dependency on a research verifier. Full control over refinement semantics.
- Smaller dependency surface for end users.

**Against direct-to-Z3:**
- Inventing refinement-type semantics from scratch. Substantial original work.
- Re-implements what Dafny and F* already do well.
- Pushes v0 from "research prototype demonstrating an architecture" toward "research prototype inventing verification theory," which is a different project.

**Alternatives considered:**
- *Lean 4* — powerful, growing community, but very different paradigm (interactive proof rather than automated discharge).
- *Liquid Haskell* — refinement types in a real language but tied to Haskell semantically and runtime-wise.
- *Why3* — meta-verifier that targets multiple backends; potentially interesting as a long-term target but adds an indirection.
- *Multiple backends in v0.* Rejected: spreads effort, doubles maintenance burden, when the goal is to demonstrate the architecture works at all.

**Decision needed:** ADR-0002 (already drafted) — verifier choice. The drafted ADR recommends Dafny; the recommendation is appealable.

**Scope notes:**
- The Loom AST→Dafny translation is the core of [`docs/reference/verification-internals.md`](docs/reference/verification-internals.md).
- The verifier interface should be abstracted behind a trait (`trait Verifier`) so that swapping backends is a code change, not an architectural change.
- The translation is total (every well-formed umbrella translates) but the resulting Dafny may be unverifiable (the verifier may time out or fail to prove correct claims). These are different conditions.

### 4.4 Target language compilation (`crates/loom-compile-python` initially)

**Goal.** Translate an umbrella's `does` register into a runnable target-language module that satisfies the operations declared in `relates`. Generate a test harness that runs the `shows` examples against the implementation.

**Default choice.** Python as the v0 target. Single target only.

**For Python:**
- Large LLM training corpus; LLM-generated Python is more reliable than LLM-generated less-common languages.
- Easy to read for humans inspecting generated code.
- Dynamic typing means the codegen does not need to invent type machinery; the umbrella's types are the source of truth.
- Excellent ecosystem for the kinds of examples v0 will use (algorithms, data structures, small business logic).

**Against Python:**
- Dynamic typing means runtime errors can occur that would have been caught statically in a typed target.
- Performance is not Loom's concern in v0, but the generated code will not be production-grade in performance-sensitive cases.
- Some Loom claims (capability profiles, effect tracking) have no natural Python translation in v0; they degrade to runtime assertions or are not enforced at all.

**For TypeScript as alternative:**
- Static typing; types from `knows` translate to TypeScript types directly.
- Modern tooling, good IDE support, growing LLM competence.

**Against TypeScript:**
- More complex build pipeline (tsc, dependencies).
- LLM TypeScript is generally slightly less reliable than LLM Python in current models.

**For Rust as alternative:**
- Coherence with the compiler's implementation language.
- Strong type system; ownership and capability tracking could map onto Loom's effect annotations.

**Against Rust:**
- LLM Rust is least reliable of the three.
- High activation energy for users to read generated code.
- Borrow checker conflicts with naïve generated code.

**Alternatives considered:**
- *Multi-target in v0.* Rejected: doubles or triples the codegen work for unclear benefit; demonstrating the architecture works with one target is the v0 goal.
- *No target language; just emit the verified spec.* Rejected: misses half the architecture; without code generation, the umbrella isn't an intermediate artifact, it's a destination.

**Decision needed:** ADR-0003 (already drafted) — target language choice.

**Scope notes:**
- Codegen mappings (Loom types to Python types; `does` blocks to Python functions; `shows` examples to pytest tests) are documented in [`docs/reference/verification-internals.md`](docs/reference/verification-internals.md) §3 (target codegen, distinct from verification codegen in §2).
- The generated code is intended to be human-readable. Diff-friendliness matters because git is the substrate.

### 4.5 Verifier orchestration and gap reporter (`crates/loom-verify`)

**Goal.** Coordinate the pipeline: check → compile-to-Dafny → invoke Dafny → parse results → compare claimed vs proved → emit gap report. The gap report is the load-bearing visible artifact of the bidirectional refinement discipline.

**Default choice.** Pure Rust orchestrator that invokes Dafny as a subprocess. Gap report emitted both as structured JSON (for tooling) and as Markdown (for humans). Markdown is the canonical form.

**For:**
- Subprocess invocation is simple and decouples Loom's release cycle from Dafny's.
- JSON-and-Markdown dual output covers both machine and human consumption.

**Against:**
- Subprocess invocation is slower than embedded library invocation. Negligible at v0 scale.
- JSON schema needs to be designed and versioned, which is its own small project.

**Alternatives considered:**
- *Embed Dafny as a library.* Dafny is in C#/.NET; Rust↔.NET interop is possible but adds substantial complexity. Not worth it for v0.
- *Use only Markdown output, no JSON.* Tempting for simplicity but precludes machine consumption (CI integration, future tooling). Keep JSON.

**Decision needed:** ADR-0007 — gap report schema and stability commitments.

**Scope notes:**
- Gap report design is the topic of [`docs/reference/bidirectional-refinement.md`](docs/reference/bidirectional-refinement.md) §3.
- The gap report distinguishes (a) properties claimed but not proved (timeout, verifier limitation, genuine gap), (b) properties claimed and proved (the verified claims), (c) properties not claimed but implied by the implementation's verified behavior (the *bidirectional* part — what the implementation establishes that the umbrella does not credit).
- Category (c) requires running mutation testing on claims; details in §4.7.

### 4.6 LLM operations (`crates/loom-llm`)

**Goal.** Provide CLI commands for the three named operations from the architecture paper: `distill` (prose → umbrella), `generate` (umbrella → sibling), `summarize` (sibling claims → parent umbrella's `summarizes` register, when present). Each operation invokes the LLM with a structured prompt and incorporates the result into the umbrella or implementation.

**Default choice.** Anthropic Claude API as the LLM backend, behind a `trait LLMProvider` so other providers can be added. Prompts stored as Markdown files in `crates/loom-llm/prompts/`. Operations are idempotent at the prompt level (same input + same prompt = same prompt to the LLM; the LLM's response is non-deterministic, but the *invocation* is reproducible).

**For Claude:**
- Strongest at structured-format outputs in current models.
- Good at long-context reasoning (relevant for distilling longer prose).
- Anthropic's stated commitments to safety align with the project's threat-model emphasis on adversarial dynamics.

**Against Claude:**
- Single-vendor dependency. Mitigated by the `trait LLMProvider` abstraction.

**For openness to multiple providers:**
- Researchers may want to experiment with different models.
- Local-model inference (Ollama, llama.cpp) is increasingly viable for some operations.

**Against multi-provider in v0:**
- Each provider's API has its own quirks; supporting many doubles maintenance.

**Alternatives considered:**
- *Use a wrapper library (LangChain, LlamaIndex).* Rejected: adds dependency surface for marginal benefit. Direct API calls are simpler and the operations Loom needs are not LLM-orchestration-heavy.
- *Skip LLM operations entirely in v0; let humans write umbrellas.* Tempting for scope reduction but removes the architecture's central thesis demonstration. Keep LLM operations.

**Decision needed:** ADR-0008 — LLM provider abstraction and v0 default.

**Scope notes:**
- The full prompt design is in [`docs/reference/llm-operations.md`](docs/reference/llm-operations.md).
- Each operation should produce a *diff* against the existing artifact, not a wholesale rewrite. The diff is human-reviewable before being applied.
- Prompts include the relevant parts of the umbrella's schema (the five registers) as structured context, not just as natural language.

### 4.7 Spec quality reporter (`crates/specq`)

**Goal.** Implement the techniques from the companion paper: cross-register coverage measurement, domain engagement (precondition saturation, example diversity), gap-discipline metrics, anti-pattern detection (with `loom-check`), and mutation testing on claims.

**Default choice (significant scope decision).** `specq` is a crate within the loom workspace, not a separate repository. It is wired into the standard `loom verify` pipeline and produces a quality report alongside the verification result.

**For `specq` inside loom workspace:**
- Tight integration. Shared AST types, shared verifier abstraction.
- The companion paper's threat model is part of Loom's architectural commitment (§2.4); `specq` is not optional.
- One repository, one release cycle, one test suite.

**Against `specq` inside loom workspace:**
- `specq`'s techniques generalize beyond Loom — they could wrap F*, Dafny, Liquid Haskell, or future verifiers. Inside loom, the generality is harder to expose.
- Splitting later is harder than splitting now.

**Alternatives considered:**
- *Separate repository, generic over verifier.* Stronger as a research artifact (citable separately, applicable beyond Loom). Weaker as an integrated experience.
- *Crate within workspace, separately publishable.* Compromise: lives in the workspace for v0 but can be split out if/when generic-over-verifier work begins.

**Decision needed:** ADR-0009 — specq packaging and repository boundary.

**Scope notes:**
- v0 implements the §5 defenses from the companion paper (grammar bans, cross-register coverage, domain engagement, gap discipline, anti-patterns) and §6 mutation testing on claims, with a starter set of operators.
- The full mutation-operator catalog is documented in [`docs/reference/spec-quality.md`](docs/reference/spec-quality.md) §4.
- The mutation engine assumes the verifier is fast enough for repeated invocations. If verification is slow, caching is essential.

### 4.8 CLI binary (`crates/loom-cli`)

**Goal.** A single `loom` binary exposing all user-facing operations: `loom check`, `loom build`, `loom verify`, `loom distill`, `loom generate`, `loom summarize`, `loom specq`, `loom fmt`.

**Default choice.** `clap` for argument parsing, structured subcommand layout, consistent flag naming.

**For:**
- Single binary is simpler to install and document than multiple binaries.
- `clap` is mature, well-documented, ubiquitous in Rust CLIs.

**Against:**
- Single binary couples release cycles of CLI to all underlying crates. Mitigated by the workspace structure.

**Alternatives considered:**
- *Separate binaries per operation (`loom-verify`, `loom-distill`).* Rejected: install burden, less discoverable.

**Decision needed:** None at this layer. Mostly mechanical.

**Scope notes:**
- Exit codes are part of the CLI contract: 0 for success, 1 for verification failure, 2 for usage error, 3 for tool error.
- Output formatting should respect `--json` for machine consumption.

### 4.9 Tree-sitter grammar (`tree-sitter-loom`)

**Goal.** Provide a tree-sitter grammar for editor highlighting and (potentially) as the canonical parser. Maintained in parallel with the chumsky parser for v0, with the parser-canonicalization question left open.

**Default choice.** Tree-sitter grammar emitting a parse tree compatible with editor queries (highlights, locals). Not the canonical parser in v0.

**For:**
- Editor support for free: VS Code, Helix, Neovim, Emacs all consume tree-sitter grammars.
- Future-proofs the canonical-parser decision: if tree-sitter turns out to work well for static analysis too, the grammar is already there.

**Against:**
- Two parsers to maintain.
- Tree-sitter grammar files are JavaScript; less ergonomic than Rust.

**Alternatives considered:**
- *Skip editor support in v0.* Possible but reduces dogfooding quality; we want to use Loom as we build it.
- *LSP-based highlighting only (no tree-sitter).* LSP semantic highlighting is good but requires running the language server. Tree-sitter works statically.

**Decision needed:** ADR-0010 — tree-sitter grammar in v0: parallel-only, or canonical parser.

**Scope notes:**
- Tree-sitter queries for highlights and locals should be in the grammar repo, not assumed to be provided by editors.

### 4.10 Language server (`crates/loom-lsp`)

**Goal.** Provide LSP support for editor integration: diagnostics, hover, go-to-definition, completions for claim forms.

**Default choice.** Deferred. Build after v0 is functional.

**For deferring:**
- LSP work is substantial and orthogonal to demonstrating the architecture.
- Editor highlighting via tree-sitter covers the most common need.

**Against deferring:**
- Without an LSP, the developer experience is meaningfully worse; iteration is slower.
- Dogfooding is harder.

**Alternatives considered:**
- *Include LSP in v0.* Tempting; reject for scope.

**Decision needed:** None at v0 level. ADR can be written when LSP work begins (post-v0).

**Scope notes:**
- `tower-lsp` is the recommended scaffolding when work begins.

### 4.11 Examples (`examples/`)

**Goal.** A set of `.lm` files demonstrating Loom's capabilities, runnable end-to-end through the v0 pipeline, continuously verified by CI.

**Default choice.** Five examples in v0:

1. `01-hello-umbrella/` — minimal: one `knows`, one `relates`, one `shows`, one `does`, one `proves`. Smoke test.
2. `02-ledger/` — the conservation example from the architecture paper §5.2. Demonstrates the umbrella's structure with a meaningful invariant.
3. `03-todo-list/` — a practical small application; demonstrates that Loom can be used for ordinary work.
4. `04-bidirectional-demo/` — designed specifically to demonstrate the gap report doing work: an implementation that establishes more than the umbrella claims, triggering category-(c) findings (see §4.5).
5. `05-spec-quality-demo/` — designed to demonstrate `specq` catching a weak spec: a hand-crafted umbrella where one claim is vacuous, expected to be flagged.

**For five examples:**
- Covers the full pipeline.
- Each example demonstrates one specific capability.
- Enough to feel like a real corpus, not a single toy.

**Against five examples:**
- Each example is a maintenance burden.
- More examples means more places for grammar changes to break things.

**Alternatives considered:**
- *Two examples (hello + one realistic).* Too few; doesn't demonstrate the gap report or specq.
- *Ten examples.* Too many; v0 should be a tight demonstration.

**Decision needed:** ADR-0011 — example corpus contents.

**Scope notes:**
- Each example has a `README.md` walking through what it demonstrates.
- CI runs all examples on every PR; example breakage is a release blocker.

### 4.12 Documentation (`docs/`)

**Goal.** Sufficient documentation that a stranger can clone the repo, build it, and understand both how to use Loom and why it is designed the way it is.

**Default choice.** Markdown documentation, organized by audience:

- **Users** (people writing umbrellas): `docs/reference/language-reference.md`, `docs/reference/claims-reference.md`, `docs/reference/llm-operations.md`, `docs/reference/spec-quality.md`.
- **Contributors** (people working on Loom itself): `docs/reference/verification-internals.md`, `docs/reference/bidirectional-refinement.md`, the ADRs in `docs/adr/`.
- **Researchers** (people evaluating the project's claims): `docs/research/` (background documents).

**For Markdown:**
- No tooling overhead. GitHub renders it. Editors render it.

**Against Markdown:**
- No first-class cross-referencing. Linked sections degrade as files move.
- No code-execution-in-docs (unlike Jupyter or mdbook with executable blocks).

**Alternatives considered:**
- *mdbook.* Lightweight, generates a documentation site with cross-linking. Worth considering when v0 ships; not necessary during build-out.
- *Docusaurus.* Heavier. Not needed at v0 scale.

**Decision needed:** ADR-0012 — documentation tooling for the published site (post-v0).

**Scope notes:**
- The documents listed in §0 are drafted alongside this plan and should be treated as initial versions, expected to evolve.

### 4.13 Continuous integration and release (`.github/workflows/`)

**Goal.** CI that runs the test suite, verifies all examples, runs `specq` on all examples, runs `loom check` on the documentation's embedded examples. Release pipeline that builds binaries for major platforms.

**Default choice.** GitHub Actions. Cargo workspace tests on every push. Examples verified on every push. Binary releases triggered on tags.

**For:**
- Project lives on GitHub; GitHub Actions is the path of least resistance.
- Free for public repositories.

**Against:**
- Vendor lock-in to GitHub. Mitigated because the workflow files are portable.

**Alternatives considered:**
- *None seriously.* CI on GitHub Actions is the default.

**Decision needed:** None at v0 level.

**Scope notes:**
- The CI pipeline includes Dafny installation. Vendor a Dafny version; do not depend on "latest stable."

---

## 5. Out of scope for v0

The following are explicitly deferred. Adding any of them requires re-opening this plan.

### 5.1 Actor runtime and supervised execution

The architecture paper describes a runtime substrate with actors and supervisors. v0 generates plain function-and-module Python; no actor model, no supervisor tree, no message passing. The actor model is a major project on its own and is orthogonal to demonstrating the umbrella's role as an intermediate verified artifact.

### 5.2 Multi-user collaboration / CRDTs

Single-user, single-machine. Concurrent editing of umbrellas, conflict-free merging of claims, multi-user gap reports — all deferred.

### 5.3 Multi-target compilation

One target language in v0 (Python). The architecture's claim about cross-target portability is interesting but not load-bearing for the thesis; deferred.

### 5.4 Capability tracking and effect inference

The umbrella's `@net`, `@db`, `@clock` annotations are documented in the language reference but not enforced in v0. Effect leakage attacks (§3.4 of the spec quality paper) are noted as a limitation. Capability enforcement is post-v0 work.

### 5.5 Live runtime view

The "live runtime view" from the architecture paper — observing a running system and relating its behavior back to umbrella claims — is deferred. v0 verifies statically; runtime observation is post-v0.

### 5.6 Visual editor

No GUI in v0. CLI plus editor support via LSP/tree-sitter only.

### 5.7 Fine-tuned LLM

v0 uses general-purpose LLMs via API. The future-work item from the architecture paper about fine-tuning on umbrella corpora is deferred.

### 5.8 Production deployment story

v0 is a research prototype. Packaging for production use (containers, deployment recipes, monitoring, observability of the verifier) is deferred.

---

## 6. Success criteria for v0

v0 ships when:

1. The five examples (§4.11) all parse, check, verify, codegen, and execute successfully through the standard pipeline.
2. The gap report (§4.5) produces meaningful output on `04-bidirectional-demo/` showing at least one claim that is partially proved and at least one property established by the implementation that the umbrella does not credit (category-c finding).
3. `specq` (§4.7) correctly identifies the weak claim in `05-spec-quality-demo/` and produces a quality report with non-trivial mutation kill rates on the other examples.
4. The LLM operations (`distill`, `generate`, `summarize`) produce usable output on the examples — meaning: a developer can write prose for `03-todo-list`, run `loom distill`, get an umbrella that is structurally correct (passes `loom check`) and substantively reasonable (a human reviewer would not reject it outright). The LLM operations are *useful*, not necessarily *autonomous*.
5. CI runs the full pipeline on every push.
6. Documentation in `docs/` is sufficient for a stranger to clone the repo, build it, and understand what each piece does, with no oral tradition required.
7. A `README.md` exists with a one-page summary and a quickstart that produces a working example.

The criteria above are *necessary*. Things not on the list — performance benchmarks, comparison with other systems, broad user adoption — are *not* v0 criteria. They are post-v0 concerns.

---

## 7. Cross-cutting concerns

### 7.1 Licensing

**Default choice.** Apache-2.0 for the project; CC-BY-4.0 for documentation and research papers.

**For Apache-2.0:**
- Permissive; allows commercial use.
- Patent grant included.

**Against Apache-2.0:**
- More verbose than MIT.

**Alternatives considered:**
- MIT (simpler, no patent grant).
- AGPL (stronger copyleft; would restrict adoption).

**Decision needed:** ADR-0013 — license.

### 7.2 Testing strategy

Unit tests per crate. Integration tests in `crates/loom-verify/tests/` running the full pipeline against fixture inputs. The examples (`examples/`) serve as the end-to-end test suite.

Property-based testing (via `proptest`) for the parser (round-trip: parse-emit-parse should be idempotent) and for the AST→Dafny translation (translation should be type-preserving).

Snapshot testing (via `insta`) for diagnostic output, gap reports, and quality reports. Snapshot review is part of PR review.

### 7.3 Versioning

Semantic versioning. v0 is `0.x.y`; APIs may break between minor versions until v1.0. The umbrella file format is versioned independently from the toolchain; umbrellas declare their format version in a header.

### 7.4 Error handling philosophy

Diagnostics are first-class. The compiler produces structured diagnostics with severity, source location, and explanation. Following Rust's lead, diagnostics include suggested fixes where possible.

Panics in the compiler are bugs. The compiler should never panic on malformed input; it should produce a diagnostic. Internal invariant violations may panic.

### 7.5 Reproducibility

LLM operations are not deterministic, but invocations should be reproducible: same prompt + same model + same temperature produces the same logged invocation. The model's response is recorded in the operation's audit log (in the umbrella's history) so that decisions can be traced even if the LLM's behavior changes between sessions.

Verifier invocations are deterministic given the same Dafny version and the same SMT solver. v0 pins Dafny version; ADR-0014 will pin the SMT solver.

---

## 8. Risks and mitigations

### 8.1 Dafny coupling proves restrictive

*Risk.* Loom claims express things Dafny cannot.

*Mitigation.* `Verifier` trait abstraction (§4.3). Identify the restrictive cases early via the example corpus and triage. If multiple examples cannot translate, the verifier choice is re-evaluated via a fresh ADR.

### 8.2 LLM operations produce unusable output

*Risk.* `loom distill` produces umbrellas the verifier rejects on common prose. `loom generate` produces implementations that do not satisfy umbrellas.

*Mitigation.* Treat the LLM operations as best-effort, not autonomous. Output is a *draft* the human reviews. The architecture does not depend on LLM operations being correct on the first try; it depends on the iteration cycle (LLM proposes, verifier reports, human/LLM revises) converging in a reasonable number of rounds. v0 success criterion §6.4 measures usability, not autonomy.

### 8.3 Mutation testing is too slow to be practical

*Risk.* `specq` mutation runs are slow enough that they are not run regularly, defeating the defense.

*Mitigation.* Cache aggressively at the (claim, mutation, verifier-version) level. Parallelize mutations. Permit sampling. Plan for `specq` runs to happen at gate points (PR review, release) rather than every build.

### 8.4 The architecture is right but the prototype does not convince

*Risk.* v0 demonstrates the pipeline works but does not produce a compelling case for adoption.

*Mitigation.* This is the most important risk and the hardest to mitigate. The example corpus (§4.11) is the primary vehicle for the convincing. `04-bidirectional-demo/` and `05-spec-quality-demo/` specifically exist to demonstrate Loom's contribution over writing Dafny directly. If those examples are not compelling, the v0 prototype's value is uncertain regardless of its technical correctness.

### 8.5 The cheating attractor strikes Loom itself

*Risk.* When Loom is used in development, the LLM games the verifier or the spec quality reporter, producing umbrellas that pass mechanically but are weak.

*Mitigation.* Treat `specq` reports as a metric the team tracks but does not exclusively optimize against. Cross-check via human review of umbrellas at gate points. Update mutation operators when new attack patterns are observed empirically. The defense is layered and includes human judgment by design.

### 8.6 Scope creep into runtime / collaboration / multi-target

*Risk.* During implementation, the temptation to "just add" capability tracking, actors, or multi-target codegen derails v0.

*Mitigation.* This plan's §5 is the contract. Adding anything from §5 requires re-opening the plan via PR review with a new ADR. The temptation is real; the discipline is to defer.

---

## 9. Open decisions (ADR fodder)

The following ADRs need to be written. The first four are drafted in `docs/adr/` alongside this plan; the rest are flagged for future work.

| ID | Topic | Status | Section |
|---|---|---|---|
| ADR-0001 | Implementation language for the compiler (Rust) | Drafted | §4 (workspace-wide) |
| ADR-0002 | Verification backend (Dafny vs F* vs Z3-direct) | Drafted | §4.3 |
| ADR-0003 | Target language for codegen (Python) | Drafted | §4.4 |
| ADR-0004 | Runtime model: no actors in v0 | Drafted | §5.1 |
| ADR-0005 | Parser approach (chumsky vs tree-sitter-canonical) | Open | §4.1 |
| ADR-0006 | Lint rule set and default severities | Open | §4.2 |
| ADR-0007 | Gap report schema and stability commitments | Open | §4.5 |
| ADR-0008 | LLM provider abstraction and v0 default | Open | §4.6 |
| ADR-0009 | specq packaging (workspace vs separate repo) | Open | §4.7 |
| ADR-0010 | tree-sitter as parallel vs canonical parser | Open | §4.9 |
| ADR-0011 | Example corpus contents | Open | §4.11 |
| ADR-0012 | Documentation site tooling (post-v0) | Open | §4.12 |
| ADR-0013 | License | Open | §7.1 |
| ADR-0014 | SMT solver pinning | Open | §7.5 |

Additional ADRs may emerge as components are built. The above is the seed list.

---

## 10. References

### Research background (`docs/research/`)

- [`verifiable-umbrella-paper-v2.md`](docs/research/verifiable-umbrella-paper-v2.md) — the architecture paper that v0 realizes.
- [`spec-quality-under-llm-authorship.md`](docs/research/spec-quality-under-llm-authorship.md) — the companion paper that develops `specq`.
- [`process-gates-and-value-gates.md`](docs/research/process-gates-and-value-gates.md) — related essay on value gates vs. process gates; useful context for §1.3 and §2.4, not a dependency.

### Project-internal references

- [`docs/reference/language-reference.md`](docs/reference/language-reference.md) — the Loom surface language.
- [`docs/reference/claims-reference.md`](docs/reference/claims-reference.md) — the five registers and their claim forms.
- [`docs/reference/verification-internals.md`](docs/reference/verification-internals.md) — how Loom translates to Dafny.
- [`docs/reference/bidirectional-refinement.md`](docs/reference/bidirectional-refinement.md) — the gap report and the discipline.
- [`docs/reference/llm-operations.md`](docs/reference/llm-operations.md) — distill, generate, summarize.
- [`docs/reference/spec-quality.md`](docs/reference/spec-quality.md) — using and extending `specq`.

### External

- Dafny — verification backend: https://dafny.org/
- F* — alternative verification backend: https://www.fstar-lang.org/
- Z3 — SMT solver used by Dafny: https://github.com/Z3Prover/z3
- Anthropic API — LLM backend: https://docs.claude.com/

---

*This plan is the seed of the project. It is expected to evolve as decisions are made and ADRs are written. Substantial changes to commitments in §2 or §5 require explicit re-opening of this document.*
