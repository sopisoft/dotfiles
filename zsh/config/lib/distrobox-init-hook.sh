#!/bin/sh
set -eu

host_root="/run/host"
host_home="${host_root}/home/$USER"
host_dotfiles="${host_home}/dotfiles"
local_dotfiles_root="${HOME}/.local/share/dotfiles"
local_dotfiles="${local_dotfiles_root}/host"

copy_file() {
    src="$1"
    dest="$2"

    [ -f "$src" ] || return 0

    mkdir -p "$(dirname "$dest")"
    rm -f "$dest"
    cp -fp "$src" "$dest"
}

copy_dir() {
    src="$1"
    dest="$2"

    [ -d "$src" ] || return 0

    rm -rf "$dest"
    mkdir -p "$dest"
    cp -a "$src/." "$dest/"
}

copy_home_manager_file() {
    src="$1"
    dest="$2"

    if [ -L "$src" ]; then
        target="$(readlink "$src")"
        case "$target" in
            /*)
                src="${host_root}${target}"
                ;;
        esac
    fi

    copy_file "$src" "$dest"
}

copy_home_manager_dir() {
    src="$1"
    dest="$2"
    staging="$(mktemp -d)"

    [ -d "$src" ] || return 0

    find "$src" -type d | while IFS= read -r dir; do
        rel="${dir#$src/}"
        [ "$dir" = "$src" ] && rel=""
        mkdir -p "${staging}/${rel}"
    done

    find "$src" \( -type f -o -type l \) | while IFS= read -r file; do
        rel="${file#$src/}"
        out="${staging}/${rel}"
        mkdir -p "$(dirname "$out")"

        if [ -L "$file" ]; then
            target="$(readlink "$file")"
            case "$target" in
                /*)
                    file="${host_root}${target}"
                    ;;
            esac
        fi

        cp -fp "$file" "$out"
    done

    rm -rf "$dest"
    mkdir -p "$dest"
    cp -a "$staging/." "$dest/"
    rm -rf "$staging"
}

ensure_link() {
    path="$1"
    target="$2"

    mkdir -p "$(dirname "$path")"
    rm -f "$path"
    ln -s "$target" "$path"
}

sync_dotfiles_repo() {
    src="$1"
    dest="$2"
    staging="$(mktemp -d)"

    [ -d "$src" ] || return 1

    find "$src" -mindepth 1 -maxdepth 1 ! -name .git -exec cp -a {} "$staging/" \;

    rm -rf "$dest"
    mkdir -p "$(dirname "$dest")"
    mv "$staging" "$dest"
}

neovim_version_ge() {
    nvim_bin="$1"
    version="$("$nvim_bin" --version 2>/dev/null | awk 'NR == 1 { sub(/^v/, "", $2); print $2 }')"
    [ -n "$version" ] || return 1
    dpkg --compare-versions "$version" ge 0.12.0
}

ensure_neovim_wrappers() {
    target="$1"

    ensure_link "$HOME/.local/bin/nvim" "$target"
    ensure_link "$HOME/.local/bin/vim" "$target"
}

install_local_neovim() {
    arch="$(dpkg --print-architecture 2>/dev/null || true)"

    case "$arch" in
        amd64)
            asset_arch="x86_64"
            ;;
        arm64)
            asset_arch="arm64"
            ;;
        *)
            return 1
            ;;
    esac

    command -v curl >/dev/null 2>&1 || return 1
    command -v tar >/dev/null 2>&1 || return 1

    install_root="$HOME/.local/opt"
    install_dir="$install_root/nvim"
    archive_root="nvim-linux-${asset_arch}"
    archive_url="https://github.com/neovim/neovim-releases/releases/latest/download/${archive_root}.tar.gz"
    staging="$(mktemp -d)"

    if ! curl -fsSL "$archive_url" -o "$staging/nvim.tar.gz"; then
        rm -rf "$staging"
        return 1
    fi

    if ! tar -xzf "$staging/nvim.tar.gz" -C "$staging"; then
        rm -rf "$staging"
        return 1
    fi

    rm -rf "$install_dir"
    mkdir -p "$install_root"
    mv "$staging/$archive_root" "$install_dir"
    rm -rf "$staging"

    [ -x "$install_dir/bin/nvim" ] || return 1
    ensure_neovim_wrappers "$install_dir/bin/nvim"
}

ensure_neovim() {
    local_nvim="$HOME/.local/opt/nvim/bin/nvim"

    if [ -x "$local_nvim" ] && neovim_version_ge "$local_nvim"; then
        ensure_neovim_wrappers "$local_nvim"
        return 0
    fi

    if command -v nvim >/dev/null 2>&1; then
        system_nvim="$(command -v nvim)"
        if neovim_version_ge "$system_nvim"; then
            ensure_neovim_wrappers "$system_nvim"
            return 0
        fi
    fi

    install_local_neovim || true
}

source_dotfiles=""

if [ -d "$host_dotfiles" ]; then
    sync_dotfiles_repo "$host_dotfiles" "$local_dotfiles" || true
fi

if [ -d "$local_dotfiles" ]; then
    source_dotfiles="$local_dotfiles"
fi

if [ -n "$source_dotfiles" ]; then
    copy_file "$source_dotfiles/zsh/.zshrc" "$HOME/.zshrc"
    copy_file "$source_dotfiles/zsh/.zshenv" "$HOME/.zshenv"
    copy_file "$source_dotfiles/zsh/.zprofile" "$HOME/.zprofile"

    mkdir -p "$HOME/.config"
    copy_dir "$source_dotfiles/config/direnv" "$HOME/.config/direnv"
    copy_dir "$source_dotfiles/config/nix" "$HOME/.config/nix"
    copy_dir "$source_dotfiles/config/nvim" "$HOME/.config/nvim"
    copy_file "$source_dotfiles/config/starship.toml" "$HOME/.config/starship.toml"
    copy_dir "$source_dotfiles/config/zellij" "$HOME/.config/zellij"
    copy_dir "$source_dotfiles/zsh/config" "$HOME/.config/zsh"
elif [ -d "$host_home" ]; then
    copy_home_manager_file "$host_home/.zshrc" "$HOME/.zshrc"
    copy_home_manager_file "$host_home/.zshenv" "$HOME/.zshenv"
    copy_home_manager_file "$host_home/.zprofile" "$HOME/.zprofile"

    mkdir -p "$HOME/.config"
    copy_home_manager_dir "$host_home/.config/direnv" "$HOME/.config/direnv"
    copy_home_manager_dir "$host_home/.config/nix" "$HOME/.config/nix"
    copy_home_manager_dir "$host_home/.config/nvim" "$HOME/.config/nvim"
    copy_home_manager_file "$host_home/.config/starship.toml" "$HOME/.config/starship.toml"
    copy_home_manager_dir "$host_home/.config/zellij" "$HOME/.config/zellij"
    copy_home_manager_dir "$host_home/.config/zsh" "$HOME/.config/zsh"
fi

ensure_neovim
