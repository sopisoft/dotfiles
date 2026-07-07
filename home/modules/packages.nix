{ pkgs, dotfilesPackage, ... }: {
  home.packages = with pkgs; [
    alacritty
    cargo
    clippy
    curl
    distrobox
    fd
    fontconfig
    git
    neovim
    nodejs
    podman
    ripgrep
    rust-analyzer
    rustc
    rustfmt
    tmux
    udev-gothic-nf
    unzip
    dotfilesPackage
    zsh
  ];
}
