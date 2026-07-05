{ lib, pkgs, xtaskPackage, ... }: {
  xdg.configFile."nvim" = {
    source = ../../config/nvim;
    recursive = true;
  };

  home.activation.syncNvimPack = lib.hm.dag.entryAfter [ "linkGeneration" ] ''
    PATH=${lib.makeBinPath [ pkgs.git pkgs.neovim ]}:$PATH \
      NVIM_BIN=${pkgs.neovim}/bin/nvim \
      ${xtaskPackage}/bin/xtask sync-nvim-pack
  '';
}
