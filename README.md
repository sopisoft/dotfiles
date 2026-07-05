# dotfiles

Dotfiles with Home Manager.

## Layout

- `flake.nix`, `flake.lock`: Nix entrypoint
- `Cargo.toml`, `Cargo.lock`, `src/`: Rust `xtask` task runner
- `.cargo/config.toml`, `rust-toolchain.toml`: Rust toolchain setup
- `home/`: Home Manager modules for host-side CLI tools and user config
- `config/`, `zsh/`: host-side application and shell configuration
- `distrobox/ros-jazzy.ini`: ROS Jazzy container definition
- `udev/rules.d/`: managed udev rules copied to `/etc/udev/rules.d/`

## Commands

Bootstrap after Nix is installed:

```bash
nix run path:.#xtask -- install
```

Daily commands:

```bash
xtask update
xtask rebuild
xtask enter
xtask healthcheck
xtask cleanup
xtask udev apply
xtask udev status
xtask backups list
xtask rollback [generation]
xtask install-ros-jazzy
xtask update-ros-jazzy
xtask update-flake-inputs
xtask update-neovim-plugins
xtask install-hazkey
```

## Initial Setup

Prerequisites:

- Ubuntu 26 host
- a regular user with `sudo`
- `dialout` group membership if you use USB serial devices

Install multi-user Nix first. The official Linux command is:

```bash
curl --proto '=https' --tlsv1.2 -L https://nixos.org/nix/install | sh -s -- --daemon
```

After the installer finishes, open a new login shell and run:

```bash
cd ~/dotfiles
nix run path:.#xtask -- install
```

`nix run path:.#xtask -- install` performs:

1. Host package installation with `apt` for `distrobox`, `podman`, build tools, and related dependencies
2. Home Manager activation
3. Host-side package provisioning, including Rust and Node.js through Nix
4. udev rule synchronization from `udev/rules.d/`
5. `ros-jazzy` container creation from `distrobox/ros-jazzy.ini`
6. ROS 2 Jazzy, `colcon`, and `rosdep` installation inside the container
7. `rosdep update`

Operations that require root privileges:

- host package installation with `apt-get`
- copying managed udev rules into `/etc/udev/rules.d/`
- container-side `apt`, `rosdep init`, and `/etc/profile.d` updates
- adding the user to `dialout`

## Daily Use

Re-apply only host-side configuration:

```bash
xtask rebuild
```

Update flake inputs, refresh Neovim plugins, re-apply Home Manager, re-apply udev rules, and update the ROS container:

```bash
xtask update
```

Enter the ROS environment:

```bash
xtask enter
```

The shell alias is also available:

```bash
ros
```

`ros` calls `xtask enter`, so both paths use the same container entry logic.

## Health Check

```bash
xtask healthcheck
```

It checks:

- host commands: `nix`, `xtask`, `cargo`, `rustc`, `node`, `distrobox`, `distrobox-assemble`, `podman`, `nvim`
- host `dialout` membership
- managed udev rule installation status
- presence of the `ros-jazzy` container
- container commands: `ros2`, `colcon`, `rosdep`, `nvim`, `distrobox-host-exec`
- host `git` invocation from inside the container
- host `nvim` invocation from inside the container
- visibility of `/dev/ttyUSB*` and `/dev/ttyACM*`

## udev Rules

Managed udev rules live in:

```text
udev/rules.d/
```

Rules must include `dotfiles-` in the filename. They are synchronized to:

```text
/etc/udev/rules.d/
```

Apply managed rules:

```bash
xtask udev apply
```

Check that installed rules match the repository copy:

```bash
xtask udev status
```

The default managed rule file is:

- `udev/rules.d/70-dotfiles-serial-access.rules`

It sets `dialout`, `0660`, and `uaccess` for `ttyUSB*` and `ttyACM*`.

## Backups and Rollback

Home Manager collision backups and imported legacy backups are stored in:

```text
~/.local/state/dotfiles/backups/generations/
```

- default retention: `10`
- override with: `DOTFILES_BACKUP_LIMIT=<n>`
- list generations: `xtask backups list`
- rollback latest generation: `xtask rollback`
- rollback a specific generation: `xtask rollback <generation>`

## Troubleshooting

If USB devices are not visible:

1. Check `id -nG` on the host and confirm `dialout` is present.
2. If it is missing, run `sudo usermod -aG dialout "$USER"` and log in again.
3. Check `ls /dev/ttyUSB* /dev/ttyACM* 2>/dev/null` on the host.
4. Run `xtask udev status`.
5. Run `xtask healthcheck`.
6. Add device-specific rules under `udev/rules.d/` if the generic serial rules are not enough.
7. Check `systemctl --user status podman.socket` if rootless Podman is not working.

To recreate the container:

```bash
ROS_JAZZY_BOX_REPLACE=1 xtask install-ros-jazzy
```

## Recommended Workflow

- Use host-side Neovim to edit the workspace.
- Open the workspace on the host filesystem.
- Run `colcon build`, `ros2 run`, and other ROS commands after `xtask enter`.
- Keep the editor on the host and the ROS runtime inside the container.
