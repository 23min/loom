# Loom

## Why Loom exists

LLM-assisted coding has moved through phases. Prompt engineering: coax the right answer. Vibe coding: stop coaxing, let the model run. Spec-driven development: write the requirements down first and have the model implement them. Each phase tightened the front of the pipeline. None of them changed the back: you still read the output and hope.

What comes next has to be guarantees. Not better prompts, not stricter specs — a way to mechanically check that the code does what was claimed, and to surface the parts that were not checked rather than quietly absorb them. Loom exists to provide that.

## The idea

Between the prose that states the intent and the code that implements it, Loom inserts a third artifact: a short, structured document — an **umbrella** — short enough for a person to read end to end, precise enough for a verifier to check the implementation against claim by claim. The output is not pass-or-fail: anything the verifier cannot establish surfaces as an explicit **gap**, so what hasn't been proved is as visible as what has.

## What it is

Loom is a research prototype of the **Verifiable Umbrella** architecture: a three-layer model for software construction in which

1. **Prose** captures human intent in durable form,
2. an **umbrella** of structured formal claims sits as the verified intermediate artifact between prose and code, and
3. **siblings** of LLM-authored implementation modules are mechanically verified against the umbrella's claims.

The umbrella is small enough for a human to read fully; the implementation is detailed enough for a verifier (Dafny, initially) to check against it. The discipline is *bidirectional*: obligations flow downward, properties flow upward, and the difference is reified as a **gap report** — the load-bearing visible artifact of the discipline.

Loom v0 is a single-user research prototype intended to validate the architecture. It is not a general-purpose programming language, and it is not production-grade.

## Status

Seed stage. Code does not yet exist. The repository currently holds:

- [`PLAN.md`](PLAN.md) — the v0 plan, the seed document that drives everything else.
- [`project-structure.md`](project-structure.md) — the intended repository layout.
- [`docs/`](docs/) — language reference, claims reference, verification internals, bidirectional refinement, LLM operations, spec quality, ADRs, and the four background research documents under [`docs/research/`](docs/research/).

## Where to start

- New here? Read [`docs/research/verifiable-umbrella-paper-v2.md`](docs/research/verifiable-umbrella-paper-v2.md) for the architecture, then [`PLAN.md`](PLAN.md) for what v0 commits to building.
- Looking for the language? [`docs/reference/language-reference.md`](docs/reference/language-reference.md) and [`docs/reference/claims-reference.md`](docs/reference/claims-reference.md).
- Looking for the discipline? [`docs/reference/bidirectional-refinement.md`](docs/reference/bidirectional-refinement.md).
- Wondering how far guarantees reach *across* modules — the "global"/architecture question? [`docs/reference/compositional-correctness.md`](docs/reference/compositional-correctness.md), with the worked two-umbrella [`examples/05-composition/`](examples/05-composition/).
- Worried about LLMs gaming the specs? [`docs/reference/spec-quality.md`](docs/reference/spec-quality.md) and [`docs/research/spec-quality-under-llm-authorship.md`](docs/research/spec-quality-under-llm-authorship.md).
