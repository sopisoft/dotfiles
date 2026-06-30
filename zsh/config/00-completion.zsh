# Keep completion startup quiet and deterministic across host and containers.
autoload -Uz compinit

_zsh_cache_dir="${XDG_CACHE_HOME:-$HOME/.cache}/zsh"
_zcompdump="${_zsh_cache_dir}/.zcompdump-${ZSH_VERSION}"

mkdir -p "$_zsh_cache_dir"
chmod go-w "$_zsh_cache_dir" >/dev/null 2>&1 || true

zstyle ':completion:*' menu select
if [[ -s "$_zcompdump" ]]; then
    compinit -C -d "$_zcompdump"
else
    compinit -d "$_zcompdump"
fi

unset _zcompdump _zsh_cache_dir
