use super::{system, udev};
use crate::backup;
use crate::context::HostContext;
use crate::home_manager;
use anyhow::{Context, Result};

pub fn install(context: &HostContext, _skip_host_packages: bool) -> Result<()> {
    require_nix(context)?;
    home_manager::apply_home_manager(context)?;
    udev::apply(context)?;
    Ok(())
}

pub fn update(context: &HostContext) -> Result<()> {
    require_nix(context)?;
    update_flake_inputs(context)?;
    switch(context)?;
    udev::apply(context)?;
    Ok(())
}

pub fn switch(context: &HostContext) -> Result<()> {
    require_nix(context)?;
    home_manager::apply_home_manager(context)
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
