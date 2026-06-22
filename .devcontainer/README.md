# loom devcontainer

A reproducible environment for loom: the verification toolchain (Dafny +
z3) the [loom-ultralight experiment](../docs/loom-ultralight.md) needs, Rust for
the experiment harness and loom-light implementation work, and
[aiwf](https://github.com/23min/aiwf) for loom's own planning and provenance.

## What's in it

Everything is baked into a [`Dockerfile`](Dockerfile) on top of
`mcr.microsoft.com/devcontainers/base:ubuntu-24.04` — an **Ubuntu** base that
ships the `vscode` user, sudo, and common shell tooling. There are **no
devcontainer features** (so no ghcr.io feature fetches to time out on); each
toolchain is a pinned, cached image layer.

| Tool | How it's installed | Why |
|---|---|---|
| **Go** | Dockerfile, official tarball (`GO_VERSION`, arch-aware) | builds/installs aiwf — its *only* use here |
| **Rust** | Dockerfile, `rustup` (`RUST_VERSION`); deps via `Cargo.lock` | the experiment harness **and** loom-light implementation |
| **Dafny + z3** | Dockerfile, self-contained release binary (`DAFNY_VERSION`) | the verifier — bundles its own runtime + z3, **no .NET SDK** |
| **gh** | Dockerfile, official apt repo | GitHub access |
| **Claude Code CLI** | Dockerfile, `claude.ai/install.sh` | agent workflows |
| **aiwf** | `init.sh`, `go install …@latest` | planning + provenance; refreshed every container create |

Pinned versions are `ARG`s at the top of each toolchain block in the
[`Dockerfile`](Dockerfile) (`GO_VERSION`, `RUST_VERSION`, `DAFNY_VERSION`) — the
knobs to bump. aiwf is deliberately **not** pinned: it installs `@latest` in
[`init.sh`](init.sh), which re-runs on every rebuild. The harness's Rust
dependencies are byte-pinned separately by `experiments/loom-ultralight/Cargo.lock`.

## Design notes

This container is deliberately **self-contained** and **Ubuntu-native**:

- **No .NET.** Dafny is the only .NET-rooted dependency, and we install its
  self-contained release binary (which embeds its own runtime + z3) rather than
  the `dotnet tool` — so there is no .NET SDK in the image. See the arch caveat
  below.
- **No devcontainer features.** Toolchains are installed directly in the
  Dockerfile (the pattern used by the sibling `deathcleaning` / `wa-shui-do`
  containers), so the image builds from pinned upstreams with no ghcr.io
  feature-fetch step.
- **Go exists only to install aiwf**, which publishes no prebuilt binary. There
  is no Go source in loom.
- **aiwf is `@latest`, not a sibling build.** The experiment doesn't import aiwf
  at runtime — the `Canonicalize` contract is transcribed into self-contained
  Dafny in the experiment doc — so aiwf here is only for planning/provenance.

The shared-state machinery (the `/tmp/.loom-*` mount sources, the
plugin-shadow workaround for
[claude-code#31388](https://github.com/anthropics/claude-code/issues/31388))
is borrowed from aiwf's proven setup, with loom-specific mount names so the
two repos can be open in containers simultaneously without colliding.

### Architecture caveat (Dafny)

Dafny ships an `x64-ubuntu` self-contained binary but **no linux-arm64 build**.
On amd64 hosts (this repo's) it runs natively. On arm64 (e.g. Apple Silicon)
the Dockerfile fails the Dafny step loudly — fall back to `dotnet tool install
--global Dafny` (arch-portable via NuGet) or build with `--platform=linux/amd64`.

## Using it

1. **Set `ANTHROPIC_API_KEY` on the host** before opening the container — it
   is forwarded via `remoteEnv` so the experiment harness can call the API:
   ```bash
   export ANTHROPIC_API_KEY=sk-ant-...
   ```
2. **VS Code:** Command Palette → *Dev Containers: Reopen in Container*.
   **CLI:** `devcontainer up --workspace-folder /path/to/loom`.
3. On create, [`init.sh`](init.sh) wires git identity + the gh credential
   helper, installs aiwf `@latest`, runs `aiwf init`, and prints a tool-version
   banner. It is idempotent across rebuilds.

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

- **`dafny` not found / image build fails at the Dafny step:** the pinned
  `DAFNY_VERSION` may not have an `x64-ubuntu` asset, or you're building on
  arm64 (see the architecture caveat). Bump `DAFNY_VERSION` in the `Dockerfile`
  or use the `dotnet tool` fallback, then rebuild.
- **`ANTHROPIC_API_KEY` empty inside the container:** it was not set in the
  host shell that launched the container. Set it on the host and rebuild (or
  `export` it inside the container for a one-off run).
- **Harness build:** the harness is Rust; `experiments/loom-ultralight/run.sh`
  runs `cargo run`, and the first build fetches crates per `Cargo.lock`.

## Reproducibility note

The image is byte-pinned by the `ARG` versions in the `Dockerfile` (Go, Rust,
Dafny) plus the gh/Claude upstreams; the harness's Rust dependencies are pinned
by `experiments/loom-ultralight/Cargo.lock`. aiwf is the one intentional
exception — `@latest`, refreshed per container create. Because the toolchains
live in cached Dockerfile layers, rebuilds are fast and don't re-fetch unless a
pin (or the layer above it) changes.
