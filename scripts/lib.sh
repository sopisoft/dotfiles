#!/usr/bin/env bash

DOTFILES_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NIX_CONF_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/nix"
NIX_CONF_FILE="${NIX_CONF_DIR}/nix.conf"
NIX_MAX_JOBS="${NIX_MAX_JOBS:-1}"
NIX_BUILD_CORES="${NIX_BUILD_CORES:-1}"
NIX_EVAL_CORES="${NIX_EVAL_CORES:-1}"

backup_ext() {
    date +"hm-backup-%Y%m%d%H%M%S"
}

ensure_nix_command() {
    if command -v nix >/dev/null 2>&1; then
        return
    fi

    curl -fsSL https://install.determinate.systems/nix | sh -s -- install --no-confirm
}

source_nix_environment() {
    local candidate

    for candidate in \
        /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh \
        "$HOME/.nix-profile/etc/profile.d/nix.sh"
    do
        if [[ -r "$candidate" ]]; then
            # shellcheck disable=SC1090
            source "$candidate"
            return
        fi
    done
}

write_nix_conf() {
    mkdir -p "$NIX_CONF_DIR"
    rm -f "$NIX_CONF_FILE"
    cp "$DOTFILES_DIR/config/nix/nix.conf" "$NIX_CONF_FILE"
}

apply_home_manager() {
    local backup
    local nix_config_lines
    local activation_package
    backup="$(backup_ext)"
    nix_config_lines="experimental-features = nix-command flakes
accept-flake-config = true
max-jobs = ${NIX_MAX_JOBS}
cores = ${NIX_BUILD_CORES}
eval-cores = ${NIX_EVAL_CORES}"

    if [[ -n "${NIX_CONFIG:-}" ]]; then
        export NIX_CONFIG="${NIX_CONFIG}"$'\n'"${nix_config_lines}"
    else
        export NIX_CONFIG="${nix_config_lines}"
    fi

    activation_package="$(
        nix build "${DOTFILES_DIR}#homeConfigurations.sopi.activationPackage" \
            --no-link \
            --print-out-paths
    )"

    HOME_MANAGER_BACKUP_EXT="$backup" "${activation_package}/activate"
}
