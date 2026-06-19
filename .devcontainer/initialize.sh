#!/usr/bin/env bash
# Runs on the HOST (not in the container) via devcontainer.json
# `initializeCommand`. Prepares stable mount sources under /tmp so
# devcontainer.json `mounts:` entries don't have to reference $HOME
# (which devcontainer.json can't expand portably).
#
# The mount-source names are loom-specific (`.loom-*`) so that opening
# loom and a sibling project (e.g. aiwf) in containers at the same time
# does not have the two repos fight over the same /tmp symlinks.
#
# Plugin shadow-mount workaround for anthropics/claude-code#31388:
# Claude Code's plugin index stores absolute host paths. A
# macOS-pathed index (~/.claude/plugins/...) breaks inside a Linux
# container, and a Linux-pathed index breaks back on the host. We
# shadow the container's plugin-index dir with ~/.claude-linux/plugins
# so the host's macOS-pathed index stays untouched and the container
# has its own Linux-pathed parallel index.
#
# Remove the plugins-mount entry here AND the corresponding mount in
# devcontainer.json once claude-code#31388 ships a fix that resolves
# plugin paths relative to $HOME:
#   https://github.com/anthropics/claude-code/issues/31388
#
# The three mount points:
#   ~/.claude               -> /tmp/.loom-claude-mount          (full state shared with host)
#   ~/.claude-linux/plugins -> /tmp/.loom-claude-plugins-mount  (container-only plugin index)
#   ~/.config/gh            -> /tmp/.loom-gh-mount              (gh auth shared with host)

set -euo pipefail

mkdir -p "$HOME/.claude"
mkdir -p "$HOME/.claude-linux/plugins"
mkdir -p "$HOME/.config/gh"

ln -sfn "$HOME/.claude"                /tmp/.loom-claude-mount
ln -sfn "$HOME/.claude-linux/plugins"  /tmp/.loom-claude-plugins-mount
ln -sfn "$HOME/.config/gh"             /tmp/.loom-gh-mount
