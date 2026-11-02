#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

for command in bash curl git; do
    if ! command -v "$command" >/dev/null 2>&1; then
        echo "missing prerequisite: $command (install it with the host OS package manager)" >&2
        exit 1
    fi
done

if ! command -v brew >/dev/null 2>&1 && ! command -v sudo >/dev/null 2>&1; then
    echo "missing prerequisite: sudo (required only for the standard Homebrew bootstrap)" >&2
    exit 1
fi

brew_bin="$(command -v brew || true)"
if [[ -z "$brew_bin" ]]; then
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    for candidate in /home/linuxbrew/.linuxbrew/bin/brew /opt/homebrew/bin/brew; do
        if [[ -x "$candidate" ]]; then
            brew_bin="$candidate"
            break
        fi
    done
fi
if [[ -z "$brew_bin" ]]; then
    echo "Homebrew was not found after installation" >&2
    exit 1
fi

eval "$("$brew_bin" shellenv)"
brew tap "Giammarco-Ferranti/deja"
brew trust --formula "giammarco-ferranti/deja/deja"
brew bundle --file="$repo_root/brew/Brewfile"

mise trust --yes "$repo_root/mise.toml"
mise install
if [[ -f "$repo_root/pixi.toml" ]]; then
    pixi install --manifest-path "$repo_root/pixi.toml"
fi

mise exec -- cargo install --path "$repo_root" --root "$HOME/.local" --locked
"$HOME/.local/bin/dotfiles" deploy
