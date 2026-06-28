---
id: G-0005
title: Gate strength population to valid specs; unit-test probe_spec trichotomy
status: addressed
discovered_in: M-0006
addressed_by:
    - M-0008
---

## What's missing

Two related strength-gate hardenings:

1. **Gate the strength entailment population to valid specs.** `probe_spec` measures every
   spec that extracts and *resolves*; the resolve guard (`entails(..., "true")`) only
   excludes specs that fail to type-check, not an ex-falso-contradictory over-claim (under
   ex falso `true` is provable, so such a spec would entail every obligation and inflate
   the rates toward the null). Gate the entailment-rate population to the kill-rate-valid
   specs so over-claims cannot leak into the strength signal.
2. **Make `probe_spec`'s timeout routing unit-testable.** The §5 trichotomy (Verified →
   `counts`/`definite`; Failed → `definite`; Timeout → `obligation_timeouts`, dropped from
   the denominator) has no deterministic committed test — a real Z3 timeout can't be forced
   without a wall-clock dependency. Give `probe_spec` an injectable outcome closure (as
   `classify_ladder` takes a `probe` closure) so the routing is pinned without Dafny.

## Why it matters

Item 1 was **dormant** in the E-0002 run — the fsm over-claims surfaced as probe errors
(unresolvable when assumed), so the strength population happened to equal the valid
population and the NO-GO is unaffected. But the design does not *guarantee* it, and the
successor study's pre-registration should close it explicitly (over-claiming is a live
signal there). Item 2 closes a branch-coverage gap on a load-bearing measure.
