#!/usr/bin/env bash
set -e

XRDP_DISPLAY="${DISPLAY:-:10}"

if [ -r /etc/profile ]; then
  # shellcheck disable=SC1091
  . /etc/profile
fi

for profile in "$HOME/.bash_profile" "$HOME/.bash_login" "$HOME/.profile"; do
  if [ -r "$profile" ]; then
    # shellcheck disable=SC1090
    . "$profile"
    break
  fi
done

if [ -r /etc/default/locale ]; then
  # shellcheck disable=SC1091
  . /etc/default/locale
  export LANG LANGUAGE
fi

if [ -r /etc/xrdp/session-env.sh ]; then
  # shellcheck disable=SC1091
  . /etc/xrdp/session-env.sh
fi

if grep -qi microsoft /proc/sys/kernel/osrelease 2>/dev/null || [ -e /proc/sys/fs/binfmt_misc/WSLInterop ]; then
  unset WAYLAND_DISPLAY WAYLAND_SOCKET
  export DISPLAY="$XRDP_DISPLAY"
fi

if grep -qi microsoft /proc/sys/kernel/osrelease 2>/dev/null || [ -e /proc/sys/fs/binfmt_misc/WSLInterop ]; then
  panel_dir="$HOME/.config/xfce4/xfconf/xfce-perchannel-xml"
  install -d -m 0755 "$panel_dir"
  if [ -r /etc/xrdp/xfce4-panel.wsl.xml ]; then
    install -m 0644 /etc/xrdp/xfce4-panel.wsl.xml "$panel_dir/xfce4-panel.xml"
  fi
fi

exec dbus-run-session --config-file="$HOME/.config/dbus-1/session.conf" -- xfce4-session
