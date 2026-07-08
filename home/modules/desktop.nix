{...}: {
  home.sessionVariables = {
    GTK_IM_MODULE = "fcitx";
    QT_IM_MODULE = "fcitx";
    SDL_IM_MODULE = "fcitx";
    XMODIFIERS = "@im=fcitx";
  };

  dconf.settings = {
    "org/gnome/settings-daemon/plugins/media-keys" = {
      terminal = ["<Primary><Alt>t"];
    };
  };

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
      default = ["Alacritty.desktop"];
      GNOME = ["Alacritty.desktop"];
    };
  };

  xdg.configFile."user-dirs.locale".text = "en_US\n";

  xdg.configFile."autostart/fcitx5.desktop".text = ''
    [Desktop Entry]
    Type=Application
    Name=Fcitx 5
    Exec=fcitx5 -d --replace
    X-GNOME-Autostart-enabled=true
    NoDisplay=true
    Hidden=false
    OnlyShowIn=GNOME;KDE;X-Cinnamon;LXDE;LXQt;MATE;XFCE;
  '';
}
