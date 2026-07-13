{
  description = "sopi dotfiles";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixos-wsl = {
      url = "github:nix-community/NixOS-WSL";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    home-manager,
    nixos-wsl,
    ...
  }: let
    lib = nixpkgs.lib;
    supportedSystems = ["x86_64-linux"];
    forAllSystems = lib.genAttrs supportedSystems;
    username = "sopi";
    mkPkgs = system:
      import nixpkgs {
        inherit system;
      };
  in {
    packages = forAllSystems (system: let
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
      hazkey = pkgs.callPackage ./pkgs/hazkey.nix {};
      fcitx5-hazkey = pkgs.callPackage ./pkgs/fcitx5-hazkey.nix {
        fcitx5Mozc = pkgs.fcitx5-mozc;
        hazkeyPackage = self.packages.${system}.hazkey;
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

    devShells = forAllSystems (system: let
      pkgs = mkPkgs system;
    in {
      default = pkgs.mkShell {
        packages = with pkgs; [
          alejandra
          cargo
          clippy
          rust-analyzer
          rustc
          rustfmt
        ];
      };
    });

    formatter = forAllSystems (system: (mkPkgs system).alejandra);

    nixosConfigurations.native-desktop = let
      system = "x86_64-linux";
    in
      nixpkgs.lib.nixosSystem {
        inherit system;
        specialArgs = {
          inherit username;
          hazkeyPackage = self.packages.${system}.hazkey;
        };
        modules = [
          ./nixos/native-desktop.nix
          home-manager.nixosModules.home-manager
          {
            home-manager.useGlobalPkgs = true;
            home-manager.useUserPackages = true;
            home-manager.extraSpecialArgs = {
              inherit username;
              dotfilesPackage = self.packages.${system}.dotfiles;
              hazkeyPackage = self.packages.${system}.hazkey;
              fcitx5HazkeyPackage = self.packages.${system}.fcitx5-hazkey;
            };
            home-manager.users.${username} = import ./home/home.nix;
          }
        ];
      };

    nixosConfigurations.wsl-desktop = let
      system = "x86_64-linux";
    in
      nixpkgs.lib.nixosSystem {
        inherit system;
        specialArgs = {
          inherit username;
          hazkeyPackage = self.packages.${system}.hazkey;
        };
        modules = [
          nixos-wsl.nixosModules.default
          ./nixos/wsl-desktop.nix
          home-manager.nixosModules.home-manager
          {
            home-manager.useGlobalPkgs = true;
            home-manager.useUserPackages = true;
            home-manager.extraSpecialArgs = {
              inherit username;
              dotfilesPackage = self.packages.${system}.dotfiles;
              hazkeyPackage = self.packages.${system}.hazkey;
              fcitx5HazkeyPackage = self.packages.${system}.fcitx5-hazkey;
            };
            home-manager.users.${username} = {
              imports = [./home/home.nix];
              sopi.syncNvimPack = false;
            };
          }
        ];
      };

    homeConfigurations = let
      system = "x86_64-linux";
      pkgs = mkPkgs system;
      mkHome = environment:
        home-manager.lib.homeManagerConfiguration {
          inherit pkgs;
          extraSpecialArgs = {
            inherit username;
            dotfilesPackage = self.packages.${system}.dotfiles;
            hazkeyPackage = self.packages.${system}.hazkey;
            fcitx5HazkeyPackage = self.packages.${system}.fcitx5-hazkey;
          };
          modules = [
            ./home/home.nix
            {
              sopi.environment = environment;
              home = {
                username = username;
                homeDirectory = "/home/${username}";
              };
            }
          ];
        };
    in {
      ${username} = mkHome "native";
      "${username}-native" = mkHome "native";
      "${username}-wsl" = mkHome "wsl";
    };
  };
}
