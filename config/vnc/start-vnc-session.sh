#!/usr/bin/env bash
set -euo pipefail

exec x11vnc \
  -display "${DISPLAY:-:0}" \
  -auth guess \
  -forever \
  -shared \
  -localhost \
  -noxdamage \
  -rfbport 5900
