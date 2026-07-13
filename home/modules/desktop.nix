{...}: let
  defaults = import ../../config/defaults.nix;
in {
  home.sessionVariables = {
    DefaultIMModule = defaults.inputMethod.framework;
    FCITX_ADDON_DIRS = "$HOME/.nix-profile/lib/fcitx5:$HOME/.nix-profile/lib/x86_64-linux-gnu/fcitx5";
    FCITX_DATA_DIRS = "$HOME/.nix-profile/share/fcitx5:$FCITX_DATA_DIRS";
    GLFW_IM_MODULE = defaults.inputMethod.framework;
    GTK_IM_MODULE = defaults.inputMethod.framework;
    QT_IM_MODULE = defaults.inputMethod.framework;
    SDL_IM_MODULE = defaults.inputMethod.framework;
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
  xdg.configFile."plasma-workspace/env/fcitx5.sh" = {
    source = ../../config/fcitx5/environment.sh;
    executable = true;
  };

  home.file.".local/bin/start-fcitx5" = {
    source = ../../config/fcitx5/start-fcitx5.sh;
    executable = true;
  };

  xdg.configFile."autostart/fcitx5.desktop".source = ../../config/fcitx5/autostart.desktop;

  xdg.configFile."fcitx5/profile".source = ../../config/fcitx5/profile;
}
