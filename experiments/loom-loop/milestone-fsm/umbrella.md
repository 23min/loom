# Umbrella — milestone status-transition FSM

*A loom umbrella — the whole document below; `module milestone_fsm` is its formal spine (five registers, [language-reference.md](../../../docs/reference/language-reference.md) §4). Formal bodies are Dafny pending the `.lm` toolchain — verified via [`milestone-fsm.dfy`](milestone-fsm.dfy); result + provenance in [`gap-report.md`](gap-report.md).*

## Intent

A milestone is in one of four statuses — `draft`, `in_progress`, `done`, `cancelled` — starting at
`draft`. It may go `draft → in_progress` only if it has at least one acceptance criterion (no ACs ⇒
cannot start). It may go `in_progress → done` only if all of its ACs are met (any unmet ⇒ cannot
finish). It may be cancelled — including with no ACs, and with ACs unmet. `done` and `cancelled`
are terminal.

## Umbrella — `module milestone_fsm`

```loom
module milestone_fsm {

  knows {
    Status :: Draft | InProgress | Done | Cancelled
    // hasAC — has >=1 acceptance criterion;  allACsMet — every AC is met
  }

  relates {
    allowed(from: Status, to: Status, hasAC: bool, allACsMet: bool) -> bool
  }

  shows {
    has_ACs_start:       allowed(Draft, InProgress, true, true)      -> true
    no_ACs_start:        allowed(Draft, InProgress, false, true)     -> false
    no_ACs_cancel:       allowed(Draft, Cancelled, false, true)      -> true
    all_met_finish:      allowed(InProgress, Done, true, true)       -> true
    unmet_AC_finish:     allowed(InProgress, Done, true, false)      -> false
    unmet_AC_cancel:     allowed(InProgress, Cancelled, true, false) -> true
    done_terminal:       allowed(Done, InProgress, true, true)       -> false
    cancelled_terminal:  allowed(Cancelled, InProgress, true, true)  -> false
  }

  does {
    // modeled from real aiwf transition.go @ v0.20.0 (KindMilestone FSM table)
    allowed(from: Status, to: Status, hasAC: bool, allACsMet: bool) -> bool {
      (from = Draft      and to = InProgress)                        or
      (from = Draft      and to = Cancelled)                         or
      (from = InProgress and to = Cancelled)                         or
      (from = InProgress and to = Done and (not hasAC or allACsMet))
    }
  }

  proves {
    c1_no_ACs_no_start:        for-all m: bool, not allowed(Draft, InProgress, false, m)
    c2_unmet_no_finish:        for-all h: bool, not allowed(InProgress, Done, h, false)
    c3_done_terminal:          for-all t: Status, h: bool, m: bool, not allowed(Done, t, h, m)
    c4_cancelled_terminal:     for-all t: Status, h: bool, m: bool, not allowed(Cancelled, t, h, m)
    c5_draft_cancellable:      for-all h: bool, m: bool, allowed(Draft, Cancelled, h, m)
    c6_inprogress_cancellable: for-all h: bool, m: bool, allowed(InProgress, Cancelled, h, m)
    c7_has_AC_can_start:       for-all m: bool, allowed(Draft, InProgress, true, m)
    c8_all_met_can_finish:     allowed(InProgress, Done, true, true)
  }

  gap {
    ac_done_guard_is_no_open_not_all_met
    cancel_verb_differs_from_promote_cancelled
  }
}
```

## Back-translation

Each `proves` claim in plain English, so the intent can be audited against the formal form:

1. **c1** — a draft with **no** AC is never allowed to start.
2. **c2** — an in_progress milestone is never allowed to finish while **not all** ACs are met.
3. **c3** — nothing ever leaves **done**.
4. **c4** — nothing ever leaves **cancelled**.
5. **c5** — a **draft** may always be cancelled.
6. **c6** — an **in_progress** milestone may always be cancelled.
7. **c7** — a draft that **has** an AC may start.
8. **c8** — an in_progress milestone with **all** ACs met may finish.

## `gap` detail

- **`ac_done_guard_is_no_open_not_all_met`** — the code's real done-guard is "no *open* AC," not
  "all *met*": a `deferred`/`cancelled` AC still lets a milestone finish (`transition.go:265-275`;
  standing check `milestone-done-incomplete-acs`, `acs.go:303-339`). The `(hasAC, allACsMet)`
  vocabulary can't express this, so `does` is *stricter* than the code on that case, and c2's no-AC
  counterexample is partly a modeling artifact.
- **`cancel_verb_differs_from_promote_cancelled`** — `aiwf promote M cancelled` allows cancel in any
  AC state (what `does` encodes, so c5/c6 verify), but `aiwf cancel M` **blocks** cancelling an
  `open`-AC milestone (`promote.go:246-253`) — refuting c6 on that surface.
- Also: `--force` relaxes the FSM *direction* rule but not the AC gate; no sovereign / tdd-phase
  gate applies to milestone edges.
