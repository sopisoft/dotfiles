#!/bin/sh
set -eu

script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
init_hook="${script_dir}/distrobox-init-hook.sh"

if [ -r "$init_hook" ]; then
    sh "$init_hook"
fi

if [ "$#" -eq 0 ]; then
    exec zsh -l
fi

exec "$@"
