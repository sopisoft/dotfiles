{
  pkgs,
  dotfilesPackage,
  ...
}: {
  home.file = {
    ".zshenv".source = ../../zsh/.zshenv;
    ".zprofile".source = ../../zsh/.zprofile;
    ".zshrc".source = ../../zsh/.zshrc;
    ".local/bin/dotfiles".source = "${dotfilesPackage}/bin/dotfiles";
    ".local/bin/vim".source = "${pkgs.neovim}/bin/nvim";
  };

  xdg.configFile."zsh" = {
    source = ../../zsh/config;
    recursive = true;
  };
}
