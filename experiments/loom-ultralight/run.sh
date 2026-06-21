#!/usr/bin/env bash
# loom-ultralight — calibrate, then run the endogenous-gaming experiment.
# Needs the devcontainer toolchain (Dafny + Z3 + Rust) and ANTHROPIC_API_KEY.
set -euo pipefail
cd "$(dirname "$0")"

echo "== Step 0a: dafny verify canonicalize.dfy (GoldSpec + Idempotent — M-0001 AC-1) =="
dafny verify canonicalize.dfy

echo
echo "== Step 0b: calibrate — gold spec must be valid and kill 8/8 (M-0001 AC-2) =="
cargo run --release --quiet -- --calibrate

echo
echo "== Step 1: run experiment (needs ANTHROPIC_API_KEY — M-0002) =="
cargo run --release --quiet -- --run
