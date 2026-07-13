{pkgs, ...}: let
  defaults = import ../../config/defaults.nix;
in {
  services.xserver = {
    enable = true;
    xkb.layout = defaults.keyboard.xkb;
    desktopManager.xfce.enable = true;
  };

  services.xrdp = {
    enable = true;
    openFirewall = true;
    defaultWindowManager = "${pkgs.xfce4-session}/bin/xfce4-session";
  };

  networking.firewall.allowedTCPPorts = [3390];

  environment.etc = {
    "xrdp/session-env.sh" = {
      source = ../../config/xrdp/session-env.sh;
      mode = "0755";
    };
    "xrdp/startwm.sh" = {
      source = ../../config/xrdp/startwm.sh;
      mode = "0755";
    };
    "xrdp/reconnectwm.sh" = {
      source = ../../config/xrdp/reconnectwm.sh;
      mode = "0755";
    };
  };

  environment.systemPackages = with pkgs; [
    adwaita-icon-theme
    dconf
    exo
    gnome-themes-extra
    papirus-icon-theme
    thunar
    xfce4-panel
    xfce4-session
    xfce4-settings
    xfce4-whiskermenu-plugin
    xfconf
    xfdesktop
    xfwm4
    xrdp
  ];
}
