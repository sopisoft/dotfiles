{ lib, pkgs, ... }:
let
  syncNvimPack = ../../scripts/sync-nvim-pack.sh;
  syncNvimPackPath = lib.makeBinPath [ pkgs.git pkgs.neovim pkgs.bash ];
in {
  xdg.configFile."nvim" = {
    source = ../../config/nvim;
    recursive = true;
  };

  home.activation.syncNvimPack = lib.hm.dag.entryAfter [ "linkGeneration" ] ''
    PATH=${syncNvimPackPath}:$PATH NVIM_BIN=${pkgs.neovim}/bin/nvim ${pkgs.bash}/bin/bash ${syncNvimPack}
  '';
}
