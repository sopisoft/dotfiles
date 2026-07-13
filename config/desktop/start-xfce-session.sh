#!/usr/bin/env bash
set -eu

if [ -r "$HOME/.nix-profile/etc/profile.d/hm-session-vars.sh" ]; then
  . "$HOME/.nix-profile/etc/profile.d/hm-session-vars.sh"
fi

exec dbus-run-session --config-file="$HOME/.config/dbus-1/session.conf" -- xfce4-session
