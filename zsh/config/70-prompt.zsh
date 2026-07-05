# Prefer the host prompt binary inside distroboxes when available.
if [[ -n ${STARSHIP_DISTROBOX:-} && -x /run/host/usr/local/bin/starship ]]; then
    eval "$(/run/host/usr/local/bin/starship init zsh)"
elif command -v starship >/dev/null 2>&1; then
    eval "$(starship init zsh)"
fi
