# Gap Report — milestone status-transition FSM

Loom loop 1 · `M-0014` / `E-0004` · **the audit trail (E3) of one whole-loop turn.**

- **Verifier:** Dafny 4.9.0 / Z3 · reproducible from [`milestone-fsm.dfy`](milestone-fsm.dfy) (G1)
- **Model:** real aiwf `transition.go` @ v0.20.0 (impl-modeler subagent, blind to the claims)
- **Claims:** the human's Intent + Examples ([`umbrella.md`](umbrella.md), umbrella-author subagent, blind to the code)
- **Verifier result:** `6 verified, 3 errors` · **Examples matched: 7 / 8**

## (A) Claimed and proved — intent matches the code

C3 `done` terminal · C4 `cancelled` terminal · C5 draft always cancellable · C6 in_progress
always cancellable · C7 a draft with an AC may start · C8 an in_progress milestone with all ACs
met may finish.

## (B) Claimed but NOT proved — intent and code **disagree**

- **C1 — "no ACs ⇒ can't start (`draft → in_progress`)."** *Counterexample found.* The real code
  permits `draft → in_progress` **unconditionally** — no AC guard on that edge (FSM table
  `"draft": {"in_progress", "cancelled"}`, `transition.go:24`; `ValidateTransition` is pure table
  lookup). This is the one of eight examples the code disagrees with (**ex2**).
  - **TRIPLE-CONFIRMED:** (1) source — the FSM table has no AC guard; (2) formal — `dafny verify`
    counterexample; (3) **behavioral** — a throwaway no-AC milestone was promoted to `in_progress`
    by the live binary: `aiwf promote M-0015 draft -> in_progress` (probe reset away afterward).
  - **Verdict:** a real discrepancy. aiwf does **not** enforce "a milestone needs an acceptance
    criterion before work starts," contrary to a reasonable operator expectation. Candidate aiwf
    gap (see Follow-ups).
- **C2 — "ACs not all met ⇒ can't finish."** Holds for a milestone that *has* an AC (ex5 ✓). But a
  milestone with **no ACs** can go `in_progress → done` (the code's real guard is "no *open* AC,"
  and a no-AC milestone has none). The concrete examples never reached this state — because the
  intent assumes you can't *be* in_progress without ACs. **The universal claim caught a path the
  examples couldn't.** Partly entangled with the interface limitation below; the underlying reality
  (a no-AC milestone flows `draft → in_progress → done` unguarded) is real and follows from C1.

## (C) What the code does that the intent didn't address

- **"All met" is really "no *open* AC"** — a `deferred`/`cancelled` AC (not `met`) still lets a
  milestone finish.
- **`--force` relaxes direction but not the AC gate.**
- **The two cancel surfaces diverge — and one *refutes* the intent.** `aiwf promote M cancelled`
  allows cancel with any AC state (what the model encodes, so C5/C6 verified). But `aiwf cancel M`
  (the cancel *verb*) **blocks** cancelling a milestone with an `open` AC (`promote.go:246-253`) —
  so the operator's "cancellable with ACs unmet" is **false on that surface**. A second, smaller
  intent-vs-code gap the single-surface model masked (surfaced by the independent review).

## Fidelity (honesty about the model)

The `(hasAC, allACsMet)` interface cannot perfectly express the code's real "no open AC" guard, so
C2's no-AC counterexample is partly a modeling artifact. The impl-modeler surfaced this with
`file:line` evidence rather than hiding it (see `umbrella.md` § Model fidelity notes).

## The four observations (M-0014 / AC-4)

- **Tractability** — verified instantly; zero Z3 timeouts. But a discrete table is the *easy* end;
  canonicalize's strings/recursion is the real stress, still ahead.
- **Faithfulness** — high. The impl-modeler grounded every claim in `file:line` and volunteered
  where the interface is lossy; the umbrella-author flagged its own silences (draft→done,
  cancelling an all-met milestone, what "met" means). Lossiness surfaced, not hidden.
- **Value** — **yes.** Surfaced a real, non-obvious discrepancy (C1) that otherwise needs reading
  275 lines of Go across several files; the universal claim caught a path (C2) the examples missed.
- **Effort** — the human wrote one paragraph of intent + confirmed 8 examples + read this report in
  English. **Zero Dafny, zero Go read.** The formal layer was entirely two blind subagents + one
  verifier run + one behavioral probe.

## Follow-ups

- **Candidate aiwf gap:** "a milestone can be promoted to `in_progress` with no acceptance
  criteria" — triple-confirmed here. To be filed against aiwf if the operator judges it unintended.
