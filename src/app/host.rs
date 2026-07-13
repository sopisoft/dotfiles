use super::{container, nvim, remote, system, udev};
use crate::backup;
use crate::context::{HostContext, ROS_JAZZY_CONTAINER_NAME};
use crate::home_manager;
use anyhow::{Context, Result};

pub fn install(
    context: &HostContext,
    _skip_host_packages: bool,
    skip_ros_jazzy: bool,
) -> Result<()> {
    require_nix(context)?;
    home_manager::apply_home_manager(context)?;
    remote::apply(context)?;
    udev::apply(context)?;
    if !skip_ros_jazzy {
        install_ros_jazzy(context)?;
    }
    Ok(())
}

pub fn update(context: &HostContext, skip_ros_jazzy: bool) -> Result<()> {
    require_nix(context)?;
    update_flake_inputs(context)?;
    nvim::update_neovim_plugins_as_target(context)?;
    switch(context)?;
    remote::apply(context)?;
    udev::apply(context)?;
    if skip_ros_jazzy {
        return Ok(());
    }
    if container::container_exists(context)? {
        update_ros_jazzy(context)
    } else {
        eprintln!("warning: ROS Jazzy distrobox is missing. Run `dotfiles install-ros-jazzy`.");
        Ok(())
    }
}

pub fn switch(context: &HostContext) -> Result<()> {
    require_nix(context)?;
    home_manager::apply_home_manager(context)
}

pub fn jazzy(context: &HostContext, args: &[String]) -> Result<()> {
    container::require_container(context)?;
    prepare_jazzy_devices()?;
    let launcher = context.preferred_container_launcher()?;
    let mut hook_args = vec![
        String::from("internal"),
        String::from("hook"),
        String::from("enter-login"),
    ];
    if !args.is_empty() {
        hook_args.push(String::from("--"));
        hook_args.extend(args.iter().cloned());
    }
    let mut command = context.command_as_target("distrobox");
    command.arg("enter");
    command.arg(ROS_JAZZY_CONTAINER_NAME);
    command.arg("--");
    command.arg(launcher);
    command.args(hook_args);
    system::exec_replace(command)
}

fn prepare_jazzy_devices() -> Result<()> {
    let mut changed = false;
    for prefix in ["ttyUSB", "ttyACM"] {
        for entry in std::fs::read_dir("/dev")
            .with_context(|| "failed to read /dev for device permission adjustment")?
        {
            let path = entry?.path();
            let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
                continue;
            };
            if !name.starts_with(prefix) {
                continue;
            }
            changed = true;
            system::run_sudo([
                "chmod",
                "0666",
                path.to_str().context("device path is not valid utf-8")?,
            ])
            .with_context(|| format!("failed to adjust permissions for {}", path.display()))?;
        }
    }

    if !changed {
        eprintln!("warning: no /dev/ttyUSB* or /dev/ttyACM* devices found");
    }
    Ok(())
}

pub fn cleanup(context: &HostContext) -> Result<()> {
    if let Ok(nix_bin) = home_manager::resolve_nix_bin(context) {
        context.log("Collecting old Nix store paths");
        let status = context
            .command_as_target(&nix_bin)
            .arg("store")
            .arg("gc")
            .status()
            .context("failed to run nix store gc")?;
        context.status_ok(status, "nix store gc")?;
    }

    backup::prune_backups(context)?;
    if container::container_exists(context)? {
        container::run_container_command(context, &["internal", "container", "cleanup"])?;
    }

    if system::command_exists("podman") {
        context.log("Pruning unused Podman images");
        let status = context
            .command_as_target("podman")
            .arg("image")
            .arg("prune")
            .arg("-f")
            .status()
            .context("failed to prune podman images")?;
        context.status_ok(status, "podman image prune")?;
    }

    Ok(())
}

pub fn install_ros_jazzy(context: &HostContext) -> Result<()> {
    container::ensure_ros_jazzy_box(context)?;
    container::run_container_command(context, &["internal", "container", "install"])
}

pub fn update_ros_jazzy(context: &HostContext) -> Result<()> {
    container::require_container(context)?;
    container::run_container_command(context, &["internal", "container", "update"])
}

pub fn update_flake_inputs(context: &HostContext) -> Result<()> {
    let nix_bin = require_nix(context)?;
    let status = context
        .command_as_target(&nix_bin)
        .env("NIX_CONFIG", home_manager::nix_config_from_environment())
        .arg("flake")
        .arg("update")
        .arg("--flake")
        .arg(home_manager::flake_ref(context))
        .status()
        .context("failed to update flake inputs")?;
    context.status_ok(status, "nix flake update")
}

fn require_nix(context: &HostContext) -> Result<std::path::PathBuf> {
    home_manager::resolve_nix_bin(context).context(
        "nix binary not found. Install multi-user Nix first, then run `nix run path:.#dotfiles -- install` from the repository root.",
    )
}
