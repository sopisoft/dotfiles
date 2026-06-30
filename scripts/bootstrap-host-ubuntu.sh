#!/usr/bin/env bash
set -euo pipefail

if ! command -v apt-get >/dev/null 2>&1; then
    echo "This script is intended to be run on Ubuntu or Debian-based systems."
    exit 1
fi

sudo apt-get update
sudo DEBIAN_FRONTEND=noninteractive apt-get install -y \
    build-essential \
    ca-certificates \
    curl \
    dconf-cli \
    dbus-user-session \
    distrobox \
    fuse-overlayfs \
    git \
    podman \
    slirp4netns \
    uidmap \
    unzip \
    xdg-user-dirs \
    xz-utils

if command -v systemctl >/dev/null 2>&1; then
    systemctl --user enable --now podman.socket >/dev/null 2>&1 || true
fi
