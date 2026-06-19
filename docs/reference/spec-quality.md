# Spec quality

> **Status:** draft (v0 design)
> **Audience:** users running `loom specq`; contributors maintaining the mutation engine, defenses, and reporting.
> **Companion:** [`docs/research/spec-quality-under-llm-authorship.md`](../research/spec-quality-under-llm-authorship.md) — the research paper that develops the threat model and the techniques. This document is the implementation-side view of the same content.

---

## 1. What `specq` does

`specq` is the spec quality reporter: a wrapper around `loom verify` that detects characteristic weaknesses in umbrellas. It exists because the companion paper's threat model — when LLMs author specifications, gradient incentive pushes toward specifications that are easy to satisfy — is part of Loom's architectural commitment (§2.4 of `PLAN.md`).

Running `specq` against an umbrella produces a *spec quality report* alongside the normal verification output. The report identifies specific weaknesses (vacuous claims, narrow examples, gappy clauses, decorative postconditions) with diagnostic location information and suggested fixes.

`specq` is not optional. It is part of the standard pipeline. Umbrellas that pass `loom verify` but fail `loom specq` are not considered verified for the purposes of Loom's architectural guarantees.

---

## 2. The layered defense

Spec quality is enforced by a stack of defenses, ordered from cheapest to most expensive:

1. **Grammar bans** — syntactic restrictions on the specification language. Trivial vacuities are simply unparseable.
2. **Cross-register coverage** — every register must engage with the others; unused vocabulary is flagged.
3. **Anti-pattern detection** — lint-style rules for known weak shapes.
4. **Domain engagement** — statistical measures of how much of the input space the spec exercises.
5. **Gap discipline** — checks on how gaps are declared and how their count evolves.
6. **Mutation testing on claims** — the central novel technique.

Each layer catches a class of weaknesses the cheaper layers miss. Together they constitute the spec quality measurement.

---

## 3. Grammar bans

Implemented at the parser level (`crates/loom-syntax`).

### 3.1 Banned forms

The following are rejected at parse time:

- Properties whose body is `true`, `false`, or `x = x` for any literal `x`.
- Postconditions that are a disjunction over every value of the output (trichotomy on numerical outputs, exhaustive enumeration of variant cases without any constraint).
- Refinement type predicates that are `true` or that match every value of the underlying type.

### 3.2 Examples

```loom
// REJECTED at parse
proves {
  bad: for-all x: int, true       // ERROR: vacuous body
}

knows {
  Bad :: {x: int | true}           // ERROR: vacuous refinement
}
```

Grammar bans are the cheapest defense. They catch the laziest cheats and signal to the LLM (via the diagnostic) that this surface form is unacceptable. The LLM may then produce semantically equivalent obscurations, which are caught at the next layer.

---

## 4. Cross-register coverage

Implemented in `crates/loom-check`.

### 4.1 Coverage rules

- Every type in `knows` must appear in at least one `relates` signature, `shows` example, or `proves` body.
- Every predicate in `knows` must appear in at least one `requires`, `ensures`, or `proves` body.
- Every operation in `relates` must have an entry in `does`, at least one example in `shows`, and at least one property in `proves`.
- Every example in `shows` must reference an operation in `relates`.
- Every property in `proves` must mention an operation (i.e., the body must contain a function application, not be a closed predicate over types only).

### 4.2 Severity

In v0, these are warnings (some operations are trivially uninteresting). v0.x raises them to errors with explicit suppression.

### 4.3 Suppression

For intentional exceptions:

```loom
knows {
  #[allow(unused_type, reason = "used by downstream importers")]
  UtilityType :: ...
}
```

The `reason` field is required when suppressing; reviewers can audit suppressions.

---

## 5. Anti-pattern detection

Implemented in `crates/loom-check/src/anti_patterns.rs`.

Lint-style rules for shapes correlated with weak specifications:

### 5.1 Patterns flagged

- **Output-blind properties.** A `proves` body that mentions only inputs and not the operation's output (or `result`).
- **Refinement equivalences.** A refinement predicate that is logically equivalent to the underlying type's universal predicate (e.g., `{x: int | x = x}`).
- **Postcondition-as-precondition repeats.** `ensures` body that is exactly the `requires` body, asserting nothing new.
- **Empty-only examples.** All `shows` examples use empty/zero values of every type they touch.
- **Eventually-without-bound.** Claims of the form `eventually X` without a time or step bound. (v0.x; bounds syntax not in v0.)
- **Refinement-via-undefined-predicate.** `{x: T | foo(x)}` where `foo` is itself defined to be `true`.

### 5.2 Configuration

Each pattern can be set to `allow`, `warn`, or `error` in `loom.toml`:

```toml
[lint]
output_blind_properties = "warn"
postcondition_repeats_precondition = "error"
empty_only_examples = "warn"
```

### 5.3 Adding new patterns

When `specq` deployment reveals new attack patterns (e.g., a novel obscuration the LLM finds), they are added to the anti-pattern set. Each addition is reviewed for false-positive risk before landing.

---

## 6. Domain engagement

Implemented in `crates/specq/src/domain.rs`.

### 6.1 Precondition saturation

For each operation, generate K random values from the input types (via property-based fuzzing). Count how many satisfy the operation's precondition. Report the ratio.

```
operation: transfer
  precondition: from.open ∧ to.open ∧ from.balance >= amount
  sampled 1000 values from (Account, Account, PositiveAmount)
  precondition satisfaction rate: 92%
```

A high rate is healthy. A low rate (e.g., 3%) means the precondition rejects most of the input domain, which may be intentional but warrants reviewer attention:

```
operation: transfer
  precondition satisfaction rate: 3%
  
  FINDING [precondition-narrow]:
    The precondition rejects 97% of values drawn from the input types.
    This may indicate over-restriction (the operation hardly ever applies)
    or a need to tighten the input types instead of the precondition.
```

### 6.2 Example diversity

For the examples in `shows`, measure:

- **Type coverage.** Of all the types referenced by an operation, do the examples include values at boundaries (zero, max, just-inside-bounds, empty/non-empty for collections)?
- **Variant coverage.** For sum types, are all variants exercised by at least one example?

Reported per operation:

```
operation: describe_result
  variant coverage: 2/3 (missing AccountClosed)
  
  FINDING [unexercised-variant]:
    The TransferResult variant AccountClosed has no example.
    Examples cover Success and InsufficientFunds.
```

### 6.3 Boundary engagement

For numeric types and refinement types, check whether examples include boundary values. Reported per type:

```
type: Money (= {x: int | x >= 0})
  examples using Money: 7 total
  boundary engagement:
    zero (the type's minimum):  no example  ⚠
    small (1-10):               4 examples
    large (>1000):              3 examples
```

### 6.4 Configurability

Sample sizes, boundary definitions, and severity levels are configurable in `loom.toml`. Defaults are reasonable for v0; deployments may tune.

---

## 7. Gap discipline

Implemented in `crates/specq/src/gaps.rs`.

### 7.1 Structured gap requirement

Gaps must be declared with a typed restriction:

```loom
proves {
  conservation:
    for-all from: Account, to: Account, amount: PositiveAmount, ...
  
  // explicit gap with typed restriction
  gap conservation: when concurrent_transfers(...)
}
```

The restriction (`when concurrent_transfers(...)`) must be a closed predicate. Free-prose gap notes ("see issue #123") are not gaps; they are TODO comments and do not modify the umbrella's guarantee structure.

### 7.2 Gap drift monitoring

`specq --history` reports gap count over the umbrella's commit history (read from git):

```
Gap drift for examples/02-ledger/ledger.lm
  Commit a1b2c3 (2026-04-01): 0 gaps
  Commit d4e5f6 (2026-04-15): 1 gap (conservation: concurrent)
  Commit g7h8i9 (2026-05-01): 3 gaps (added: fairness:long-running, ordering:permissive)
  Commit j1k2l3 (HEAD):       3 gaps

  TREND: gap count increasing
```

A trend of increasing gaps is a finding worth reviewer attention; it may indicate spec strength is regressing.

### 7.3 Gap-coverage analysis

For each gap, measure what fraction of the type domain falls under the gap's restriction:

```
gap conservation: when concurrent_transfers(T)
  estimated fraction of (Account, Account, Transfer) tuples falling in restriction: 8%
  
  Note: 92% of the domain is still covered by the (non-gap) claim.
```

If a gap covers most of the type domain (e.g., 90%), the unrestricted claim covers very little:

```
gap conservation: when high_volume_transfers(T)
  estimated fraction in restriction: 85%
  
  FINDING [gap-dominates]:
    The gap restriction covers most of the operation's domain.
    The claim's effective coverage is only 15% of inputs.
    Consider whether the gap should be promoted to a hard precondition
    or whether the claim should be proved over the harder cases.
```

---

## 8. Mutation testing on claims

The central novel technique. Implemented in `crates/specq/src/mutate/`.

### 8.1 Mechanics

For each `proves` claim, the mutation engine produces a set of *mutants*: small syntactic perturbations of the claim. Each mutant is sent through the verifier. The outcome (verified, failed, timeout, equivalent) is recorded.

The diagnostic — refined per §6.4 of the companion paper — is:

- **Strengthening mutations** (the claim becomes harder to satisfy). If verification still passes, the implementation establishes the strengthened claim too. This indicates *spec slack*: the original claim was understating what the implementation does.
- **Weakening mutations** (the claim becomes easier to satisfy). Should always pass; failure indicates a verifier bug. Not informative for spec quality.

The signal is in strengthening-mutation *survival*: a high survival rate means the spec is leaving slack.

### 8.2 Mutation operators

The v0 operator set (`crates/specq/src/mutate/operators.rs`):

**Comparison strengthening.** `<=` → `<`, `<` → `<=` with strict bound, `>=` → `>`, etc. Strengthens by tightening the comparison.

**Conjunct addition.** `A ∧ B` → `A ∧ B ∧ C` where C is drawn from the umbrella's predicate vocabulary. Strengthens by requiring more.

**Disjunct deletion.** `A ∨ B` → `A`. Strengthens by removing a possibility.

**Postcondition tightening.** `ensures Q` → `ensures Q ∧ R` for plausible R. Strengthens by claiming more about the output.

**Precondition strengthening.** `requires P` → `requires P ∧ Q`. Strengthens the requirement on the caller; equivalent to weakening the spec's claim (the spec promises behavior for fewer inputs). This operator is informative in the *weakening* direction: if survival rate is low (the spec doesn't tolerate the strengthened precondition), the spec's precondition is *not* over-restrictive.

**Quantifier tightening.** `for-all x: T, P(x)` → `for-all x: {x: T | C}, P(x)`. Restricts the quantified domain; informative for understanding which range of inputs is bearing weight.

**Variable swap in binary relations.** `f(x, y)` → `f(y, x)`. Tests whether directionality is bearing weight.

The operator set is extensible. New operators are added when patterns emerge.

### 8.3 Equivalent-mutant detection

Some mutations produce semantically equivalent formulas (`A ∧ B` → `B ∧ A`). These are uninformative and excluded from the kill-rate calculation. Detection is via:

- Syntactic normalization (sort conjuncts, canonicalize literals).
- AST equivalence check after normalization.

Equivalent-mutant detection is undecidable in general; the v0 detector is heuristic and may have false positives (treating semantically distinct mutants as equivalent) and false negatives. Both are tracked.

### 8.4 The report

```
Mutation report for examples/02-ledger/ledger.lm

Claim: conservation
  Total mutations:    14
  Equivalent:         2 (excluded)
  Strengthening:      8
    Killed:           7  (87%)
    Survived:         1
      Mutation: postcondition tightening
      Mutation result: also verifies
      Interpretation: implementation maintains a tighter invariant
      Suggestion: consider adding this to the umbrella's proves
  Weakening:          4
    Survived:         4  (100%, as expected)
  Timeout:            0

Claim: no_overdrafts
  Total mutations:    11
  Equivalent:         1
  Strengthening:      6
    Killed:           2  (33%)  
    Survived:         4
      ⚠ WEAK SPEC: 67% strengthening survival rate
      Survived mutations:
        - quantifier tightening on Account.balance
        - conjunct addition: from'.open
        - postcondition tightening: result type
        - comparison: >= 0 → > 0
      The implementation establishes substantially more than the
      umbrella claims. Recommended: strengthen the claim.
  Weakening:          4
    Survived:         4
  Timeout:            0

  FINDING [spec-slack]:
    no_overdrafts has 67% strengthening-mutation survival.
    The implementation maintains stronger invariants than the spec.
```

The report is both per-claim (detailed) and aggregate (summary). Aggregate kill rates are tracked over commit history (similar to gap drift).

### 8.5 Cost

Each mutation requires a full Dafny invocation. For an umbrella with 10 claims and 10 mutations per claim, that's 100 Dafny calls. At seconds-per-call, this is minutes to hours.

Mitigations:
- **Caching.** Cache by `(claim_hash, mutation, dafny_version, z3_version)`. Re-runs of unchanged claims hit the cache.
- **Sampling.** Run a random subset on every push; full mutation runs on PR review or release.
- **Parallelization.** Mutations are independent; the test runner parallelizes them across available cores.

In CI:

```yaml
# every push: sample
- run: loom specq --sample-rate 0.2

# PR check: full
- run: loom specq --full
```

---

## 9. Aggregate quality metric

A summary metric:

```
Spec quality for examples/02-ledger/ledger.lm

  Grammar:                clean (no banned forms)
  Cross-register:         complete (all rules satisfied)
  Anti-patterns:          0 findings
  Domain engagement:      89% precondition saturation (good)
                          2/3 variant coverage (one gap)
  Gap discipline:         1 gap, 8% domain restriction, stable
  Mutation kill rate:     83% (strengthening, aggregate)

  Overall: B+
  
  Highest-priority improvement:
    no_overdrafts (33% strengthening kill rate, see above)
```

The letter grade is a rough summary, not a precise number. The detailed findings are what drives action.

The grade is not a number to optimize directly. The threat-model essay (`process-gates-and-value-gates.md`) makes the structural point that an LLM that optimizes against the metric will produce decoratively-complex specs that score well but are still misaligned with intent. The metric is one signal in a layered defense; reviewer judgment is the other.

---

## 10. Workflow

```bash
# during development
loom check umbrella.lm                  # structural
loom verify umbrella.lm                 # baseline
loom specq umbrella.lm --quick          # cheap defenses only

# before commit
loom specq umbrella.lm                  # standard run

# before release
loom specq umbrella.lm --full           # full mutation
loom specq umbrella.lm --history        # trend analysis
```

`specq` outputs to stdout by default; `--output report.md` writes to a file (used in CI).

---

## 11. Limitations

### 11.1 Mutation testing doesn't catch everything

The companion paper §9.1 enumerates: formally strong but misaligned with intent; coordinated weakening across claims; underspecification; domain-specific patterns. None of these are caught.

The right response is layered defense plus reviewer judgment. `specq` raises the floor; it does not eliminate the human.

### 11.2 Verifier coupling

`specq` is coupled to the verifier's capabilities. A weak verifier dominated by timeouts produces uninformative mutation kill rates (everything times out, nothing is killed or survived). v0 uses Dafny; results vary across verifiers.

### 11.3 The metric is itself an attack surface

If the LLM learns to optimize for mutation kill rate, it will produce specs that score well without being meaningful. The mitigations from §9 of the companion paper apply: rotate operators, add new operators as patterns emerge, treat the rate as one signal among many.

---

## 12. References

- [`docs/research/spec-quality-under-llm-authorship.md`](../research/spec-quality-under-llm-authorship.md) — the full research treatment.
- [`docs/research/process-gates-and-value-gates.md`](../research/process-gates-and-value-gates.md) — the structural argument grounding the defenses.
- [`docs/bidirectional-refinement.md`](bidirectional-refinement.md) — the related use of mutation testing for the gap report.
- [`docs/claims-reference.md`](claims-reference.md) — the registers `specq` evaluates.
