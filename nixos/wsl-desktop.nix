{
  lib,
  username,
  ...
}: {
  imports = [
    ./modules/base.nix
  ];

  wsl = {
    enable = true;
    defaultUser = username;
    startMenuLaunchers = true;
    wslConf = {
      network = {
        hostname = "nixos-wsl";
        generateHosts = true;
        generateResolvConf = true;
      };
      interop.appendWindowsPath = false;
    };
  };

  networking = {
    hostName = "nixos-wsl";
    networkmanager.enable = lib.mkForce false;
    wireless.enable = lib.mkForce false;
  };

  systemd.services."getty@tty1".enable = lib.mkForce false;
}
