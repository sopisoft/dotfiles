#!/usr/bin/env bash
set -euo pipefail

NVIM_BIN="${NVIM_BIN:-nvim}"
XDG_CONFIG_HOME="${XDG_CONFIG_HOME:-$HOME/.config}"
XDG_DATA_HOME="${XDG_DATA_HOME:-$HOME/.local/share}"
LEGACY_CONFIG_PACK_DIR="${XDG_CONFIG_HOME}/nvim/pack"
LEGACY_PACK_DIR="${XDG_DATA_HOME}/nvim/site/pack/dotfiles"
STAGING_ROOT="$(mktemp -d)"
STAGING_CONFIG_HOME="${STAGING_ROOT}/config"

cleanup() {
    rm -rf "${STAGING_ROOT}"
}

trap cleanup EXIT

if ! command -v "${NVIM_BIN}" >/dev/null 2>&1; then
    echo "nvim is not available on PATH." >&2
    exit 1
fi

rm -rf "${LEGACY_PACK_DIR}" "${LEGACY_CONFIG_PACK_DIR}"
mkdir -p "${STAGING_CONFIG_HOME}/nvim"
cp -a "${XDG_CONFIG_HOME}/nvim/." "${STAGING_CONFIG_HOME}/nvim/"

XDG_CONFIG_HOME="${STAGING_CONFIG_HOME}" XDG_DATA_HOME="${XDG_DATA_HOME}" \
    "${NVIM_BIN}" --headless +qa
