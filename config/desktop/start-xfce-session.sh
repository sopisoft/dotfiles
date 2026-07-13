#!/usr/bin/env bash
set -eu

if [ -r "$HOME/.config/dotfiles/session-env.sh" ]; then
  . "$HOME/.config/dotfiles/session-env.sh"
fi

exec dbus-run-session --config-file="$HOME/.config/dbus-1/session.conf" -- xfce4-session
