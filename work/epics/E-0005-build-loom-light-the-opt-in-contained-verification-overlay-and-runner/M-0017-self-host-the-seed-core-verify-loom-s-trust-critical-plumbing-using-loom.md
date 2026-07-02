---
id: M-0017
title: 'Self-host the seed core: verify loom''s trust-critical plumbing using loom'
status: in_progress
parent: E-0005
depends_on:
    - M-0016
tdd: required
acs:
    - id: AC-1
      title: Self-host overlay contained and opt-in
      status: met
      tdd_phase: done
    - id: AC-2
      title: Substrate-dispatch totality self-hosts and verifies
      status: met
      tdd_phase: done
    - id: AC-3
      title: Umbrella-parser totality self-hosts and verifies
      status: met
      tdd_phase: done
    - id: AC-4
      title: Atomic-write crash-safety self-hosts and verifies
      status: met
      tdd_phase: done
    - id: AC-5
      title: Frozen contracts hold from the inside
      status: open
      tdd_phase: done
---
## Goal

Turn loom on loom: verify loom's own trust-critical plumbing through loom itself. Hand-author a self-host overlay in this repo whose umbrellas claim loom's load-bearing invariants and whose Dafny lowerings prove them, run under the M-0016 runner. This is the dogfood **and** the risk mitigation the epic pulls early (E-0005 §60): stress-test the five frozen contracts *from the inside* before the tree grows a second substrate (M-0018), so any contract weakness surfaces now — as a fix behind a contract — rather than as a rewrite later.

## Context

M-0016 froze the five contracts and proved loom's plumbing with **Rust unit tests** (`equivalence.rs`, `durability.rs`, `umbrella_totality.rs`, the backend tests). This milestone re-expresses the subset those tests cover that a *formal model* genuinely strengthens — as loom umbrellas + Dafny lowerings — and verifies them with `loom verify`. Its value is **not** re-proving the plumbing (the Rust tests already do that); it is proving the frozen contracts are expressive enough to carry loom's own properties from the inside, before the tree grows.

**Scope note — a deliberate narrowing of E-0005 §24/§68's "all five".** loom verifies Dafny lowerings, not Rust directly (no Rust→Dafny extraction — ADR-0017). Three of the five load-bearing properties are naturally Dafny-modelable and self-host with real proof value; two — gap-report writer↔reader **equivalence** (D2) and **reproducibility** (G1) — are serde / meta facts whose honest home stays the Rust tests M-0016 already carries. A Dafny model of those would prove a toy round-trip, not loom. This milestone self-hosts the modelable three and leaves the other two as their existing Rust tests. A self-host Dafny lowering models the invariant and is pinned to its Rust subject by reference; it does not claim to verify the Rust implementation directly.

## Acceptance criteria

### AC-1 — Self-host overlay contained and opt-in

A `loom/` overlay in this repo carries loom's self-host properties; an opt-in target (the same `make loom` shape as M-0016, or a cargo alias) runs `loom verify` over it and is off the default build/test graph. Removing the overlay leaves loom's own pipeline byte-identical — containment, mirrored onto loom-on-loom.

### AC-2 — Substrate-dispatch totality self-hosts and verifies

An umbrella + Dafny lowering models substrate→backend routing as total (every substrate maps to exactly one backend; none silently unverified) and verifies `proved` via `loom verify`. The umbrella pins the claim to `loom::backend::dispatch` by symbol + pinned version.

### AC-3 — Umbrella-parser totality self-hosts and verifies

An umbrella + Dafny lowering models umbrella parsing as a total function (every input yields a parsed umbrella or a typed rejection — never a panic) and verifies `proved`. Pinned to `loom::umbrella::parse`.

### AC-4 — Atomic-write crash-safety self-hosts and verifies

An umbrella + Dafny lowering models the temp-write→rename protocol and proves no observable partial or torn report (a crash leaves the destination fully-old or absent). Verifies `proved`. Pinned to `loom::atomic`.

### AC-5 — Frozen contracts hold from the inside

Expressing loom's own three properties required **no change** to any of the five frozen contracts (overlay boundary, gap-report schema, umbrella format, runner/backend seam, property independence), and each self-host report's `subject` records the pinned Rust symbol + version its model stands for. Demonstrable: the self-host overlay lands with zero diff to the contract surfaces, and `loom verify` populates `subject` for each property. A property that *cannot* be expressed without editing a frozen contract is recorded as a gap — that discovery is the milestone's point.

## Constraints

- **Opt-in, contained** — same bar as M-0016: the self-host overlay is off the default graph and removable without trace.
- **No contract drift** — if a property cannot be expressed without changing a frozen contract, that is a finding (a gap), not a licence to edit the contract. Discovering such a case now, cheaply, is the reason the milestone is pulled early.
- **Honest models** — each Dafny lowering models the invariant and is pinned to the Rust subject by reference; it is not claimed to verify the Rust implementation directly.
- **`tdd: required`** — each property AC runs red (umbrella authored, model absent or failing) → green (model verifies `proved`) → done.

## Design notes

- The three models mirror the M-0016 aiwf seed's shape (a small Dafny state-machine / totality proof), so they exercise the same runner path the aiwf seed did — a from-the-inside test of the backend seam under loom's own properties.
- Model↔source binding reuses the gap-report `subject` fields (`repo` / `ref` / `path` / `symbol`) frozen in M-0016; self-host is the first real exercise of that binding on loom's own code, so it also validates the symbol-not-line-number pinning discipline.
- Reproducibility (G1) and schema equivalence (D2) stay as Rust tests. If later wanted, a G1 "same-inputs → byte-identical report" property is better self-hosted as a runner-level check than as a Dafny model — noted, not scoped here.

## Out of scope

- Verifying loom's Rust *implementation* directly — there is no extraction (ADR-0017).
- The gap-report equivalence (D2) and reproducibility (G1) properties as Dafny umbrellas — they stay as the Rust tests M-0016 carries.
- Installing the aiwf seed overlay into the real aiwf repo — a separate deployment concern.
- The second substrate (M-0018) and anything beyond the seed Dafny runner.

## Dependencies

- **M-0016** — the five frozen contracts, the runner, and the Dafny backend the self-host overlay runs on. (`depends_on` recorded.)

## Work log

_(filled during implementation)_

## Decisions made during implementation

_(filled during implementation)_

## Validation

_(filled at wrap)_

## Deferrals

_(filled at wrap)_

## Reviewer notes

_(filled at wrap)_
