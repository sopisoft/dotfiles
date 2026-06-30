{ ... }: {
  home.file = {
    ".zshenv".source = ../../zsh/.zshenv;
    ".zprofile".source = ../../zsh/.zprofile;
    ".zshrc".source = ../../zsh/.zshrc;
    ".local/bin/vim".source = ../../bin/vim;
    ".local/bin/x-terminal-emulator".source = ../../bin/x-terminal-emulator;
  };

  xdg.configFile."zsh" = {
    source = ../../zsh/config;
    recursive = true;
  };
}
