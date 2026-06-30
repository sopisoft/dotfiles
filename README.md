# dotfiles

Home Manager and flake-based dotfiles for an Ubuntu host, with a `distrobox` workflow for ROS 2 Jazzy.

## Layout

- [`bin/`](./bin) Small user-facing wrapper commands tracked as real files
- [`config/`](./config) Application config for `alacritty`, `direnv`, `nix`, `nvim`, `starship`, and `zellij`
- [`home/`](./home) Home Manager modules
- [`scripts/`](./scripts) Bootstrap, switch, update, and ROS box setup scripts
- [`zsh/`](./zsh) Shell startup files and `distrobox` helpers

## Bootstrap

```bash
cd ~/dotfiles
./scripts/bootstrap.sh
```

`bootstrap.sh` runs these steps in order:

1. Install required Ubuntu/Debian host packages such as `distrobox` and `podman`, unless disabled.
2. Install Determinate Nix if `nix` is not available yet.
3. Write `~/.config/nix/nix.conf`.
4. Apply the Home Manager configuration.
5. Sync Neovim `vim.pack` plugins.
6. Install `nvm` and a default Node.js LTS release.
7. Create and initialize the `ros-jazzy` distrobox when enabled.

Skip host package installation:

```bash
SKIP_HOST_PACKAGES=1 ./scripts/bootstrap.sh
```

Constrain build parallelism more aggressively:

```bash
NIX_MAX_JOBS=1 NIX_BUILD_CORES=1 ./scripts/bootstrap.sh
```

Constrain both build and evaluation parallelism:

```bash
NIX_MAX_JOBS=1 NIX_BUILD_CORES=1 NIX_EVAL_CORES=1 ./scripts/bootstrap.sh
```

## Daily Use

Apply the current configuration:

```bash
./scripts/switch.sh
```

`switch.sh` builds the flake activation package directly instead of invoking the Home Manager CLI through `nix run`.

Sync only Neovim `vim.pack` plugins:

```bash
./scripts/sync-nvim-pack.sh
```

Update flake-pinned sources:

```bash
./scripts/update-sources.sh
```

Update only Neovim plugin pins:

```bash
./scripts/update-pack-plugins.sh
```

## Notes

- Home Manager backs up unmanaged conflicting files as `*.hm-backup-*`.
