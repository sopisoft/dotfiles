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
- `distrobox/`: optional ROS Jazzy container definition
- `udev/`: optional host udev rules

The canonical input method is Fcitx5 with Hazkey as the default Japanese input
method. Mozc remains available as a fallback. Hazkey, fonts, desktop packages,
shell tools, Neovim, and application configuration are installed through Nix
and Home Manager. The remote desktop path is the host distribution's xrdp on
top of an X11 XFCE session so Windows can connect with the built-in Remote
Desktop client.

## Requirements

- Linux, either native or WSL
- a regular user account
- multi-user Nix with flakes enabled
- `sudo` only for optional udev rules, system configuration, or device access

Ubuntu, Fedora, and other distributions are supported because the user-level
packages and configuration are managed by Nix/Home Manager rather than a
distribution package manager. xrdp still needs system services and PAM under
`/etc`, so `dotfiles` manages a small integration layer there when privileges
are available. Unlike normal user packages, xrdp is intentionally installed
from the host distribution so PAM, systemd, and Xorg modules match the host.

## Install

From the repository root:

```bash
curl --proto '=https' --tlsv1.2 -L https://nixos.org/nix/install | sh -s -- --daemon
```

Open a new login shell, then apply the default user configuration:

```bash
nix run path:.#dotfiles -- install --skip-ros-jazzy
```

The default configuration is `homeConfigurations.sopi`. Explicit variants
are also available:

```bash
nix run github:nix-community/home-manager -- switch --flake .#sopi-native
nix run github:nix-community/home-manager -- switch --flake .#sopi-wsl
```

`dotfiles install` and `dotfiles update` also apply the xrdp system integration
when `sudo` is available. This installs:

- host packages for `xrdp` and `xorgxrdp` when missing
- `/etc/xrdp/startwm.sh`, `session-env.sh`, `reconnectwm.sh`, `sesman.ini`, `xrdp.ini`, and WSL-only XFCE panel defaults
- `/etc/pam.d/xrdp-sesman` using the host distribution's standard PAM stack
- `/etc/X11/xrdp/xorg.conf` for xorgxrdp startup
- `xrdp-sesman.service` and `xrdp.service`

The managed session is XFCE with GNOME-like theming, Hazkey/Fcitx defaults, and
an X11-only startup path. Windows clients should connect to `localhost:3390`
with the built-in Remote Desktop client. Port `3390` is used to avoid
colliding with Windows' own RDP listener on `3389`. Use the Linux account name
and password for the managed user, for example `sopi`.

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

`dotfiles install-hazkey` is retained for compatibility. It now reapplies the
Home Manager generation, which installs Hazkey and the canonical Fcitx5 profile
without using a distribution-specific installer.

## Optional ROS environment

The ROS container is optional and is skipped by the default install command.
To enable it:

```bash
dotfiles install-ros-jazzy
dotfiles jazzy
```

Container creation may require rootless container support from the host. If
the host does not provide it, continue using the user configuration with
`--skip-ros-jazzy`.

## NixOS

For machines running NixOS itself, the flake includes reusable Native and WSL
system configurations:

```bash
sudo nixos-rebuild switch --flake .#disk2-desktop
sudo nixos-rebuild switch --flake .#wsl-desktop
```

Both configurations share the same locale, fonts, Fcitx5/Hazkey setup, audio
stack, user defaults, and the xrdp-backed XFCE desktop module.

## Backups and udev

Home Manager collision backups are stored under
`~/.local/state/dotfiles/backups/generations/`. The retention limit defaults to
10 and can be changed with `DOTFILES_BACKUP_LIMIT`.

Managed udev rules live in `udev/rules.d/`. Applying them requires privileges;
this step can be skipped on systems where udev is not available.
