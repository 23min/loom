# seed-downstream — a stand-in for a downstream repo (aiwf) that has adopted loom.
#
# The DEFAULT pipeline (`make` / `make all`) is loom-free. loom lives entirely under
# loom/ and is invoked only via the opt-in `make loom` target — never a prerequisite of
# `all`. Removing loom/ must leave `make all` byte-identical (M-0016/AC-1).
#
# `.RECIPEPREFIX = >` uses `>` instead of a leading tab for recipe lines (GNU make 3.82+),
# so the fixture is robust to whitespace handling.
.RECIPEPREFIX = >
.PHONY: all loom

# The loom binary. Defaults to whatever a downstream installs on PATH; the test harness
# overrides it with the freshly-built binary (`make loom LOOM_BIN=<path>`).
LOOM_BIN ?= loom

# The host's normal pipeline. loom is NOT part of it.
all:
> @echo "host build: ok"

# Opt-in loom verification — a separate target, never a prerequisite of `all` (M-0016/AC-2).
# `make loom` verifies every property; `make loom PROP=<id>` scopes to a single property.
loom:
> @$(LOOM_BIN) verify loom $(if $(strip $(PROP)),--prop $(strip $(PROP)))
