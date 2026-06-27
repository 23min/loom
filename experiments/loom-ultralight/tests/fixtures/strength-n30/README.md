# strength-n30 — frozen generation corpus (AC-2 golden regression input)

178 cached model generations (3 models × 2 conditions × up to 30 trials) from the
loom-ultralight N=30 run `1782154633`, whose aggregate structural-strength result
is the committed golden fixture [`results/strength-n30.json`](../../../results/strength-n30.json).

This corpus is **frozen test data**, not a live run directory: it pins the
canonicalize subject's strength verdicts so the M-0003 obligation-list
generalization can be proven behavior-preserving (E-0002 / M-0003 AC-2). The raw
`runs/` directory is gitignored; this committed copy makes the regression
reproducible from a fresh clone (loom principle G1).

Do not regenerate or edit these files — a changed input would silently move the
golden target.
