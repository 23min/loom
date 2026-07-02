---
id: G-0010
title: Unreadable umbrella.md aborts the verify batch, not a per-property error
status: open
discovered_in: M-0016
---
## Problem

`crates/loom/src/runner.rs` reads each `umbrella.md` with `read_to_string` in
`report_for`, propagated via `?` through `verify`. A binary / non-UTF-8 (or
otherwise unreadable) umbrella errors at file-read *before* `parse()`, so the
error propagates out and aborts the whole batch — no report is written for that
property *or* for any sibling that would have verified. AC-5's guarantee ("a
malformed umbrella is recorded, never a silent skip") holds for parse failures
(`umbrella::parse` → typed `ParseError` → `GapReport::parse_error`) but not for
unreadable files. Confirmed via the CLI: exit 1, zero reports written.

## Direction

Extend per-property error containment to the read step: read bytes (or catch the
read/utf-8 error) and emit a `parse_error`-style report for that property, so one
unreadable umbrella degrades to a single error report and siblings still verify.
Keep parser totality intact (already sound).

## Discovered
M-0016 wrap review (code lens, finding N2).
