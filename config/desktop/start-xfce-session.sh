#!/usr/bin/env bash
set -eu

exec dbus-run-session --config-file="$HOME/.config/dbus-1/session.conf" -- xfce4-session
