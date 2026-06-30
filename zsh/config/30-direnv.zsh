# Direnv stays optional so shell startup does not fail on minimal systems.
if command -v direnv >/dev/null 2>&1; then
    eval "$(direnv hook zsh)"
fi
