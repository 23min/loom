---
id: G-0006
title: reallocate validity gate rejects correct-but-not-auto-provable specs
status: open
discovered_in: M-0011
---
## What's missing

The validity gate (`validate_spec` → empty-body `dafny verify` of the candidate's
`ensures` against the reference impl) treats a spec as valid only if Dafny can
**auto-prove** it. Correct reallocation specs naturally include clauses that are TRUE of the
reference impl but not auto-provable: the rename existential `HasId(t', newId)` (needs a
witness) and iff-characterizations like `t'[i].id == newId <==> t[i].id == oldId` (need the
`!HasId(t, newId)` precondition reasoning per element). Such specs are marked invalid.

## Why it matters

The §6 over-claiming dimension is `1 − valid/extracted`. With the current gate, "invalid"
conflates over-claiming (specs too strong for the correct impl) with "correct but not
auto-provable", and it systematically penalizes the **thorough** specs the disinterested arm
writes. A smoke test on `reallocate` (M-0011, throwaway N=1 × 3 models) returned 6/6 invalid,
including a demonstrably correct disinterested `opus-4.8` spec — confirming both §6 dimensions
would be confounded by a Dafny-automation artifact rather than measuring the pre-registered
effect. Addressed by `M-0012` (validity-gate hardening) per `D-0003`.
