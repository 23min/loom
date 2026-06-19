#!/usr/bin/env bash
# Runs INSIDE the container via devcontainer.json `postCreateCommand`
# and `updateContentCommand`. Idempotent — every step is safe to
# re-run after a container rebuild.
#
# See .devcontainer/README.md for the operator guide.

set -euo pipefail

echo "==> loom devcontainer postcreate"

# ---- pinned tool versions (the only knobs to bump) ----------------
# aiwf: planning + provenance for the loom repo. Pinned to a release
# (not the ../aiwf working tree) so the environment is reproducible and
# independent of the sibling checkout. Keep in sync with the host.
AIWF_VERSION="v0.15.1"
# Dafny: the verifier for the loom-ultralight experiment. The dotnet
# tool package bundles a compatible z3, so no separate z3 install is
# needed. Bump this if `dotnet tool install` reports the version is
# unavailable on NuGet.
DAFNY_VERSION="4.9.0"
# The experiment harness is Rust: the toolchain comes from the `rust`
# devcontainer feature, and its dependencies are pinned by the harness's
# Cargo.lock — nothing to pin here.

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

# --- Dafny + z3 (the verifier) -------------------------------------
# Installed as a .NET global tool (lands `dafny` in ~/.dotnet/tools,
# already on PATH via devcontainer.json remoteEnv). The package bundles
# the z3 build Dafny expects.
if ! command -v dafny >/dev/null 2>&1; then
  echo "==> Installing Dafny ${DAFNY_VERSION} (bundles z3)"
  dotnet tool install --global Dafny --version "${DAFNY_VERSION}"
  export PATH="$HOME/.dotnet/tools:$PATH"
fi

# --- Rust harness toolchain ----------------------------------------
# The experiment harness is a Rust Cargo project; rustup/cargo are
# provided by the `rust` devcontainer feature and are already on PATH.
# Crates are fetched on first build per the harness's Cargo.lock.
# Nothing to install here.

# --- Claude Code CLI -----------------------------------------------
if ! command -v claude >/dev/null 2>&1; then
  echo "==> Installing Claude Code CLI"
  curl -fsSL https://claude.ai/install.sh | bash
  export PATH="$HOME/.local/bin:$PATH"
fi

# --- aiwf binary + planning/provenance scaffold --------------------
# Pinned release (not the sibling working tree). `aiwf init` is
# idempotent: it regenerates the gitignored skill adapters and the
# chain-aware git hooks without clobbering committed scaffold.
echo "==> Installing aiwf ${AIWF_VERSION} and materializing planning scaffold"
go install "github.com/23min/aiwf/cmd/aiwf@${AIWF_VERSION}"
export PATH="$(go env GOPATH)/bin:$PATH"
aiwf init || true

# --- verification + banner -----------------------------------------
echo
echo "==> Tool versions:"
go version       || true
rustc --version  || true
cargo --version  || true
dafny --version  || echo "    (dafny NOT found — bump DAFNY_VERSION in init.sh and rebuild)"
aiwf version     || true
claude --version || true

cat <<'BANNER'

================================================================
loom devcontainer ready.

Verify the verifier can prove a trivial goal:
  echo 'lemma T() ensures 1+1==2 {}' > /tmp/t.dfy && dafny verify /tmp/t.dfy

Run the loom-ultralight experiment (once its files are materialized):
  cd experiments/loom-ultralight && ./run.sh   # needs ANTHROPIC_API_KEY

ANTHROPIC_API_KEY is forwarded from the host environment. If it is
empty inside the container, set it on the host before opening the
container (export ANTHROPIC_API_KEY=... in your host shell profile).

See .devcontainer/README.md for the full operator guide.
================================================================

BANNER
