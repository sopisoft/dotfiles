{ pkgs, xtaskPackage, ... }: {
  home.file = {
    ".zshenv".source = ../../zsh/.zshenv;
    ".zprofile".source = ../../zsh/.zprofile;
    ".zshrc".source = ../../zsh/.zshrc;
    ".local/bin/xtask".source = "${xtaskPackage}/bin/xtask";
    ".local/bin/vim".source = "${pkgs.neovim}/bin/nvim";
    ".local/bin/x-terminal-emulator".source = "${pkgs.alacritty}/bin/alacritty";
  };

  xdg.configFile."zsh" = {
    source = ../../zsh/config;
    recursive = true;
  };
}
