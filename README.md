# dotfiles

Nix Flake and Home Manager configuration for a Linux workstation. The user
configuration is distribution-independent and can be used on a native Linux
installation or inside WSL. NixOS system configurations are provided as an
optional reference for machines that use NixOS directly.

## Structure

- `flake.nix`: packages, Home Manager configurations, and NixOS configurations
- `config/`: canonical application configuration files and shared defaults
- `home/`: Home Manager modules
- `nixos/`: reusable NixOS modules and machine configurations
- `src/`: the `dotfiles` task runner
- `udev/`: optional host udev rules

The canonical input method is Fcitx5 with Hazkey as the default Japanese input
method. Mozc remains available as a fallback. Hazkey, fonts, desktop packages,
shell tools, Neovim, and application configuration are installed through Nix
and Home Manager. Remote desktop access is provided by a Home Manager managed
VNC server on top of the existing X11 XFCE session.

## Requirements

- Linux, either native or WSL
- a regular user account
- multi-user Nix with flakes enabled
- `sudo` only for optional udev rules, system configuration, or device access

## Install

From the repository root:

```bash
curl --proto '=https' --tlsv1.2 -L https://nixos.org/nix/install | sh -s -- --daemon
```

Open a new login shell, then apply the default user configuration:

```bash
nix run path:.#dotfiles -- install
```

The default configuration is `homeConfigurations.sopi`. Explicit variants
are also available:

```bash
nix run github:nix-community/home-manager -- switch --flake .#sopi-native
nix run github:nix-community/home-manager -- switch --flake .#sopi-wsl
```

`dotfiles install` and `dotfiles update` apply the Home Manager configuration
that installs and starts the VNC server. The VNC server shares the active X11
XFCE session, so any VNC client can connect to port `5900`.

## Daily commands

```bash
dotfiles switch
dotfiles update
dotfiles healthcheck
dotfiles cleanup
dotfiles remote status
dotfiles backups list
dotfiles rollback [generation]
dotfiles udev apply
dotfiles udev status
```

## NixOS

For machines running NixOS itself, the flake includes reusable Native and WSL
system configurations:

```bash
sudo nixos-rebuild switch --flake .#native-desktop
sudo nixos-rebuild switch --flake .#wsl-desktop
```

## Backups

Home Manager collision backups are stored under
`~/.local/state/dotfiles/backups/generations/`. The retention limit defaults to
10 and can be changed with `DOTFILES_BACKUP_LIMIT`.
