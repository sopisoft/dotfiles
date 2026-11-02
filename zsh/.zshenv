export ZDOTDIR="$HOME"
export skip_global_compinit=1

typeset -U path PATH
path=(
    "$HOME/.local/bin"
    "$HOME/.local/share/mise/shims"
    "/home/linuxbrew/.linuxbrew/bin"
    "/opt/homebrew/bin"
    "/usr/local/texlive/2026/bin/x86_64-linux"
    $path
)
export PATH

export CARGO_HOME="$HOME/.cargo"
export DOTFILES_DIR="${DOTFILES_DIR:-$HOME/dotfiles}"
export EDITOR="nvim"
export VISUAL="nvim"
export TERMINAL="alacritty"
export HISTFILE="$HOME/.zsh_history"
export HISTSIZE=100000
export SAVEHIST=100000
