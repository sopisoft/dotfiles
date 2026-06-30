{ ... }: {
  xdg.userDirs = {
    enable = true;
    createDirectories = true;
    setSessionVariables = true;
    desktop = "$HOME/Desktop";
    documents = "$HOME/Documents";
    download = "$HOME/Downloads";
    music = "$HOME/Music";
    pictures = "$HOME/Pictures";
    publicShare = "$HOME/Public";
    templates = "$HOME/Templates";
    videos = "$HOME/Videos";
  };

  xdg.terminal-exec = {
    enable = true;
    settings = {
      default = [ "Alacritty.desktop" ];
      GNOME = [ "Alacritty.desktop" ];
    };
  };

  dconf.settings."org/gnome/desktop/default-applications/terminal" = {
    exec = "x-terminal-emulator";
    exec-arg = "--";
  };

  xdg.configFile."user-dirs.locale".text = "en_US\n";
}
