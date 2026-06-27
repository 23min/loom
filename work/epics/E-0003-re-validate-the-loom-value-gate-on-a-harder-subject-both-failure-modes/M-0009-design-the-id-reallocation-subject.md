---
id: M-0009
title: Design the id-reallocation subject
status: draft
parent: E-0003
depends_on:
    - M-0008
tdd: required
acs:
    - id: AC-1
      title: Gold spec verifies against the reference impl within timeout
      status: open
      tdd_phase: red
    - id: AC-2
      title: Obligation set is pinned to the gold ensures and ranks a weaker spec lower
      status: open
      tdd_phase: red
    - id: AC-3
      title: Mutant bank is clause-isolated and fully killed by the gold spec
      status: open
      tdd_phase: red
    - id: AC-4
      title: Over-claim fixture is caught by the validity gate
      status: open
      tdd_phase: red
    - id: AC-5
      title: Reallocation subject registered and calibrates green end-to-end
      status: open
      tdd_phase: red
---

## Goal

## Acceptance criteria

### AC-1 — Gold spec verifies against the reference impl within timeout

### AC-2 — Obligation set is pinned to the gold ensures and ranks a weaker spec lower

### AC-3 — Mutant bank is clause-isolated and fully killed by the gold spec

### AC-4 — Over-claim fixture is caught by the validity gate

### AC-5 — Reallocation subject registered and calibrates green end-to-end

