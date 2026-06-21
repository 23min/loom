#!/usr/bin/env bash
# loom-ultralight — calibrate first; the paid experiment is opt-in.
#
#   ./run.sh           Calibrate only: dafny verify + the 8/8 mutant check.
#                      No API key, no cost. START HERE.
#   ./run.sh --full    Also run the experiment (needs ANTHROPIC_API_KEY; spends
#                      API tokens).
set -euo pipefail
cd "$(dirname "$0")"

echo "== Step 0a: dafny verify canonicalize.dfy (GoldSpec + Idempotent — M-0001 AC-1) =="
dafny verify canonicalize.dfy

echo
echo "== Step 0b: calibrate — gold spec must be valid and kill 8/8 (M-0001 AC-2) =="
cargo run --release --quiet -- --calibrate

if [[ "${1:-}" != "--full" ]]; then
  echo
  echo "Calibration green. To run the experiment (API calls, spends tokens):"
  echo "    export ANTHROPIC_API_KEY=...   # if not already set"
  echo "    ./run.sh --full"
  exit 0
fi

echo
echo "== Step 1: run experiment (needs ANTHROPIC_API_KEY — M-0002) =="
cargo run --release --quiet -- --run
