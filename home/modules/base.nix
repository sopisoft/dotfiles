{...}: let
  defaults = import ../../config/defaults.nix;
in {
  home.stateVersion = "25.05";

  programs.home-manager.enable = true;

  xdg.enable = true;

  fonts.fontconfig = {
    enable = true;
    defaultFonts = {
      inherit (defaults.fonts) sansSerif serif monospace;
    };
  };

  home.sessionPath = [
    "$HOME/.local/bin"
    "$HOME/.cargo/bin"
  ];

  home.sessionVariables = {
    CARGO_HOME = "$HOME/.cargo";
    DOTFILES_DIR = "$HOME/dotfiles";
    EDITOR = "nvim";
    VISUAL = "nvim";
    TERMINAL = "alacritty";
  };
}
