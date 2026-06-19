# LLM operations

> **Status:** draft (v0 design)
> **Audience:** users invoking `loom distill`, `loom generate`, and `loom summarize`; contributors maintaining the prompt library.

This document describes the three LLM-mediated operations Loom provides, their design principles, and the conventions for the prompt library.

---

## 1. Why these three operations

The architecture paper [`docs/research/verifiable-umbrella-paper-v2.md`](../research/verifiable-umbrella-paper-v2.md) §4.10 identifies the LLM's role in the three-layer model as transformation between layers:

- **`distill`** transforms prose → umbrella (downward: human's loose description becomes a structured umbrella).
- **`generate`** transforms umbrella → sibling implementation (downward: claims become code that satisfies them).
- **`summarize`** transforms sibling claims → parent umbrella's `summarizes` register (upward: completed children let a parent claim system-level properties; post-v0).

Each operation has a specific input artifact, output artifact, and verification target. The verifier and `specq` provide the validation; the LLM provides the candidate.

The three operations are deliberately *named* (not generic chat) because each has a well-defined contract:

- **Inputs and outputs** are typed artifacts (markdown, `.lm` files, code files), not free-form text.
- **Validation** is mechanical (the output goes through `loom check`, `loom verify`, `loom specq`).
- **The LLM is one component in a pipeline**, not the pipeline itself.

This shape matters because it bounds the LLM's role. The LLM proposes; the verifier disposes; the human ratifies.

---

## 2. `distill`: prose → umbrella

### 2.1 Goal

Transform human prose describing a system into an initial Loom umbrella. The output is a structurally well-formed umbrella (passes `loom check`) and a reasonable first attempt at capturing the prose's content.

### 2.2 Inputs

- A Markdown file describing the system in prose. Sections, headings, examples in the prose are encouraged but not required.
- Optionally, references to existing umbrellas the new one should compose with (`--extends path/to/parent.lm`).

### 2.3 Output

- A `.lm` file with all five registers populated.
- The umbrella passes `loom check`.
- The umbrella's claims should align with the prose's content, though human review is expected and the LLM's draft is rarely the final form.

### 2.4 Prompt design

The prompt for `distill` includes:

1. The Loom language reference (programmatically: just the relevant sections — registers, syntax — not the full document).
2. The five-register schema with examples.
3. The cross-register coverage rules.
4. The prose to distill.
5. Instructions: "produce a Loom umbrella with all five registers. Use refinement types where appropriate. Provide at least two examples per operation. Include at least one property per operation. Mark uncertain claims with `// review` comments."

The prompt template is in `crates/loom-llm/prompts/distill.md`. The prompt is structured so the LLM produces output in the expected format reliably.

### 2.5 What `distill` does well

- Captures explicit content from the prose into structure.
- Names types and operations consistently.
- Provides plausible initial examples.

### 2.6 What `distill` does badly (the known weaknesses)

- **Generates weak claims.** The LLM tends toward properties it can clearly satisfy. The companion paper's §3 attack patterns (claim weakening, vacuous antecedents) are likely to appear in `distill` output. `specq` is essential after `distill`.
- **Under-specifies in interesting ways.** The LLM may capture the prose's explicit content but miss invariants that are implicit. The bidirectional gap report (category C) can recover some of these.
- **Names things idiosyncratically.** The first draft may have plausible but suboptimal names. Renaming is mechanical and worth doing before iterating further.
- **Over-uses gaps.** When uncertain, the LLM marks claims as gaps. The gap-discipline checks in `specq` flag accumulating gaps.

### 2.7 Workflow

```bash
loom distill prose.md -o umbrella.lm
loom check umbrella.lm                # structural check
loom specq umbrella.lm                # weak-claim check
loom verify --do-not-run umbrella.lm  # check syntactic Dafny correctness (no impl yet)
# review umbrella.lm
# either accept or iterate by editing prose.md and re-running, or by editing umbrella.lm directly
```

The first iteration is rarely the final umbrella. Treat `distill` output as a draft.

---

## 3. `generate`: umbrella → implementation

### 3.1 Goal

Given an umbrella with `knows`, `relates`, `shows`, and `proves` populated (but possibly empty `does`), produce an implementation in `does` that satisfies the contracts. The implementation should pass `loom verify`.

### 3.2 Inputs

- A `.lm` file with the four non-`does` registers populated.
- The `does` register may be empty, partially populated, or contain TODO markers.

### 3.3 Output

- The same `.lm` file with `does` populated.
- The file passes `loom verify` (all claims in category A) for the generated parts.

### 3.4 Prompt design

The prompt for `generate` includes:

1. The umbrella's `knows`, `relates`, `shows`, and `proves` contents (programmatically extracted).
2. Examples of `does` bodies from a small curated set (boilerplate templates).
3. Instructions: "produce a `does` body for each operation in `relates` that satisfies the `requires`/`ensures` contracts and the `proves` claims. Use immutable updates (`with` syntax). Use named `let` bindings for intermediate values when it improves readability. Do not introduce new types or operations."

`generate` is restricted: it may not modify `knows`, `relates`, `shows`, or `proves`. If the LLM finds the contracts unsatisfiable, the right output is a diagnostic ("unable to generate; the postcondition appears to contradict the precondition in operation X") rather than a silent weakening of the contracts.

The prompt template is in `crates/loom-llm/prompts/generate.md`.

### 3.5 What `generate` does well

- For operations with tight pre/postconditions (like `open_account` whose ensures determines the result completely), produces the right body on the first try.
- For algebraic transformations (record updates, arithmetic), produces clean expressions.

### 3.6 What `generate` does badly

- **Conjures unnecessary cases.** When `does` should be a one-liner, the LLM may produce a `match` over variants that don't exist or branches that aren't reachable. Review carefully.
- **Fails on under-specified contracts.** If `relates` does not pin down the result (e.g., postcondition `ensures result.balance >= 0` without saying *what* the balance should be), `generate` produces *some* implementation satisfying the constraint, but probably not the intended one. Either tighten the contract or accept that `generate` will pick.
- **Verification failures.** Sometimes the LLM's first attempt produces code Dafny rejects. The orchestrator should support an iterative loop: generate → verify → on failure, feed the verifier's diagnostics back to the LLM and regenerate. This loop is bounded (e.g., 3 iterations) to avoid runaway costs.

### 3.7 Workflow

```bash
loom generate umbrella.lm             # populates `does`
loom verify umbrella.lm               # verifies the new `does`
# on success: review and commit
# on failure: inspect diagnostics, either fix manually or `loom generate --iterate` to retry with feedback
```

---

## 4. `summarize`: child claims → parent summarizes (post-v0)

### 4.1 Goal

When a parent umbrella claims system-level properties grounded in children's claims, the `summarizes` register expresses the relationship: *the parent's claim follows from the conjunction of these children's claims*. `summarize` produces the `summarizes` register's content from the children's umbrellas.

### 4.2 Inputs

- A parent umbrella (with `summarizes` to be populated).
- A set of children umbrellas (each with their own `proves` contents).

### 4.3 Output

- The parent's `summarizes` register, populated with claims of the form:
  ```
  parent_claim:
    requires { child_a.claim_x, child_b.claim_y }
    means { ... }
  ```

### 4.4 Status

`summarize` is post-v0. The shape of the operation is documented here for design continuity but is not implemented in v0.

The challenge is that `summarize` is the only operation where the LLM is asserting a *derivation*: that the parent's `means` follows from the children's `requires`. The derivation is a small proof. Verifying it requires the verifier to discharge the implication mechanically. This is straightforward in principle but requires the cross-umbrella verification path the v0 plan defers (§4.6 of [`docs/claims-reference.md`](claims-reference.md), §5.7 of `PLAN.md`).

---

## 5. Prompt library

### 5.1 Location

Prompts live in `crates/loom-llm/prompts/`:

```
crates/loom-llm/prompts/
├── distill.md
├── generate.md
└── summarize.md       # post-v0
```

Each prompt file is a Markdown document with three sections:

- **Context** (~30% of length): describes what the LLM is being asked to do, including the Loom language reference excerpts needed.
- **Examples** (~50%): one or more curated input/output pairs showing the expected transformation.
- **Instructions** (~20%): the specific operation, format requirements, and constraints.

### 5.2 Versioning

Prompts are versioned via filenames or git history (TBD). When a prompt changes, the LLM operations record the prompt version in the operation's audit log. This is so that reproducibility is preserved even as prompts evolve.

### 5.3 Maintenance

When `specq` or `loom verify` reveals systematic weaknesses in LLM output, the prompts are updated. The pattern is:

1. A new weakness is observed (e.g., the LLM produces postconditions that are conjunctions of preconditions).
2. The weakness is added to the prompt's "anti-patterns to avoid" section.
3. The prompt is re-evaluated on a small test corpus to confirm the change reduces the weakness without introducing new ones.

Prompt regressions are bugs; prompt improvements are PRs.

---

## 6. The LLM provider abstraction

```rust
pub trait LLMProvider {
    fn complete(&self, prompt: &str, params: &Params) -> Result<Completion, LLMError>;
    fn name(&self) -> &'static str;
    fn model(&self) -> &str;
}
```

`LLMProvider` abstracts the LLM API call. v0 ships with an `AnthropicProvider` implementation. Other providers can be added without changing the operations.

Provider configuration is in `loom.toml`:

```toml
[llm]
provider = "anthropic"
model = "claude-3-opus"
temperature = 0.0
max_tokens = 4000
```

Temperature is 0.0 by default for reproducibility; higher temperatures may produce more creative but less consistent output.

### 6.1 Authentication

API keys come from environment variables (`ANTHROPIC_API_KEY`, etc.) and are never written to disk. The CLI reports a clear diagnostic if the required key is missing.

### 6.2 Cost tracking

Each operation logs the token count (input and output) and estimated cost in the audit log. Users can inspect costs via `loom llm-stats`.

---

## 7. Audit log

Every LLM operation appends a record to an audit log at `.loom/llm-audit.jsonl`:

```json
{
  "operation": "distill",
  "timestamp": "2026-05-22T14:30:00Z",
  "provider": "anthropic",
  "model": "claude-3-opus",
  "prompt_version": "distill@a1b2c3",
  "prompt_tokens": 1234,
  "completion_tokens": 567,
  "input_hash": "sha256:...",
  "output_hash": "sha256:..."
}
```

The audit log is git-tracked. Reviewers can see when an LLM operation was invoked, with what inputs, and what it produced (by hash; the actual content is in the artifact's history).

The audit log is one of the substrate-walk-back's substrate-compatible mechanisms: it's append-only, but each entry is independent (no hash chain), and merges across branches concatenate naturally.

---

## 8. The orchestrator's role

The CLI orchestrates the LLM operations:

```bash
loom distill prose.md --output umbrella.lm
loom generate umbrella.lm                       # in-place modification
loom generate umbrella.lm --iterate --max-tries 3
```

The orchestrator:

1. Validates inputs (file exists, is well-formed).
2. Constructs the prompt from the template + inputs.
3. Calls the LLM provider.
4. Parses the LLM's output.
5. Validates the output against schema constraints (does the output have all five registers? does it pass `loom check`?).
6. If validation fails and `--iterate` was specified, feeds the failure back into a follow-up prompt.
7. Writes the validated output.
8. Logs to the audit trail.

The orchestrator does not invoke the verifier; it produces an artifact for the user to run `loom verify` on. The separation is intentional: LLM operations are about *producing artifacts*, verification is about *checking them*. The user makes the choice to verify; the LLM operations don't presume.

---

## 9. Cost and rate-limiting

### 9.1 Cost

`distill` on a small prose document: tens of cents.
`generate` on a small umbrella: tens of cents.
`specq --full` invoking the LLM for explanation generation: dollars (per umbrella, depending on mutation count).

Costs scale roughly linearly with input and output sizes. Loom does not impose a cost ceiling but reports estimated cost in the audit log. Users running in CI should be aware of the costs.

### 9.2 Rate-limiting

The provider's rate limits apply (e.g., Anthropic's per-minute and per-day quotas). The CLI surfaces rate-limit errors clearly. For high-volume use (e.g., running `loom generate` across many umbrellas in CI), users should configure batched processing or use higher-tier API access.

### 9.3 Caching

LLM responses are cached by `(prompt_hash, model, temperature)`. Repeated invocations with identical inputs hit the cache. The cache lives in `.loom-cache/llm/` and is excluded from git.

---

## 10. Limitations and known issues

### 10.1 Non-determinism

LLMs at temperature > 0 are non-deterministic. Even at temperature 0, identical prompts may produce slightly different outputs across API versions. Loom records the model and prompt versions so that surprises are explicable.

### 10.2 Hallucinated references

LLMs may produce umbrellas referencing types or operations that don't exist (in `knows` or imported modules). `loom check` catches these; the user must correct manually or re-run `distill` with clearer prose.

### 10.3 Quality varies by domain

LLMs are better at familiar domains (CRUD, simple state machines, accounting) than novel ones (cryptographic protocols, distributed consensus, hardware models). Loom's prompts focus on common cases; users in unfamiliar domains should expect more iteration and more human review.

### 10.4 The threat model applies here

The companion paper's threat model — claim weakening, gap-as-escape, example narrowing — applies directly to `distill`. The defense is `loom check` + `loom specq`, not the prompt. Improving the prompt is a partial mitigation; relying on it solely would be insufficient.

---

## 11. References

- [`docs/research/verifiable-umbrella-paper-v2.md`](../research/verifiable-umbrella-paper-v2.md) §4.10, §6 — original architecture.
- [`docs/research/spec-quality-under-llm-authorship.md`](../research/spec-quality-under-llm-authorship.md) — threat model for LLM-authored specifications.
- [`docs/spec-quality.md`](spec-quality.md) — `specq`, the layered defense.
- [`docs/language-reference.md`](language-reference.md) — the language the LLM is generating.
- Anthropic API docs: https://docs.claude.com/
