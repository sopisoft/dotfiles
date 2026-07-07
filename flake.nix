{
  description = "sopi dotfiles";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, home-manager, ... }:
    let
      lib = nixpkgs.lib;
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = lib.genAttrs supportedSystems;
      username = "sopi";
      mkPkgs = system:
        import nixpkgs {
          inherit system;
          config.allowUnfree = false;
        };
    in {
      packages = forAllSystems (system:
        let
          pkgs = mkPkgs system;
        in {
          dotfiles = pkgs.pkgsStatic.rustPlatform.buildRustPackage {
            pname = "dotfiles";
            version = "0.1.0";
            src = pkgs.lib.sourceByRegex ./. [
              "^Cargo\\.lock$"
              "^Cargo\\.toml$"
              "^src(/.*)?$"
            ];
            cargoLock.lockFile = ./Cargo.lock;
            doCheck = false;
          };
          default = self.packages.${system}.dotfiles;
        });

      apps = forAllSystems (system: {
        dotfiles = {
          type = "app";
          program = "${self.packages.${system}.dotfiles}/bin/dotfiles";
        };
        default = self.apps.${system}.dotfiles;
      });

      devShells = forAllSystems (system:
        let
          pkgs = mkPkgs system;
        in {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              clippy
              rust-analyzer
              rustc
              rustfmt
            ];
          };
        });

      homeConfigurations.${username} =
        let
          system = "x86_64-linux";
          pkgs = mkPkgs system;
        in
          home-manager.lib.homeManagerConfiguration {
            inherit pkgs;
            extraSpecialArgs = {
              inherit username;
              dotfilesPackage = self.packages.${system}.dotfiles;
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
