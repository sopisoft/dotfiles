#!/bin/sh
set -eu

for enter_script in \
    "$HOME/.local/share/dotfiles/host/zsh/config/lib/distrobox-enter-login.sh" \
    "$HOME/.config/zsh/lib/distrobox-enter-login.sh"
do
    if [ -r "$enter_script" ]; then
        exec sh "$enter_script" "$@"
    fi
done

if [ "$#" -eq 0 ]; then
    exec zsh -l
fi

exec "$@"
