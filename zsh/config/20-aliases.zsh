# Keep short aliases focused on commands used every day.
alias l='ls -CF'
alias la='ls -A'
alias ll='ls -alF'
alias ros='distrobox enter ros-jazzy'
alias vim='nvim'

export NVM_DIR="${NVM_DIR:-$HOME/.local/share/nvm}"

_dotfiles_load_nvm() {
    [[ -s "$NVM_DIR/nvm.sh" ]] || return 1

    unset -f _dotfiles_load_nvm nvm
    # shellcheck disable=SC1091
    source "$NVM_DIR/nvm.sh"
}

nvm() {
    _dotfiles_load_nvm || {
        print -u2 "nvm is not installed: $NVM_DIR/nvm.sh"
        return 127
    }

    nvm "$@"
}
