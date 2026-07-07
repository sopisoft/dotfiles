# dotfiles

Dotfiles with Home Manager.

## Layout

- `flake.nix`, `flake.lock`: Nix entrypoint
- `Cargo.toml`, `Cargo.lock`, `src/`: Rust `dotfiles` task runner
- `.cargo/config.toml`, `rust-toolchain.toml`: Rust toolchain setup
- `home/`: Home Manager modules for host-side CLI tools and user config
- `config/`, `zsh/`: host-side application and shell configuration
- `distrobox/ros-jazzy.ini`: ROS Jazzy container definition
- `udev/rules.d/`: managed udev rules copied to `/etc/udev/rules.d/`

## Commands

Bootstrap after Nix is installed:

```bash
nix run path:.#dotfiles -- install
```

Daily commands:

```bash
dotfiles update
dotfiles switch
dotfiles jazzy
dotfiles healthcheck
dotfiles cleanup
dotfiles udev apply
dotfiles udev status
dotfiles backups list
dotfiles rollback [generation]
dotfiles install-ros-jazzy
dotfiles update-ros-jazzy
dotfiles update-flake-inputs
dotfiles update-neovim-plugins
dotfiles install-hazkey
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
nix run path:.#dotfiles -- install
```

`nix run path:.#dotfiles -- install` performs:

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
dotfiles switch
```

Update flake inputs, refresh Neovim plugins, re-apply Home Manager, re-apply udev rules, and update the ROS container:

```bash
dotfiles update
```

Enter the ROS environment:

```bash
dotfiles jazzy
```

The shell alias is also available:

```bash
jazzy
```

`jazzy` calls `dotfiles jazzy`, so both paths use the same container entry logic.

## Health Check

```bash
dotfiles healthcheck
```

It checks:

- host commands: `nix`, `dotfiles`, `cargo`, `rustc`, `node`, `distrobox`, `distrobox-assemble`, `podman`, `nvim`
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
dotfiles udev apply
```

Check that installed rules match the repository copy:

```bash
dotfiles udev status
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
- list generations: `dotfiles backups list`
- rollback latest generation: `dotfiles rollback`
- rollback a specific generation: `dotfiles rollback <generation>`

## Troubleshooting

If USB devices are not visible:

1. Check `id -nG` on the host and confirm `dialout` is present.
2. If it is missing, run `sudo usermod -aG dialout "$USER"` and log in again.
3. Check `ls /dev/ttyUSB* /dev/ttyACM* 2>/dev/null` on the host.
4. Run `dotfiles udev status`.
5. Run `dotfiles healthcheck`.
6. Add device-specific rules under `udev/rules.d/` if the generic serial rules are not enough.
7. Check `systemctl --user status podman.socket` if rootless Podman is not working.

To recreate the container:

```bash
ROS_JAZZY_BOX_REPLACE=1 dotfiles install-ros-jazzy
```

## Recommended Workflow

- Use host-side Neovim to edit the workspace.
- Open the workspace on the host filesystem.
- Run `colcon build`, `ros2 run`, and other ROS commands after `dotfiles jazzy`.
- Keep the editor on the host and the ROS runtime inside the container.
