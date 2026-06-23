# Roadmap

## E-0001 — Validate the loom differentiator (loom-ultralight) (done)

### Goal

Cheaply test the load-bearing hypothesis behind loom **before** building loom-light:
that an LLM authoring a formal spec writes a *weaker* spec when it is also graded on
making its own implementation pass (incentivized) than when it only specifies
(disinterested) — **and** that a mutation / kill-rate check catches the difference.

If the gap appears and the check catches it, loom's differentiator is real and we
proceed to loom-light. If not, we learned it cheaply, before committing to a Rust
engine, a claims surface, or a verifier integration. The full design is in
`docs/loom-ultralight.md`.

| Milestone | Title | Status |
|---|---|---|
| M-0001 | Materialize the loom-ultralight experiment into runnable files | done |
| M-0002 | Run the loom-ultralight experiment and record the kill-rate gap | done |

