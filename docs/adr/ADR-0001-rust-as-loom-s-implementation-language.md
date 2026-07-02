---
id: ADR-0001
title: Rust as loom's implementation language
status: proposed
---
# ADR-0001 — Rust as loom's implementation language

> **Date:** 2026-05-22 · revised 2026-07-02 · **Deciders:** project initial author; ratified during E-0005 planning.
> **Related:** ADR-0002 (Dafny verifier) · ADR-0017 (loom generates no target code) · ADR-0003 (Python codegen — rejected).
> **Revision note:** the Rust decision stands and is affirmed; the original "code-generating compiler" framing (a Python execution backend, `crates/loom-compile-python`) is corrected here — loom generates no target code (ADR-0017).

---

## Context

loom is a **verification** tool: it reads an umbrella (claims) and an implementation, lowers the claims to a verifier (Dafny — ADR-0002), runs it, and emits a gap report. It does **not** generate target/executable code — code generation, where wanted, is the LLM's role (ADR-0017). The original plan's "execution-direction backend" / `crates/loom-compile-python` is retired.

Two properties dominate the implementation-language choice:

1. **loom is a correctness tool and should embody its own stance** — robustness, type safety, and elegance in expressing the umbrella AST, source maps, and the verifier abstraction, without ceremony.
2. **loom must be host-agnostic** — used mostly by downstream consumers across many languages, so its implementation language must not be chosen to match any one host. aiwf (the first consumer) is Go; that is **incidental and explicitly not a reason.**

Candidates considered: Rust, OCaml, Haskell, Go, TypeScript.

## Decision

Adopt **Rust** as loom's implementation language for all engine and tooling crates (the umbrella loader/checker, the Dafny-lowering backend, the verifier abstraction, the LLM client, the spec-quality reporter, and the CLI/runner), managed as a Cargo workspace. There is **no** code-generation backend: loom lowers claims to a verifier; it does not emit target code.

Rationale: algebraic data types + pattern matching make the umbrella AST and source maps ergonomic; single-binary distribution keeps loom dependency-free for any downstream regardless of that repo's language; Z3 bindings, subprocess management (for invoking Dafny), and `rayon` concurrency are well-supported; and a strong type system makes loom embody the very correctness discipline it exists to promote.

## Considered alternatives (summary)

- **OCaml / Haskell** — natural languages of formal methods with strong type systems, but meaningfully smaller contributor bases and harder cross-platform single-binary distribution.
- **Go** — excellent tooling and subprocess story, but a type system too weak for the umbrella-AST machinery (no ADTs, limited pattern matching); and, decisively here, choosing it would only be to match a host — which loom must not do.
- **TypeScript** — rich structural types and the most polished LLM SDK, but a Node runtime dependency undercuts host-agnostic single-binary distribution.
- **Rust** — AST ergonomics, host-agnostic single-binary distribution, Z3/subprocess/concurrency support, and a correctness posture that fits a verification tool.

## Consequences

- **Positive.** Single-binary, host-agnostic distribution — loom drops into any downstream irrespective of that repo's language. Z3, subprocess, and concurrency are well-supported. The type system reinforces loom's own correctness stance.
- **Negative.** Contributors from the OCaml/Haskell formal-methods world face a switching cost; the borrow checker has a learning curve — mitigated by an owned-AST representation (no lifetimes in AST types).
- **Neutral.** The choice does not preclude a future language surface (e.g. a `.lm` parser) or auxiliary tools in other languages.

## References

- ADR-0002 (Dafny verifier) · ADR-0017 (loom generates no target code) · ADR-0003 (Python codegen — rejected).
- The E-0004 Rust ultralight harness (`experiments/loom-ultralight`) that the E-0005 runner reuses.
