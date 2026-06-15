# ADR-0001: Rust as the implementation language for the Loom compiler

**Status:** Proposed (drafted alongside `PLAN.md`; awaiting first-PR ratification)
**Date:** 2026-05-22
**Deciders:** project initial author; open for review.
**Related:** `PLAN.md` §4 (workspace-wide), §4.1 (parser), §4.3 (verifier backend).

---

## Context

The Loom v0 plan calls for a multi-crate codebase implementing a parser, static checker, two compilation backends (verifier-direction and execution-direction), an orchestrator, an LLM client, a spec quality reporter, and a CLI. The implementation language for the compiler itself is a foundational choice: it constrains the ecosystem the project will draw on, the contributors who can engage with the code, the binary distribution story, and (subtly) the design vocabulary the team uses when discussing the system.

The candidate languages considered are Rust, OCaml, Haskell, Go, and TypeScript. Python and Java are excluded a priori as poor fits for a compiler-class tool.

The project's needs from its implementation language:

- A parser ecosystem with mature tooling for either hand-written or generated parsers.
- Good bindings to Z3 and to subprocess management (for invoking Dafny).
- An ergonomic concurrency story for parallelizing mutation testing in `specq`.
- Static distribution as binaries on Linux, macOS, and Windows for end users.
- A type system rich enough to express the AST, the source map, and the verifier abstraction without ceremony.
- A community of contributors with sufficient critical mass that the project does not languish for lack of accessible developers.

---

## Decision

Adopt **Rust** as the implementation language for the Loom compiler and all tooling crates (`crates/loom-syntax`, `crates/loom-check`, `crates/loom-compile-dafny`, `crates/loom-compile-python`, `crates/loom-verify`, `crates/loom-llm`, `crates/specq`, `crates/loom-cli`).

The Cargo workspace structure manages cross-crate dependencies and shared builds.

---

## Considered alternatives

### OCaml

**For OCaml.** OCaml is the natural language of formal methods. F* is implemented in OCaml; Dafny's lineage (Boogie) has OCaml-adjacent components; the verification community uses OCaml extensively. Algebraic data types, pattern matching, and immutability are first-class. If the project ever needs to engage deeply with the F* community (e.g., by considering F* as a verification backend per ADR-0002), an OCaml implementation would lower the barrier.

**Against OCaml.** The contributor base is meaningfully smaller. Tooling (debugger, language server, package management via `opam`) has improved substantially but still trails Rust's. Cross-platform binary distribution is more involved. The standard library has gaps that lead to ad-hoc third-party choices. Most importantly, the project's success will depend on attracting contributors familiar with both compiler engineering *and* the project's research thesis; OCaml restricts the first half of that intersection more than Rust does.

### Haskell

**For Haskell.** Strong type system. Excellent parser libraries (Megaparsec, Attoparsec). Strong literature on compiler implementation. Pure-by-default fits the project's immutability-everywhere philosophy.

**Against Haskell.** Laziness creates operational opacity that hurts in compiler work (where evaluation order matters for diagnostics and performance). The contributor base is even smaller than OCaml's. Build tooling (`cabal`, `stack`) is workable but adds friction. Binary distribution is harder than Rust.

### Go

**For Go.** Excellent tooling, fast builds, easy cross-platform binaries, large contributor base, good subprocess management. Simple language semantics reduce on-boarding cost.

**Against Go.** Type system is too weak for the kind of AST machinery the project needs (no algebraic data types, limited generics until recently, no pattern matching). Compiler work in Go tends toward verbose code that obscures intent. The project's design vocabulary leans algebraic; Go does not reward that vocabulary.

### TypeScript / Node

**For TypeScript.** Rich type system in the structural-typing tradition. Excellent ecosystem. Mature LLM SDKs (the Anthropic SDK is most polished in TypeScript). Easy to write tooling that integrates with editors.

**Against TypeScript.** Runtime is Node.js, which is a non-trivial dependency for end users. Performance is acceptable but not great for verification orchestration. Long-running processes (mutation testing) face memory-management quirks. The structural type system is excellent for application code but less ergonomic than nominal types for compiler ASTs. Static distribution as a single binary requires bundling (Bun, `pkg`, etc.) and is less mature.

### Rust

**For Rust.**

- Mature parser ecosystem: `chumsky`, `pest`, `lalrpop`, `nom`, plus hand-rolled options. The project's needs are well-served.
- Excellent algebraic data types and pattern matching. Compiler ASTs and source maps are ergonomic to express.
- Static distribution as single binary per target platform. No runtime dependency.
- Strong concurrency story via `rayon` (for mutation parallelism) and `tokio` (if async networking is needed for LLM calls).
- Z3 bindings exist (`z3` crate) for the post-v0 path where direct Z3 invocation becomes attractive.
- Subprocess management is straightforward (`std::process::Command`).
- LSP infrastructure is good (`tower-lsp`) for the post-v0 LSP work.
- Active contributor community across systems work and DSL/compiler work.
- Tree-sitter has first-class Rust bindings.

**Against Rust.**

- Borrow checker has a real learning cost. Contributors coming from OCaml/Haskell may find the ownership model intrusive when expressing pure transformations.
- Compile times for the workspace will not be fast, particularly for full rebuilds. Mitigation via `cargo`'s incremental compilation and careful workspace structure.
- The project's philosophy (immutable, pure, recursive AST transformations) is best expressed in ML-family languages; Rust expresses it well but with more annotation overhead.
- Some F*-community contributors may be more comfortable in OCaml. The project would benefit from cross-pollination with that community; Rust adds friction.

---

## Consequences

### Positive

- The workspace structure (Cargo workspace) provides clear crate boundaries with shared dependencies, which aligns with the project's architectural commitment to per-component clarity.
- Single-binary distribution simplifies the user experience.
- The contributor pool is large enough to sustain development.
- Z3, LSP, tree-sitter, and subprocess work are all well-supported.

### Negative

- Contributors from the formal-methods community who prefer OCaml face a switching cost.
- Compile times will require attention; the workspace must be structured to allow incremental builds and to avoid serialization through any one crate.
- The borrow checker will produce occasional friction when expressing ASTs that contain references rather than owned values; the project should commit to owned-AST representations (no lifetimes in the AST types) to keep things ergonomic.

### Neutral

- The choice does not preclude future bindings or alternative implementations in other languages (e.g., a TypeScript port of the parser for browser-based tooling).
- Some auxiliary tools may end up in other languages (e.g., the tree-sitter grammar is in JavaScript per tree-sitter's conventions); this is contained and does not affect the core.

---

## Implementation notes

- Workspace structure: per `PLAN.md` §3.
- AST types own their data; no borrowed references. Each AST node has a span (a small `Copy` type) and owns its children via `Box` or `Vec`.
- Diagnostics use `miette` or `ariadne` for spans and rendering; choice deferred to a follow-up ADR.
- Cross-crate types live in a small `loom-common` crate to avoid circular dependencies.

---

## References

- `PLAN.md` §3 (workspace structure), §4 (components).
- Cargo workspace documentation: https://doc.rust-lang.org/cargo/reference/workspaces.html
- The companion ADR-0002 (verifier backend) does not depend on this choice but is informed by it.
