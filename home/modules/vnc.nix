{...}: {
  home.file.".local/bin/start-vnc-session" = {
    source = ../../config/vnc/start-vnc-session.sh;
    executable = true;
  };

  xdg.configFile."autostart/x11vnc.desktop".source = ../../config/vnc/x11vnc.desktop;
}
