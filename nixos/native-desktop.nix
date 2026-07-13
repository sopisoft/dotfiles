{
  pkgs,
  username,
  ...
}: {
  imports = [
    ./native-filesystems.nix
    ./modules/base.nix
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
    hostName = "nixos-native";
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
