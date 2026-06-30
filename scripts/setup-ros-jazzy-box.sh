#!/usr/bin/env bash
set -euo pipefail

export DEBIAN_FRONTEND=noninteractive
export PATH="$HOME/.local/bin:$PATH"

container_dotfiles="$HOME/.local/share/dotfiles/host"
host_dotfiles="${DOTFILES_DIR:+/run/host${DOTFILES_DIR}}"

missing_packages=()
for package in ca-certificates curl git sudo zsh; do
    if ! dpkg -s "$package" >/dev/null 2>&1; then
        missing_packages+=("$package")
    fi
done

if (( ${#missing_packages[@]} > 0 )); then
    sudo apt-get update
    sudo apt-get install -y "${missing_packages[@]}"
fi

if [[ -r "$HOME/.config/zsh/lib/distrobox-init-hook.sh" ]]; then
    sh "$HOME/.config/zsh/lib/distrobox-init-hook.sh"
fi

if ! dpkg -s ros-jazzy-ros-base >/dev/null 2>&1; then
    sudo apt-get update
    sudo apt-get install -y \
        direnv \
        gnupg \
        locales \
        python3-colcon-common-extensions \
        python3-rosdep \
        python3-vcstool \
        software-properties-common \
        zsh

    sudo locale-gen en_US.UTF-8
    sudo update-locale LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8
    sudo add-apt-repository -y universe
    sudo install -d -m 0755 /etc/apt/keyrings
    sudo curl -fsSL https://raw.githubusercontent.com/ros/rosdistro/master/ros.key \
        -o /etc/apt/keyrings/ros-archive-keyring.asc
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/ros-archive-keyring.asc] http://packages.ros.org/ros2/ubuntu $(. /etc/os-release && echo "${UBUNTU_CODENAME}") main" \
        | sudo tee /etc/apt/sources.list.d/ros2.list >/dev/null
    sudo apt-get update
    sudo apt-get install -y ros-jazzy-ros-base
fi

if [[ ! -x "$HOME/.local/bin/npm" ]]; then
    if [[ -x "$container_dotfiles/scripts/install-nvm.sh" ]]; then
        bash "$container_dotfiles/scripts/install-nvm.sh"
    elif [[ -n "${host_dotfiles:-}" && -x "$host_dotfiles/scripts/install-nvm.sh" ]]; then
        bash "$host_dotfiles/scripts/install-nvm.sh"
    fi
fi

if [[ ! -f /etc/ros/rosdep/sources.list.d/20-default.list ]]; then
    sudo rosdep init || true
fi

rosdep update || true

if [[ -r "$HOME/.config/zsh/lib/distrobox-init-hook.sh" ]]; then
    sh "$HOME/.config/zsh/lib/distrobox-init-hook.sh"
fi
