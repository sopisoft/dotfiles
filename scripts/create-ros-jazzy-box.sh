#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOTFILES_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
CONTAINER_NAME="${ROS_JAZZY_BOX_NAME:-ros-jazzy}"
IMAGE="${ROS_JAZZY_BOX_IMAGE:-docker.io/library/ubuntu:24.04}"
SETUP_SCRIPT="${DOTFILES_DIR}/scripts/setup-ros-jazzy-box.sh"

if ! command -v distrobox >/dev/null 2>&1; then
    echo "distrobox is not available on PATH." >&2
    exit 1
fi

if [[ ! -r "${SETUP_SCRIPT}" ]]; then
    echo "Missing ros-jazzy setup script: ${SETUP_SCRIPT}" >&2
    exit 1
fi

if ! distrobox list --no-color 2>/dev/null | awk -F'|' '{gsub(/^[ \t]+|[ \t]+$/, "", $2); print $2}' | grep -Fxq "${CONTAINER_NAME}"; then
    distrobox create \
        --name "${CONTAINER_NAME}" \
        --image "${IMAGE}" \
        --additional-flags "--device /dev/ttyUSB0 --device /dev/ttyACM0"
fi

distrobox enter "${CONTAINER_NAME}" -- \
    env DOTFILES_DIR="${DOTFILES_DIR}" \
    bash "/run/host${SETUP_SCRIPT}"
