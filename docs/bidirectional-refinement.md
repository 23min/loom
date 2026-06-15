# Bidirectional refinement

> **Status:** draft
> **Audience:** contributors and users who want to understand what the gap report is and why it is load-bearing
> **Companion:** [`docs/research/verifiable-umbrella-paper-v2.md`](research/verifiable-umbrella-paper-v2.md) §4.4 (the original formulation)

---

## 1. The problem this solves

Conventional verification has one direction. The specification states properties; the verifier checks the implementation against the specification; the result is *verified* or *not verified*. The implementation is the unknown; the specification is treated as ground truth.

This works when the specification is reliably trustworthy. It breaks down when the specification is itself an artifact under construction — when the specification is *also* being drafted, revised, weakened, or strengthened during the work. The conventional one-directional flow gives no signal that the specification is the wrong artifact to be measuring against.

Bidirectional refinement adds the missing direction. It treats the specification and the implementation as *two artifacts standing in a relation*, both fallible, both subject to revision. The verification produces not a single verdict but a *gap report* — a structured statement of what the implementation establishes, what the specification claims, and where the two disagree.

The discipline is bidirectional in this specific sense: obligations flow downward from specification to implementation (the normal direction), and *evidence* flows upward from implementation to specification (the new direction). Where the two flows disagree, the gap report records the disagreement.

---

## 2. The three categories

For any (umbrella, implementation) pair, every property of interest falls into one of three categories.

**(A) Claimed and proved.** The umbrella states a property. The verifier discharges it. The property is part of the verified guarantee of the (umbrella, implementation) pair. This is the conventional case; the verifier reports `verified`.

**(B) Claimed but not proved.** The umbrella states a property. The verifier does not discharge it — because of a timeout, a verifier limitation, an explicit gap, or because the implementation genuinely does not establish it. The umbrella's claim is not backed by mechanical evidence. The gap report records *why* the claim is unproved, distinguishing the failure modes:

- *Timeout.* The SMT solver did not return within budget. The claim may be true; we do not know.
- *Verifier limitation.* The verifier cannot express what would need to be proved. The claim may be true; the verifier is incomplete.
- *Explicit gap.* The umbrella marks the claim as a known gap, often with a restriction (`gap conservation: when concurrent_transfers(T)`). The claim is admitted to be partial.
- *Failed.* The verifier produced a counterexample. The implementation provably does not establish the claim. This is a true verification failure and a release blocker.

**(C) Proved but not claimed.** The implementation establishes a property the umbrella does not credit. The verifier has demonstrated something the umbrella's authors did not (or have not yet) articulated. This is the *new* category — the one one-directional verification does not produce.

Category (C) is the upward flow of evidence. It tells the umbrella's authors that the implementation is stronger than they have written down. The right response is usually to add the property to the umbrella; sometimes the right response is to recognize that the implementation was accidentally stronger than needed and could be simplified.

---

## 3. The shape of the gap report

The gap report is structured. Tooling (`loom verify`) emits both a JSON form (for CI and machine consumption) and a Markdown form (for humans).

The Markdown form looks like:

```
Gap Report for examples/02-ledger/ledger.lm
Verified against Dafny 4.4.0 with Z3 4.13

Summary:
  Claimed and proved (A):     5
  Claimed and unproved (B):   1
  Proved but not claimed (C): 2

(A) Claimed and proved
  conservation
    sum(L before T) = sum(L after T)
    Verified for all values of L: Ledger, T: Transfer
    SMT discharge time: 0.4s

  no_overdrafts
    for-all account A, balance(A) >= 0
    Verified
    SMT discharge time: 0.2s

  [... 3 more ...]

(B) Claimed and unproved
  fairness
    for-all transfers T1 T2, ordered(T1, T2) implies completed_before(T1, T2)
    Status: explicit gap (gap fairness: when concurrent)
    Restricted to non-concurrent transfers
    Open issue: address concurrency in v0.2

(C) Proved but not claimed
  Account totals are non-negative on all reachable states
    The implementation maintains a stronger invariant than the umbrella claims.
    Suggestion: add this as a `proves` clause, or simplify the implementation
    if the stronger invariant is incidental.

  Transfer operation is associative on commuting transfers
    The implementation has an algebraic structure the umbrella does not credit.
    Suggestion: review whether this is intentional architecture.
```

The JSON form has the same content with a stable schema (ADR-0007), suitable for diffing across runs and for CI integration.

---

## 4. Where category (C) findings come from

Category (A) and (B) come for free: the verifier reports them as part of normal operation. Category (C) requires additional work — specifically, mutation testing on claims.

The mechanism: for each claim the verifier proves, the mutation engine produces *strengthening* mutations (a tightened comparison, an added conjunct, a tightened type bound). If the strengthened claim still verifies, the implementation establishes more than the original claim. The strengthening that survives identifies the direction of the additional strength.

This is the same machinery the spec quality reporter (`specq`) uses to detect weak specifications — see [`docs/spec-quality.md`](spec-quality.md). The bidirectional gap report and the spec quality report are facets of the same technique: mutation testing on claims, run in two directions for two purposes. Strengthening mutations that survive populate category (C) of the gap report. Weakening mutations that survive (which should always survive, per §6.4 of the spec quality paper) are diagnostic noise in the gap-report direction and informative in the quality-report direction. Strengthening mutations that survive are the spec slack the implementation is establishing past the umbrella's claims.

The mechanical cost is non-trivial — each mutation is a verifier re-run — and the implementation is therefore expected to populate category (C) at gate points (PR review, release) rather than every build. Categories (A) and (B) are populated on every build.

---

## 5. What the discipline asks of authors

The bidirectional discipline asks the human–LLM team to *engage with all three categories* rather than only category (A).

**Category (B) requires triage.** When a claim is unproved, the right question is which subcase: timeout, verifier limitation, gap, or failure. Each has a different response. Timeouts may indicate a need to refine the proof obligation. Verifier limitations may indicate a need for a richer assertion or for the claim to be restated. Gaps require either acceptance (the claim is admitted to be partial) or work (the claim must be made provable). Failures are blockers and must be addressed before the umbrella ships.

The discipline asks for these distinctions to be made explicit. An umbrella with many category-(B) entries marked `Status: timeout` is not the same as one with many entries marked `Status: explicit gap`; the former is a tractability problem, the latter is a coverage problem. Reviewers should ask which kind.

**Category (C) requires reflection.** When the implementation establishes more than the umbrella claims, the right response is not always "add the property to the umbrella." Sometimes the implementation is incidentally stronger because of an implementation detail that should not be enshrined as a guarantee. Sometimes the implementation is intentionally stronger and the umbrella is incomplete. The discipline asks for the choice to be made explicitly, not by default.

A category-(C) finding that is converted to a claim becomes part of the umbrella's verified guarantees. A category-(C) finding that is *not* converted is recorded as known, but the umbrella does not promise it; future implementations are free to weaken to the umbrella's level.

---

## 6. What this is not

Bidirectional refinement is not:

- **A correctness proof of the specification.** Category (C) findings tell us the implementation is stronger than the specification claims. They do not tell us the specification is *correct* — that it captures the human's intent. Intent alignment requires human review of the specification's claims, and is supported by but not replaced by the gap report.

- **A safety net for arbitrary implementations.** The gap report is meaningful only when the verifier successfully discharges most claims. An umbrella with many category-(B) failures has a gap report dominated by the failures; category (C) is uninformative if (A) is sparse.

- **A substitute for testing.** Examples in the `shows` register are runnable tests. The bidirectional discipline supplements example-based testing with property-based verification; it does not replace examples.

- **A substitute for `specq`.** The gap report tells the team what the implementation establishes versus what the umbrella claims. It does not tell the team whether the umbrella's claims are *strong enough to be meaningful*. That question is `specq`'s job, using the same mutation-testing machinery applied to the umbrella in isolation.

---

## 7. The relationship to the cheating attractor

The companion essay [`docs/research/process-gates-and-value-gates.md`](research/process-gates-and-value-gates.md) makes the structural point that LLM-mediated mechanical checks are vulnerable to gaming wherever the LLM authors the gate's definitional content. The umbrella's claims are exactly such content.

The bidirectional gap report does not solve the cheating attractor. It does two specific things:

**(i) It makes weakness visible.** If the LLM's umbrella under-claims, category (C) reveals the under-claiming: the implementation establishes more than the umbrella credits. The human reviewing the gap report sees what was elided.

**(ii) It constrains weakening over time.** If an umbrella's category-(A) shrinks and category-(B) grows from one revision to the next, the umbrella is regressing in strength. Tracking this is mechanical; reviewers can ask why.

These help. They do not eliminate. The full defense against the cheating attractor at the umbrella layer is the layered defense in [`docs/spec-quality.md`](spec-quality.md); the gap report is one layer.

---

## 8. Implementation notes for `loom-verify`

The gap report is produced by `crates/loom-verify` from three inputs:

1. The umbrella AST (from `loom-syntax`).
2. The verifier's report (from `loom-compile-dafny` and the Dafny subprocess).
3. (For category C) The output of mutation testing on claims (from `specq`).

The first two are available on every `loom verify` run; the third is enabled by `--with-gap-discovery` or `--full`, which triggers mutation testing.

Diagnostic location information must be preserved through the pipeline. A category-(B) finding without source location is hard to act on; the reviewer must locate the claim in the umbrella, then understand why it did not verify. Every gap-report entry should have a source span referencing the umbrella, and where applicable, the verifier's output excerpt.

The Markdown report is intended to be diff-friendly: stable ordering of categories, stable ordering within categories, no incidental whitespace changes between runs with the same inputs. This is so that gap reports can be committed to the repository and inspected in PR diffs.

---

## 9. Future directions

Items deferred to post-v0:

- **Cross-umbrella gap reports.** When a parent umbrella claims `customer_funds_safe ⇐ child_A.proves(X) ∧ child_B.proves(Y)`, the parent's effective guarantee depends on the child claims' strength. Computing effective guarantees through `summarizes` relations is a future direction.

- **Gap-rate trending.** Tracking category-(B) and category-(C) counts over time, per umbrella, to surface regressions and improvements as a project evolves.

- **Differential gap reports.** Comparing two versions of an umbrella to see which claims moved between categories.

- **Suggested patches.** Category-(C) findings could include an auto-generated `proves` clause that the human can accept or modify. This is LLM-assisted but the proposal itself can be mechanical.

---

## 10. References

- Architecture paper, §4.4 — original formulation of bidirectional refinement.
- Spec quality paper, §6 — mutation testing on claims, the machinery this discipline depends on.
- [`docs/spec-quality.md`](spec-quality.md) — `specq` and the layered defense.
- [`docs/verification-internals.md`](verification-internals.md) — how claims are translated to Dafny.
