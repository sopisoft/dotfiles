{
  pkgs,
  username,
  ...
}: {
  imports = [
    ./hardware-disk2.nix
    ./modules/base.nix
    ./modules/xrdp-xfce.nix
  ];

  boot.loader = {
    systemd-boot = {
      enable = true;
      editor = false;
      configurationLimit = 10;
    };
    efi = {
      canTouchEfiVariables = false;
      efiSysMountPoint = "/boot";
    };
  };

  networking = {
    hostName = "nixos-disk2";
    networkmanager.enable = true;
  };

  services.displayManager = {
    defaultSession = "xfce";
    autoLogin = {
      enable = true;
      user = username;
    };
  };

  users.users.${username}.extraGroups = ["networkmanager"];

  environment.systemPackages = with pkgs; [
    pciutils
    usbutils
  ];

  virtualisation.podman = {
    enable = true;
    dockerCompat = true;
    defaultNetwork.settings.dns_enabled = true;
  };
}
