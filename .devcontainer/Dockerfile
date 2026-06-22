# loom development container
#
# Base: Microsoft's official devcontainer base for Ubuntu 24.04. We use this
# (rather than the raw ubuntu:24.04 image) because it already ships the vscode
# user (uid 1000), sudo, the apt-sandbox gpgv fix, and common shell tooling
# (git, curl, gnupg, zsh, build-essential, jq, less). It IS Ubuntu — we just
# inherit the dev niceties instead of hand-rolling them.
#
# Everything loom needs is baked into pinned, cached layers here; there are NO
# devcontainer features (no ghcr.io feature fetches to time out on). The one
# thing NOT pinned here is aiwf — it is installed @latest at container-create
# time in init.sh, so it refreshes on every rebuild.
#
# Toolchains and why:
#   - Go      : ONLY to `go install` aiwf (aiwf publishes no release binary).
#   - Rust    : the loom-ultralight experiment harness + loom-light work.
#   - Dafny   : the verifier — self-contained release binary, bundles its own
#               runtime + z3, so there is NO .NET SDK in this image.
#   - gh      : GitHub access (apt repo, not a feature).
#   - Claude  : agent workflows (native installer, as the vscode user).
FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

ENV DEBIAN_FRONTEND=noninteractive \
    LC_ALL=C.UTF-8 \
    LANG=C.UTF-8

# System packages the base image does not already include.
#   - unzip    : extract the Dafny release zip
#   - libicu74 : ICU runtime the self-contained Dafny (a .NET app) links against
#   - gnupg    : verify the gh apt keyring
RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        gnupg \
        unzip \
        libicu74 \
    && rm -rf /var/lib/apt/lists/*

# GitHub CLI from the official apt repo (no devcontainer feature).
RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
        -o /usr/share/keyrings/githubcli-archive-keyring.gpg \
    && chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" \
        > /etc/apt/sources.list.d/github-cli.list \
    && apt-get update && apt-get install -y --no-install-recommends gh \
    && rm -rf /var/lib/apt/lists/*

# Go toolchain — only to `go install` the aiwf planning CLI, which ships no
# prebuilt binary. Pulled system-wide from the official tarball. Arch-aware so
# the image builds on amd64 and arm64. Pin deliberately; must be >= aiwf's
# go.mod directive (currently go 1.24).
ARG GO_VERSION=1.26.4
RUN arch="$(dpkg --print-architecture)" \
    && curl -fsSL "https://go.dev/dl/go${GO_VERSION}.linux-${arch}.tar.gz" -o /tmp/go.tar.gz \
    && rm -rf /usr/local/go \
    && tar -C /usr/local -xzf /tmp/go.tar.gz \
    && rm /tmp/go.tar.gz \
    && /usr/local/go/bin/go version

# Rust toolchain — the experiment harness. Installed system-wide via rustup so
# every shell sees cargo. Pin via RUST_VERSION (an exact version like 1.86.0 or
# the `stable` channel); the harness's deps are byte-pinned by its Cargo.lock.
ARG RUST_VERSION=stable
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo
RUN curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs \
        | sh -s -- -y --no-modify-path --profile minimal --default-toolchain "${RUST_VERSION}" \
    && /usr/local/cargo/bin/rustup component add rustfmt clippy \
    && chmod -R a+rwX "${RUSTUP_HOME}" "${CARGO_HOME}" \
    && /usr/local/cargo/bin/rustc --version

# Dafny — the verifier. Self-contained release binary (bundles its own .NET
# runtime AND z3), so this image needs no .NET SDK. Pin deliberately; bump
# DAFNY_VERSION to track Dafny releases.
#
# NOTE: Dafny ships an x64-ubuntu build but NO linux-arm64 build. On amd64
# (this repo's hosts) it runs natively. On arm64 (e.g. Apple Silicon) there is
# no self-contained Linux binary — fall back to `dotnet tool install --global
# Dafny` (arch-portable via NuGet) or force --platform=linux/amd64. We fail
# loudly rather than silently producing a broken `dafny`.
ARG DAFNY_VERSION=4.9.0
RUN arch="$(dpkg --print-architecture)" \
    && if [ "$arch" != "amd64" ]; then \
         echo "ERROR: Dafny ${DAFNY_VERSION} has no self-contained linux-${arch} binary." >&2; \
         echo "       Use 'dotnet tool install --global Dafny' or build --platform=linux/amd64." >&2; \
         exit 1; \
       fi \
    && curl -fsSL "https://github.com/dafny-lang/dafny/releases/download/v${DAFNY_VERSION}/dafny-${DAFNY_VERSION}-x64-ubuntu-20.04.zip" -o /tmp/dafny.zip \
    && unzip -q /tmp/dafny.zip -d /usr/local/lib \
    && rm /tmp/dafny.zip \
    && ln -sf /usr/local/lib/dafny/dafny /usr/local/bin/dafny \
    && dafny --version

# PATH + Go env for every shell. GOTOOLCHAIN=local pins us to the Go installed
# above (no silent auto-download of a different version). aiwf, installed by
# init.sh via `go install`, lands in /home/vscode/go/bin.
ENV GOTOOLCHAIN=local \
    GOPATH=/home/vscode/go \
    PATH=/usr/local/go/bin:/home/vscode/go/bin:/usr/local/cargo/bin:/home/vscode/.local/bin:${PATH}

# Ensure the per-user Go bin dir exists and is owned by vscode before init.sh
# writes the aiwf binary into it.
RUN mkdir -p /home/vscode/go/bin && chown -R vscode:vscode /home/vscode/go

USER vscode

# Claude Code CLI — Anthropic's native installer drops the binary into
# ~/.local/bin/claude (on PATH above). Installed as vscode so it lands in
# /home/vscode, not /root. Auto-updates at runtime by default.
RUN curl -fsSL https://claude.ai/install.sh | bash \
    && /home/vscode/.local/bin/claude --version
