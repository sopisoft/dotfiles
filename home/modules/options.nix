{lib, ...}: {
  options.sopi = {
    environment = lib.mkOption {
      type = lib.types.enum ["native" "wsl"];
      default = "native";
      description = "Execution environment for optional desktop integrations.";
    };

    syncNvimPack = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Synchronize Neovim packages during Home Manager activation.";
    };
  };
}
