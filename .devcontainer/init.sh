#!/usr/bin/env bash
# Runs INSIDE the container via devcontainer.json `postCreateCommand`
# and `updateContentCommand`. Idempotent — every step is safe to
# re-run after a container rebuild.
#
# The toolchains (Go, Rust, Dafny, gh, Claude CLI) are baked into the
# image by the Dockerfile. This script does only the runtime wiring that
# can't be baked: git identity, the gh credential helper, and the aiwf
# install (kept here, NOT in the Dockerfile, so it tracks @latest and
# refreshes on every container create).
#
# See .devcontainer/README.md for the operator guide.

set -euo pipefail

echo "==> loom devcontainer postcreate"

# --- GIT_* env hygiene ---------------------------------------------
# Defensively unset GIT_DIR/GIT_WORK_TREE/GIT_COMMON_DIR so the
# worktree's .git is the authoritative source. Some devcontainer probe
# paths leave these set to host paths that don't resolve in the
# container, producing `fatal: not a git repository` on later git ops.
unset GIT_DIR GIT_WORK_TREE GIT_COMMON_DIR

# --- stale core.hooksPath unset ------------------------------------
# A host-absolute `core.hooksPath` in the repo config doesn't exist
# inside the container and makes aiwf's hook-installing verbs crash.
# Unset so git's default `<gitdir>/hooks` discovery applies.
git config --unset core.hooksPath 2>/dev/null || true

# --- git identity --------------------------------------------------
# Match the host identity so aiwf commit trailers (which derive the
# actor from the localpart of user.email) stay consistent across host
# and container.
echo "==> Configuring git identity"
git config --global user.name "Peter Bruinsma"
git config --global user.email "peter@23min.com"

# --- gh credential helper ------------------------------------------
echo "==> Configuring gh credential helper"
for host in https://github.com https://gist.github.com; do
  git config --global --unset-all "credential.${host}.helper" 2>/dev/null || true
  git config --global --add "credential.${host}.helper" ""
  git config --global --add "credential.${host}.helper" "!gh auth git-credential"
done

# --- aiwf binary + planning/provenance scaffold --------------------
# Installed @latest (not pinned) so the container tracks the newest aiwf
# on every create — Go itself is baked into the image. `aiwf init` is
# idempotent: it regenerates the gitignored skill adapters and the
# chain-aware git hooks without clobbering committed scaffold.
echo "==> Installing aiwf @latest and materializing planning scaffold"
go install github.com/23min/aiwf/cmd/aiwf@latest
aiwf init || true

# --- verification + banner -----------------------------------------
echo
echo "==> Tool versions:"
go version       || true
rustc --version  || true
cargo --version  || true
dafny --version  || echo "    (dafny NOT found — check the Dockerfile DAFNY_VERSION build)"
tlc 2>&1 | head -1 || echo "    (tlc NOT found — check the Dockerfile TLA_TOOLS_VERSION build)"
aiwf version     || true
claude --version || true

cat <<'BANNER'

================================================================
loom devcontainer ready.

Verify the verifier can prove a trivial goal:
  echo 'lemma T() ensures 1+1==2 {}' > /tmp/t.dfy && dafny verify /tmp/t.dfy

Run the loom-ultralight experiment:
  cd experiments/loom-ultralight && ./run.sh          # calibrate (no API key)
  cd experiments/loom-ultralight && ./run.sh --full   # experiment (needs key)

ANTHROPIC_API_KEY is forwarded from the host environment. If it is
empty inside the container, set it on the host before opening the
container (export ANTHROPIC_API_KEY=... in your host shell profile).

See .devcontainer/README.md for the full operator guide.
================================================================

BANNER
