#!/usr/bin/env bash
set -euo pipefail

NVIM_BIN="${NVIM_BIN:-nvim}"
XDG_CONFIG_HOME="${XDG_CONFIG_HOME:-$HOME/.config}"
XDG_DATA_HOME="${XDG_DATA_HOME:-$HOME/.local/share}"

XDG_CONFIG_HOME="${XDG_CONFIG_HOME}" XDG_DATA_HOME="${XDG_DATA_HOME}" \
    "${NVIM_BIN}" --headless \
    "+lua vim.pack.update(nil, { force = true })" \
    +qa
