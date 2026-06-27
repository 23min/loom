#!/usr/bin/env bash
# loom-ultralight — calibrate first; the paid experiment is opt-in.
#
#   ./run.sh                 Calibrate every subject dry (dafny only; no API, no cost).
#                            START HERE.
#   ./run.sh --full          Also run the paid two-arm experiment on the E-0002
#                            subjects (fsm, prosey): generate specs, score kill-rate +
#                            structural strength, map each to its §6 verdict, and apply
#                            the M-0007 combination rule. Needs ANTHROPIC_API_KEY.
#   LOOM_TRIALS=1 ./run.sh --full
#                            A cheap 1-trial-per-arm SMOKE run — validates the live
#                            generation/scoring path before the full (default 10) run.
set -euo pipefail
cd "$(dirname "$0")"

# Load .env, exporting only non-empty values — so an unfilled placeholder never
# clobbers a key already exported in the environment.
if [[ -f .env ]]; then
  while IFS='=' read -r k v; do
    [[ "$k" =~ ^[A-Za-z_][A-Za-z0-9_]*$ && -n "$v" ]] && export "$k=$v"
  done < .env
fi

# subject -> gold .dfy
declare -A GOLD=( [canonicalize]=canonicalize.dfy [fsm]=fsm.dfy [prosey]=prosey.dfy )
SUBJECTS=(canonicalize fsm prosey)

echo "== Step 0: calibrate every subject (dafny only, no API, no cost) =="
for subj in "${SUBJECTS[@]}"; do
  echo
  echo "-- $subj: dafny verify ${GOLD[$subj]} (gold lemmas) --"
  dafny verify "${GOLD[$subj]}"
  echo "-- $subj: gold spec valid + kills the full mutant bank --"
  LOOM_SUBJECT="$subj" cargo run --release --quiet -- --calibrate
done

echo
echo "== Step 1: pre-registration precedes the run (AC-2 git-ancestor guard) =="
cargo run --release --quiet -- --check-prereg-ancestry

if [[ "${1:-}" != "--full" ]]; then
  echo
  echo "Calibration green. To run the paid experiment (API calls, spends tokens):"
  echo "    # put your key in .env  (ANTHROPIC_API_KEY=...)  or export it, then:"
  echo "    LOOM_TRIALS=1 ./run.sh --full   # cheap smoke run first"
  echo "    ./run.sh --full                 # the full run"
  exit 0
fi

if [[ -z "${ANTHROPIC_API_KEY:-}" ]]; then
  echo "ANTHROPIC_API_KEY is empty — set it in .env or the environment before --full." >&2
  exit 1
fi

TRIALS="${LOOM_TRIALS:-10}"
echo
echo "== Step 2: run the two-arm experiment on the E-0002 subjects (TRIALS=$TRIALS/arm) =="
declare -A RUNDIR
for subj in fsm prosey; do
  echo
  echo "-- $subj: generate (disinterested + incentivized) --"
  LOOM_SUBJECT="$subj" LOOM_TRIALS="$TRIALS" cargo run --release --quiet -- --run
  dir=$(ls -dt "runs/$subj"/*/ | head -1)
  RUNDIR[$subj]="$dir"
  echo "-- $subj: structural strength + §6 verdict over $dir --"
  LOOM_SUBJECT="$subj" LOOM_TRIALS="$TRIALS" cargo run --release --quiet -- --strength "$dir"
done

echo
echo "== Step 3: combine the two subject verdicts (M-0007 rule) -> go/no-go =="
cargo run --release --quiet -- --decide "${RUNDIR[fsm]}" "${RUNDIR[prosey]}"

echo
echo "All done. Per-subject results under runs/<subject>/<ts>/ (results.json, strength.json, verdict.json)."
