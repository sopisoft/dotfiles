#!/usr/bin/env bash
set -euo pipefail

export XDG_SESSION_TYPE=x11
export XDG_CURRENT_DESKTOP=XFCE
export XDG_SESSION_DESKTOP=xfce
export DESKTOP_SESSION=xfce
export GTK_IM_MODULE=fcitx
export QT_IM_MODULE=fcitx
export XMODIFIERS=@im=fcitx
export DefaultIMModule=fcitx
export SDL_IM_MODULE=fcitx
export GLFW_IM_MODULE=fcitx

export PATH="$HOME/.nix-profile/bin:/nix/var/nix/profiles/default/bin:/usr/local/bin:$PATH"

if grep -qi microsoft /proc/sys/kernel/osrelease 2>/dev/null || [ -e /proc/sys/fs/binfmt_misc/WSLInterop ]; then
  unset WAYLAND_DISPLAY WAYLAND_SOCKET
  export GDK_BACKEND=x11
  export QT_QPA_PLATFORM=xcb
  export SDL_VIDEODRIVER=x11
fi

xfce_session_bin="$(command -v xfce4-session || true)"
if [ -n "$xfce_session_bin" ]; then
  xfce_session_root="$(dirname "$(dirname "$(readlink -f "$xfce_session_bin")")")"
  if [ -d "$xfce_session_root/etc/xdg" ]; then
    export XDG_CONFIG_DIRS="$xfce_session_root/etc/xdg:${XDG_CONFIG_DIRS:-/etc/xdg}"
  fi
fi

if [ -S /mnt/wslg/PulseServer ]; then
  export PULSE_SERVER=unix:/mnt/wslg/PulseServer
fi

export FCITX_ADDON_DIRS="$HOME/.nix-profile/lib/fcitx5:$HOME/.nix-profile/lib/x86_64-linux-gnu/fcitx5:${FCITX_ADDON_DIRS:-}"
export FCITX_DATA_DIRS="$HOME/.nix-profile/share/fcitx5:${FCITX_DATA_DIRS:-}"

fcitx5 --disable=wayland,waylandim -d --replace >/tmp/fcitx5-xrdp.log 2>&1 || true
