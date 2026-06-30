{ ... }: {
  home.stateVersion = "25.05";

  programs.home-manager.enable = true;

  xdg.enable = true;

  fonts.fontconfig = {
    enable = true;
    defaultFonts = {
      monospace = [ "UDEV Gothic NFLG" ];
    };
  };

  home.sessionPath = [
    "$HOME/.local/bin"
    "$HOME/.cargo/bin"
  ];

  home.sessionVariables = {
    EDITOR = "nvim";
    VISUAL = "nvim";
    TERMINAL = "alacritty";
  };
}
