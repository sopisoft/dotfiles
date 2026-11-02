use super::{deploy, system, udev};
use crate::backup;
use crate::context::HostContext;
use anyhow::{Context, Result};

pub fn install(context: &HostContext) -> Result<()> {
    deploy::apply(context)?;
    udev::apply(context)?;
    Ok(())
}

pub fn update(context: &HostContext) -> Result<()> {
    deploy::apply(context)?;
    udev::apply(context)?;
    Ok(())
}

pub fn switch(context: &HostContext) -> Result<()> {
    deploy::apply(context)
}

pub fn cleanup(context: &HostContext) -> Result<()> {
    if system::command_exists_as_target(context, "pixi")? {
        context.log("Cleaning pixi caches");
        let status = context
            .command_as_target("pixi")
            .arg("clean")
            .arg("cache")
            .args([
                "--conda",
                "--pypi",
                "--mapping",
                "--exec",
                "--repodata",
                "--build-backends",
                "--build",
                "-y",
            ])
            .status()
            .context("failed to clean pixi caches")?;
        context.status_ok(status, "pixi cache cleanup")?;
    }

    backup::prune_backups(context)?;

    Ok(())
}
