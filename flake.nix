{
  description = "sopi dotfiles";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, home-manager, ... }:
    let
      system = "x86_64-linux";
      username = "sopi";
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = false;
      };
    in {
      homeConfigurations.${username} = home-manager.lib.homeManagerConfiguration {
        inherit pkgs;
        extraSpecialArgs = {
          inherit username;
        };
        modules = [
          ./home/home.nix
          {
            home = {
              username = username;
              homeDirectory = "/home/${username}";
            };
          }
        ];
      };
    };
}

