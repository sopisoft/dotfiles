{...}: {
  programs.direnv = {
    enable = true;
    enableZshIntegration = false;
    nix-direnv.enable = true;
    config = builtins.fromTOML (builtins.readFile ../../config/direnv/direnv.toml);
  };

  programs.starship = {
    enable = true;
    enableZshIntegration = false;
    settings = builtins.fromTOML (builtins.readFile ../../config/starship.toml);
  };

  programs.zellij = {
    enable = true;
    enableZshIntegration = false;
  };

  xdg.configFile = {
    "alacritty" = {
      source = ../../config/alacritty;
      recursive = true;
    };

    "zellij" = {
      source = ../../config/zellij;
      recursive = true;
    };

    "nix/nix.conf".source = ../../config/nix/nix.conf;
  };
}
