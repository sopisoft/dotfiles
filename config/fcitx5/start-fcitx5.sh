#!/usr/bin/env bash
set -euo pipefail

source "$HOME/.config/fcitx5/environment.sh"

exec fcitx5 --disable=wayland,waylandim -d --replace
