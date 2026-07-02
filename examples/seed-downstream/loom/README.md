# loom overlay

This directory is the **entire** loom footprint in this (stand-in) downstream repo. It is
opt-in (`make loom`), never part of the default pipeline, and removable without trace:
delete `loom/` and the host is byte-identical (M-0016/AC-1).

Each subdirectory is one **property** — a self-contained umbrella plus its formal lowering
and generated gap report. Properties reference their subject (aiwf `@v0.20.0`) by version;
the lowering is a self-contained model, not read from the host at verify time (G1).

Seed properties: `fsm-terminality/`, `cancel-edge-legality/` (the at-risk one),
`archive-terminality/`.
