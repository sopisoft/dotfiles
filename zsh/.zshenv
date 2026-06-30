export ZDOTDIR="$HOME"
export skip_global_compinit=1

typeset -U path PATH
path=("$HOME/.local/bin" "$HOME/.cargo/bin" $path)
export PATH

export HISTFILE="$HOME/.zsh_history"
export HISTSIZE=100000
export SAVEHIST=100000
