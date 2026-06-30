{ pkgs, ... }: {
  home.packages = with pkgs; [
    alacritty
    curl
    distrobox
    fd
    fontconfig
    git
    neovim
    podman
    ripgrep
    udev-gothic-nf
    unzip
    zsh
  ];
}
