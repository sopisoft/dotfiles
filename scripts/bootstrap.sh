#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck disable=SC1091
source "${SCRIPT_DIR}/lib.sh"

if [[ "${SKIP_HOST_PACKAGES:-0}" != "1" ]] && command -v apt-get >/dev/null 2>&1; then
    "${SCRIPT_DIR}/bootstrap-host-ubuntu.sh"
fi

ensure_nix_command
source_nix_environment
write_nix_conf
apply_home_manager
"${SCRIPT_DIR}/install-nvm.sh"

if [[ "${SKIP_ROS_JAZZY_BOX:-0}" != "1" ]] && command -v distrobox >/dev/null 2>&1; then
    "${SCRIPT_DIR}/create-ros-jazzy-box.sh"
fi
