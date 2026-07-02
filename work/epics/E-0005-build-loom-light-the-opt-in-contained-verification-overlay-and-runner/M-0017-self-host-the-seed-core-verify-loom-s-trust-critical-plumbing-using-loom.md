---
id: M-0017
title: 'Self-host the seed core: verify loom''s trust-critical plumbing using loom'
status: done
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
      status: met
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
- Model↔source binding reuses the gap-report `subject` fields (`repo` / `ref` / `path` / `symbol`) frozen in M-0016; self-host is the first real exercise of that binding on loom's own code — it carries a symbol-based pin (not a line number) end to end into the report. Honest scope: nothing yet *resolves* the symbol against source, and the ref (`v0.1.0`) is a pre-release moving target, so the durable anchor is the symbol by convention, not a released tag. Symbol resolution / tag-pinning is a later concern.
- Reproducibility (G1) and schema equivalence (D2) stay as Rust tests. If later wanted, a G1 "same-inputs → byte-identical report" property is better self-hosted as a runner-level check than as a Dafny model — noted, not scoped here.

## Out of scope

- Verifying loom's Rust *implementation* directly — there is no extraction (ADR-0017).
- The gap-report equivalence (D2) and reproducibility (G1) properties as Dafny umbrellas — they stay as the Rust tests M-0016 carries.
- Installing the aiwf seed overlay into the real aiwf repo — a separate deployment concern.
- The second substrate (M-0018) and anything beyond the seed Dafny runner.

## Dependencies

- **M-0016** — the five frozen contracts, the runner, and the Dafny backend the self-host overlay runs on. (`depends_on` recorded.)

## Work log

The self-host overlay, the three Dafny models, and the `subject` wiring landed as one `feat`
implementation commit (`216bb6e`); the wrap review added two corrective edits (folded into the
same commit — see Reviewer notes). Per-AC TDD phase timelines are in `aiwf history M-0017/AC-<N>`.

- **AC-1 — overlay contained + opt-in** (met): repo-root `loom/` overlay (three properties) +
  a `cargo loom` alias (`.cargo/config.toml`) off the default cargo graph; not a workspace
  member, removable without trace. `216bb6e`.
- **AC-2 — dispatch totality** (met): `loom/dispatch-totality/model.dfy` proves every substrate
  routes to a backend that *verifies* (`Verifies(Dispatch(s))` ∀ s); pinned to
  `loom::backend::dispatch`. Runner reports `proved`. `216bb6e`.
- **AC-3 — parser totality** (met): `loom/parser-totality/model.dfy` proves parse is a total
  partition (Ok / Missing / Duplicate / Unknown), each branch reachable; pinned to
  `loom::umbrella::parse`. `proved`. `216bb6e`.
- **AC-4 — atomic crash-safety** (met): `loom/atomic-crash-safety/model.dfy` proves the dest is
  fully-old/absent or fully-new at every crash phase, never torn; pinned to `loom::atomic`.
  `proved`. `216bb6e`.
- **AC-5 — frozen contracts hold from the inside** (met): the umbrella parser grew additive
  `subject-*` fields and the runner populates `GapReport.subject` — with **zero change** to any
  of the five frozen contracts (the `schema_is_frozen` test stays byte-identical). Each
  self-host report records its pinned symbol + version. `216bb6e`.

## Decisions made during implementation

No new decision entities were opened. The milestone realizes decisions already on record —
ADR-0017 (loom generates no target code / no Rust→Dafny extraction; a model mirrors its subject
by reference), the M-0016 frozen contracts, and this spec's Context scope-narrowing (self-host
the modelable three; keep D2/G1 as the Rust tests M-0016 carries). Three implementation choices,
all within the ACs' stated latitude, are recorded here rather than as separate entities:

- **Overlay at repo-root `loom/`** — mirrors `examples/seed-downstream/loom/`, and being a
  non-member sibling of `crates/` keeps it off the cargo build graph by construction (AC-1
  containment holds structurally).
- **Opt-in entry is a cargo alias (`cargo loom`)**, not a Makefile — idiomatic for this Rust
  workspace; AC-1 explicitly permits either.
- **Subject declared via additive `subject-*` umbrella fields** — reuses the frozen `Subject`
  schema (no Contract 2 change) and the umbrella's existing line-scan format (no Contract 3
  change); partial/duplicate declarations are typed rejections (B2), never a half-populated
  subject.

## Validation

- `cargo fmt --check` clean; `cargo clippy --all-targets -- -D warnings` clean; `cargo build` green.
- `cargo test`: **62 tests pass** (M-0016's 47 + 15 for M-0017: 9 `self_host` integration + 6
  new `umbrella` subject-parsing unit tests), incl. the three Dafny-backed self-host verdict
  tests (Dafny 4.9.0 on PATH). M-0016's `schema_is_frozen` and `verify_seed` reproducibility
  pass unchanged — the frozen contracts did not move.
- All three models `dafny verify --cores:1` clean (1 / 2 / 2 obligations). Each was
  vacuity-checked at review by falsifying a postcondition and confirming Dafny rejects it.
- End-to-end: `cargo loom` verifies the overlay to 3× `proved`, each report carrying the pinned
  `subject` (repo/ref/path/symbol) and a full audit trail.

## Deferrals

None. No work was punted — the milestone's scope was the modelable three (D2/G1 stay as the
existing M-0016 Rust tests, recorded in this spec's Out of scope, not a deferral). The M-0016
determinism gap (G-0009, wall-clock vs `--resource-limit`) covers the backend generally and is
not re-opened; the self-host models are trivial and verify deterministically under `--cores:1`.

## Reviewer notes

Two-lens wrap review ran over the full (uncommitted) change-set from fresh-context reviewers.

- **Code lens → APPROVE.** The headline risk — a vacuous "proves nothing" model — was measured
  false for all three lowerings (a falsified `ensures` was rejected by Dafny in every case).
  Zero change to the five frozen contracts confirmed (schema byte-identical; M-0016 tests green);
  the `unique_field` refactor is behavior-preserving on the frozen substrate-parse edges. Two
  non-blocking nits fixed in this milestone: (1) a Rust test now pins the "duplicate detected
  before the empty-value filter" ordering directly (was proved only by reference in the Dafny
  model); (2) — see the design lens's C1 point below.
- **Design lens → KEEP** on the subject-binding data-model. The flat additive `subject-*` fields
  reuse the frozen `Subject` and validate their boundary at the right size; AC-5's core claim is
  genuinely met and mechanically pinned. One real C1 drift fixed: `atomic-crash-safety`'s prose
  header named `write_atomic` while the canonical machine field (per AC-4) is `loom::atomic` —
  the prose now defers to the `subject-*` fields as the source of truth.
- **Documented scope boundaries (deliberate, not defects):** (a) the `parser-totality` model is
  pinned to the v0.1.0 substrate-decision core and does **not** model the `subject-*` parsing this
  milestone added to the same `parse` — that extension is total by Rust construction and covered
  by six new Rust unit tests. (b) The subject pin is a *reference* pin: nothing resolves the
  symbol against source and `v0.1.0` is a pre-release moving ref, so the durable anchor is the
  symbol by convention (the design note was corrected to say so). A `proved` self-host report is
  a claim about the Dafny lowering in `audit.inputs`, **not** a claim that the Rust `subject` is
  verified directly (ADR-0017) — carried by the audit trail and the README honesty note.
