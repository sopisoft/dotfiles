use super::container_setup;
use crate::context::{HostContext, ROS_JAZZY_CONTAINER_NAME};
use anyhow::{Context, Result, bail};
use std::env;

pub fn ensure_ros_jazzy_box(context: &HostContext) -> Result<()> {
    let manifest_path = context.repo_root.join("distrobox/ros-jazzy.ini");
    if !manifest_path.exists() {
        bail!("missing distrobox manifest: {}", manifest_path.display());
    }

    let replace_requested = env::var("ROS_JAZZY_BOX_REPLACE").ok().as_deref() == Some("1");
    if replace_requested && container_exists(context)? {
        context.log(format!(
            "Removing existing distrobox container {ROS_JAZZY_CONTAINER_NAME}"
        ));
        let status = context
            .command_as_target("distrobox")
            .arg("rm")
            .arg("--force")
            .arg(ROS_JAZZY_CONTAINER_NAME)
            .status()
            .context("failed to remove existing distrobox container")?;
        context.status_ok(status, "distrobox rm --force")?;
    }

    context.log(format!(
        "Ensuring distrobox container {ROS_JAZZY_CONTAINER_NAME}"
    ));
    let mut command = context.command_as_target("distrobox-assemble");
    command.arg("create");
    command.arg("--file");
    command.arg(manifest_path);
    command.arg("--name");
    command.arg(ROS_JAZZY_CONTAINER_NAME);
    if replace_requested {
        command.arg("--replace");
    }

    let status = command
        .status()
        .context("failed to run distrobox-assemble create")?;
    context.status_ok(status, "distrobox-assemble create")?;
    ensure_host_nix_link(context)
}

pub fn container_exists(context: &HostContext) -> Result<bool> {
    let output = context
        .command_as_target("distrobox")
        .arg("list")
        .arg("--no-color")
        .output()
        .context("failed to run distrobox list")?;
    if !output.status.success() {
        return Ok(false);
    }

    let listing = String::from_utf8_lossy(&output.stdout);
    Ok(listing
        .lines()
        .filter_map(|line| line.split('|').nth(1))
        .map(str::trim)
        .any(|name| name == ROS_JAZZY_CONTAINER_NAME))
}

pub fn require_container(context: &HostContext) -> Result<()> {
    if container_exists(context)? {
        Ok(())
    } else {
        bail!(
            "distrobox container {ROS_JAZZY_CONTAINER_NAME} does not exist. Run `dotfiles install-ros-jazzy`."
        )
    }
}

pub fn run_container_command(context: &HostContext, args: &[&str]) -> Result<()> {
    require_container(context)?;
    ensure_host_nix_link(context)?;

    let container_exe = context.preferred_container_launcher()?;
    let mut command = context.command_as_target("distrobox");
    command.env("TERM", "xterm-256color");
    command.arg("enter");
    command.arg(ROS_JAZZY_CONTAINER_NAME);
    command.arg("--");
    command.arg(container_exe);
    command.args(args);

    let status = command
        .status()
        .context("failed to run command inside distrobox")?;
    context.status_ok(status, "distrobox enter")
}

pub fn install_ros_jazzy_internal() -> Result<()> {
    container_setup::install_ros_jazzy()
}

pub fn update_ros_jazzy_internal() -> Result<()> {
    container_setup::update_ros_jazzy()
}

pub fn cleanup_ros_jazzy_internal() -> Result<()> {
    container_setup::cleanup_ros_jazzy()
}

fn ensure_host_nix_link(context: &HostContext) -> Result<()> {
    bootstrap_container_user(context)?;
    let status = context
        .command_as_target("podman")
        .arg("exec")
        .arg("--user")
        .arg("root")
        .arg(ROS_JAZZY_CONTAINER_NAME)
        .arg("sh")
        .arg("-lc")
        .arg(
            "if [ -L /nix ]; then \
                target=$(readlink /nix); \
                [ \"$target\" = /run/host/nix ] || { echo \"/nix points to unexpected target: $target\" >&2; exit 1; }; \
            elif [ -d /nix ]; then \
                if [ -d /nix/store ]; then \
                    exit 0; \
                elif [ -z \"$(ls -A /nix 2>/dev/null)\" ]; then \
                    rmdir /nix && ln -s /run/host/nix /nix; \
                else \
                    echo \"/nix exists as a non-empty directory\" >&2; \
                    exit 1; \
                fi; \
            elif [ -e /nix ]; then \
                echo \"/nix exists and is not a symlink or directory\" >&2; \
                exit 1; \
            else \
                ln -s /run/host/nix /nix; \
            fi",
        )
        .status()
        .context("failed to ensure /nix link inside container")?;
    context.status_ok(status, "ensure /nix -> /run/host/nix")
}

fn bootstrap_container_user(context: &HostContext) -> Result<()> {
    let status = context
        .command_as_target("distrobox")
        .arg("enter")
        .arg(ROS_JAZZY_CONTAINER_NAME)
        .arg("--")
        .arg("true")
        .status()
        .context("failed to bootstrap distrobox user state")?;
    context.status_ok(status, "distrobox enter -- true")
}
