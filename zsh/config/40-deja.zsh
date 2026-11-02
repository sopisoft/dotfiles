# Load deja integration when the command is available.
if command -v deja >/dev/null 2>&1; then
    if [[ -r "$HOME/.local/share/deja/init.zsh" ]]; then
        source "$HOME/.local/share/deja/init.zsh"
    else
        eval "$(deja init zsh)"
    fi
fi
