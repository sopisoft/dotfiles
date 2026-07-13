{
  config,
  lib,
  pkgs,
  dotfilesPackage,
  ...
}: {
  config = {
    home.file = {
      ".config/nvim/init.lua".source = ../../config/nvim/init.lua;

      ".config/nvim/lua" = {
        source = ../../config/nvim/lua;
        recursive = true;
      };

      ".config/nvim/nvim-pack-lock.json".source = ../../config/nvim/nvim-pack-lock.json;
    };

    home.activation.syncNvimPack = lib.mkIf config.sopi.syncNvimPack (
      lib.hm.dag.entryAfter ["linkGeneration"] ''
        PATH=${lib.makeBinPath [pkgs.git pkgs.neovim]}:$PATH \
          NVIM_BIN=${pkgs.neovim}/bin/nvim \
          ${dotfilesPackage}/bin/dotfiles sync-nvim-pack
      ''
    );
  };
}
