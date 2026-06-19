# loom devcontainer

A reproducible environment for loom: the verification toolchain (Dafny +
z3) the [loom-ultralight experiment](../docs/loom-ultralight.md) needs, the
Python SDK its harness calls, Rust for loom-light implementation work, and
[aiwf](https://github.com/23min/aiwf) for loom's own planning and
provenance.

## What's in it

| Tool | How it's installed | Why |
|---|---|---|
| **Go 1.25** | base image (`mcr.microsoft.com/devcontainers/go`) | builds/installs aiwf |
| **Dafny + z3** | `dotnet tool install Dafny` (pinned, bundles z3) | the verifier for the experiment |
| **Python 3 + `anthropic`** | dotnet/python features + `pip install` in `init.sh` | the experiment harness |
| **Rust** | `rust` devcontainer feature | loom-light implementation language |
| **aiwf** | `go install …@v0.15.1` (pinned release) | planning + provenance for this repo |
| **Claude Code CLI** | `claude.ai/install.sh` | agent workflows |
| **Node 22, gh, zsh** | devcontainer features | tooling + GitHub access |

Pinned versions live at the top of [`init.sh`](init.sh) (`AIWF_VERSION`,
`DAFNY_VERSION`) — the only knobs to bump.

## Design notes (how this differs from aiwf's devcontainer)

This container is deliberately **self-contained**:

- **aiwf is a pinned release, not a build of the `../aiwf` sibling.** The
  experiment doesn't import aiwf at runtime — the `Canonicalize` contract is
  transcribed into self-contained Dafny in the experiment doc — so aiwf here
  is only for planning/provenance. Pinning a release keeps the environment
  reproducible and independent of whatever is checked out next door.
- **No sibling mount, no Playwright, no git-worktree rewrite.** loom is a
  normal checkout, so the worktree `.git` rewrite aiwf needs doesn't apply.

The shared-state machinery (the `/tmp/.loom-*` mount sources, the
plugin-shadow workaround for
[claude-code#31388](https://github.com/anthropics/claude-code/issues/31388))
is borrowed from aiwf's proven setup, with loom-specific mount names so the
two repos can be open in containers simultaneously without colliding.

## Using it

1. **Set `ANTHROPIC_API_KEY` on the host** before opening the container — it
   is forwarded via `remoteEnv` so the experiment harness can call the API:
   ```bash
   export ANTHROPIC_API_KEY=sk-ant-...
   ```
2. **VS Code:** Command Palette → *Dev Containers: Reopen in Container*.
   **CLI:** `devcontainer up --workspace-folder /path/to/loom`.
3. On create, [`init.sh`](init.sh) installs everything, runs `aiwf init`,
   and prints a tool-version banner. It is idempotent across rebuilds.

### Smoke tests

```bash
# the verifier proves a trivial goal
echo 'lemma T() ensures 1+1==2 {}' > /tmp/t.dfy && dafny verify /tmp/t.dfy

# aiwf sees the repo
aiwf doctor

# the API key reached the container (non-empty)
test -n "$ANTHROPIC_API_KEY" && echo "key present" || echo "key MISSING — set it on the host"
```

## Troubleshooting

- **`dafny` not found after build:** the pinned `DAFNY_VERSION` may not exist
  on NuGet. Bump it in `init.sh` and rebuild. Fallback: a GitHub release zip
  from <https://github.com/dafny-lang/dafny/releases> also bundles z3.
- **`ANTHROPIC_API_KEY` empty inside the container:** it was not set in the
  host shell that launched the container. Set it on the host and rebuild (or
  `export` it inside the container for a one-off run).
- **`pip install` blocked (externally-managed):** shouldn't happen on the
  feature-built Python, but if it does, the harness can run in a venv —
  `python3 -m venv .venv && . .venv/bin/activate && pip install anthropic`.

## Reproducibility note

Feature versions are pinned by major tag in `devcontainer.json`. For a fully
byte-pinned environment, generate a lock file with the devcontainer CLL:

```bash
devcontainer features info --workspace-folder . > .devcontainer/devcontainer-lock.json
```
