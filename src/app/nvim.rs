use super::system;
use crate::context::HostContext;
use anyhow::{Context, Result, bail};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use tempfile::tempdir;

pub fn update_neovim_plugins() -> Result<()> {
    let nvim_bin = env::var("NVIM_BIN").unwrap_or_else(|_| String::from("nvim"));
    let status = ProcessCommand::new(&nvim_bin)
        .env("XDG_CONFIG_HOME", xdg_config_home())
        .env("XDG_DATA_HOME", xdg_data_home())
        .arg("--headless")
        .arg("+lua vim.pack.update(nil, { force = true })")
        .arg("+qa")
        .status()
        .with_context(|| format!("failed to run {nvim_bin}"))?;
    if status.success() {
        Ok(())
    } else {
        bail!("failed to update Neovim plugins with status {status}")
    }
}

pub fn update_neovim_plugins_as_target(context: &HostContext) -> Result<()> {
    let exe = context.host_current_exe()?;
    let status = context
        .command_as_target(&exe)
        .arg("update-neovim-plugins")
        .status()
        .context("failed to update Neovim plugins as target user")?;
    context.status_ok(status, "update-neovim-plugins")
}

pub fn sync_nvim_pack() -> Result<()> {
    let nvim_bin = env::var("NVIM_BIN").unwrap_or_else(|_| String::from("nvim"));
    let xdg_config_home = xdg_config_home();
    let xdg_data_home = xdg_data_home();
    let legacy_config_pack_dir = xdg_config_home.join("nvim/pack");
    let legacy_pack_dir = xdg_data_home.join("nvim/site/pack/dotfiles");
    let staging_root = tempdir().context("failed to create temporary directory")?;
    let staging_config_home = staging_root.path().join("config");

    fs::create_dir_all(staging_config_home.join("nvim"))
        .context("failed to create staging nvim directory")?;
    system::copy_dir_contents(
        &xdg_config_home.join("nvim"),
        &staging_config_home.join("nvim"),
    )?;
    system::remove_path_if_exists(&legacy_pack_dir)?;
    system::remove_path_if_exists(&legacy_config_pack_dir)?;

    let status = ProcessCommand::new(&nvim_bin)
        .env("XDG_CONFIG_HOME", &staging_config_home)
        .env("XDG_DATA_HOME", &xdg_data_home)
        .arg("--headless")
        .arg("+qa")
        .status()
        .with_context(|| format!("failed to run {nvim_bin}"))?;
    if status.success() {
        Ok(())
    } else {
        bail!("failed to sync nvim pack with status {status}")
    }
}

fn xdg_config_home() -> PathBuf {
    env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| system::home_dir().join(".config"))
}

fn xdg_data_home() -> PathBuf {
    env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| system::home_dir().join(".local/share"))
}
