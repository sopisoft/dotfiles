#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck disable=SC1091
source "${SCRIPT_DIR}/lib.sh"

source_nix_environment

(
    cd "${DOTFILES_DIR}"
    nix flake update
)

"${SCRIPT_DIR}/update-pack-plugins.sh"

if [[ "${APPLY_AFTER_UPDATE:-1}" == "1" ]]; then
    "${SCRIPT_DIR}/switch.sh"
fi
