{
  lib,
  pkgs,
  username,
  hazkeyPackage,
  ...
}: let
  defaults = import ../../config/defaults.nix;
in {
  system.stateVersion = "26.05";

  nix.settings = {
    experimental-features = ["nix-command" "flakes"];
    accept-flake-config = true;
  };

  time.timeZone = defaults.timezone;
  i18n = {
    defaultLocale = defaults.locale;
    extraLocales = ["ja_JP.UTF-8/UTF-8"];
    inputMethod = {
      enable = true;
      type = "fcitx5";
      fcitx5.addons = with pkgs; [
        fcitx5-gtk
        fcitx5-mozc
        qt6Packages.fcitx5-qt
        hazkeyPackage
      ];
    };
  };

  console.keyMap = defaults.keyboard.console;

  services = {
    dbus.enable = true;
    openssh.enable = true;
    pulseaudio.enable = false;
  };

  security.rtkit.enable = true;
  services.pipewire = {
    enable = true;
    alsa.enable = true;
    alsa.support32Bit = true;
    pulse.enable = true;
    wireplumber.enable = true;
  };

  hardware.graphics.enable = true;

  programs = {
    zsh.enable = true;
    firefox.enable = true;
  };

  users.users.${username} = {
    isNormalUser = true;
    description = username;
    initialPassword = "nixos";
    shell = pkgs.zsh;
    extraGroups = [
      "wheel"
      "audio"
      "video"
      "dialout"
    ];
  };

  environment.systemPackages = with pkgs; [
    alacritty
    curl
    git
    home-manager
    hazkeyPackage
    neovim
    pavucontrol
    qt6Packages.fcitx5-configtool
    wget
    xauth
  ];

  environment.pathsToLink = [
    "/lib/fcitx5"
    "/lib/x86_64-linux-gnu/fcitx5"
    "/share/fcitx5"
  ];

  fonts.packages = with pkgs; [
    noto-fonts
    noto-fonts-cjk-sans
    noto-fonts-cjk-serif
    noto-fonts-color-emoji
    udev-gothic-nf
  ];
  fonts.fontconfig.defaultFonts = {
    inherit (defaults.fonts) sansSerif serif monospace;
  };
}
