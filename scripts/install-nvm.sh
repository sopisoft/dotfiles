#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOTFILES_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
NVM_DIR="${NVM_DIR:-$HOME/.local/share/nvm}"
NODE_VERSION="${NODE_VERSION:-lts/*}"
NVM_REPO_URL="${NVM_REPO_URL:-https://github.com/nvm-sh/nvm.git}"
NVM_WRAPPER_SOURCE="${DOTFILES_DIR}/bin/nvm-tool"

install_nvm() {
    if [[ -s "${NVM_DIR}/nvm.sh" ]]; then
        return
    fi

    mkdir -p "$(dirname "${NVM_DIR}")"
    rm -rf "${NVM_DIR}"
    git clone "${NVM_REPO_URL}" "${NVM_DIR}"

    local latest_tag
    latest_tag="$(git -C "${NVM_DIR}" tag --sort=-version:refname | head -n 1)"
    if [[ -z "${latest_tag}" ]]; then
        echo "Could not determine the latest nvm tag." >&2
        exit 1
    fi

    git -C "${NVM_DIR}" checkout "${latest_tag}" >/dev/null 2>&1
}

load_nvm() {
    # shellcheck disable=SC1091
    source "${NVM_DIR}/nvm.sh"
}

install_node() {
    nvm install "${NODE_VERSION}"
    nvm alias default "${NODE_VERSION}" >/dev/null
    nvm use default >/dev/null
}

install_wrapper() {
    local tool_name="$1"
    local wrapper_path="${HOME}/.local/bin/${tool_name}"
    local wrapper_tmp

    mkdir -p "${HOME}/.local/bin"
    wrapper_tmp="$(mktemp)"

    cp "${NVM_WRAPPER_SOURCE}" "${wrapper_tmp}"
    chmod 755 "${wrapper_tmp}"
    mv "${wrapper_tmp}" "${wrapper_path}"
}

install_nvm
load_nvm
install_node
install_wrapper node
install_wrapper npm
install_wrapper npx
