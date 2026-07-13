{...}: let
  defaults = import ../../config/defaults.nix;
in {
  home.sessionVariables = {
    GTK_IM_MODULE = defaults.inputMethod.framework;
    QT_IM_MODULE = defaults.inputMethod.framework;
    XMODIFIERS = "@im=${defaults.inputMethod.framework}";
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
      KDE = ["Alacritty.desktop"];
    };
  };

  xdg.configFile."user-dirs.locale".source = ../../config/desktop/user-dirs.locale;
  xdg.configFile."dbus-1/session.conf".source = ../../config/desktop/dbus-session.conf;

  xdg.configFile."autostart/org.fcitx.Fcitx5.desktop".source =
    ../../config/fcitx5/org.fcitx.Fcitx5.desktop;

  xdg.configFile."fcitx5/profile".source = ../../config/fcitx5/profile;

  xdg.configFile."xfce4/xfconf/xfce-perchannel-xml/xfce4-keyboard-shortcuts.xml".source =
    ../../config/desktop/xfce4-keyboard-shortcuts.xml;
}
