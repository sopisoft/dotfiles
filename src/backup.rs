mod fs;
mod generation;

use self::fs::{copy_path, is_nix_store_symlink, remove_path, symlink_exists};
use self::generation::BackupGeneration;
use crate::context::HostContext;
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs as std_fs;
use std::path::{Path, PathBuf};
use time::{format_description::FormatItem, macros::format_description};
use walkdir::WalkDir;

pub(super) const TIMESTAMP_FORMAT: &[FormatItem<'static>] =
    format_description!("[year][month][day]T[hour][minute][second]Z");

const MANAGED_TARGETS: &[&str] = &[
    ".zshenv",
    ".zprofile",
    ".zshrc",
    ".local/bin/dotfiles",
    ".local/bin/vim",
    ".local/bin/x-terminal-emulator",
    ".config/alacritty",
    ".config/direnv",
    ".config/environment.d",
    ".config/fontconfig/conf.d",
    ".config/gnome-xdg-terminals.list",
    ".config/mozilla/firefox",
    ".config/nix",
    ".config/nvim",
    ".config/starship.toml",
    ".config/user-dirs.conf",
    ".config/user-dirs.dirs",
    ".config/user-dirs.locale",
    ".config/xdg-terminals.list",
    ".config/zellij",
    ".config/zsh",
];

const LEGACY_TARGETS: &[&str] = &[
    ".config/zsh/35-nvm.zsh",
    ".config/zsh/40-node-nvm.zsh",
    ".config/zsh/40-distrobox.zsh",
    ".config/zsh/50-ros.zsh",
    ".config/zsh/60-prompt.zsh",
    ".config/zsh/70-zellij.zsh",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    pub version: u32,
    pub generation_id: String,
    pub created_at: String,
    pub reason: String,
    pub entries: Vec<BackupEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupEntry {
    pub original_path: String,
    pub stored_path: String,
}

pub fn prepare_for_activation(context: &HostContext) -> Result<()> {
    std_fs::create_dir_all(generations_dir(context))
        .with_context(|| format!("failed to create {}", generations_dir(context).display()))?;
    import_legacy_backups(context)?;
    cleanup_legacy_managed_symlinks(context)?;
    backup_home_manager_collisions(context)?;
    prune_backups(context)
}

pub fn list_backups(context: &HostContext) -> Result<Vec<BackupManifest>> {
    let mut manifests = Vec::new();
    let generations_dir = generations_dir(context);
    if !generations_dir.exists() {
        return Ok(manifests);
    }

    let mut generation_paths = std_fs::read_dir(&generations_dir)
        .with_context(|| format!("failed to read {}", generations_dir.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    generation_paths.sort();
    generation_paths.reverse();

    for generation in generation_paths {
        let manifest_path = generation.join("manifest.json");
        let manifest_bytes = std_fs::read(&manifest_path)
            .with_context(|| format!("failed to read {}", manifest_path.display()))?;
        manifests.push(
            serde_json::from_slice::<BackupManifest>(&manifest_bytes)
                .with_context(|| format!("failed to parse {}", manifest_path.display()))?,
        );
    }
    Ok(manifests)
}

pub fn print_backup_list(context: &HostContext) -> Result<()> {
    for manifest in list_backups(context)? {
        println!(
            "{}  {}  {} entries",
            manifest.generation_id,
            manifest.reason,
            manifest.entries.len()
        );
    }
    Ok(())
}

pub fn rollback(context: &HostContext, generation: Option<&str>) -> Result<()> {
    let manifest = select_manifest(context, generation)?;
    for entry in &manifest.entries {
        let original_path = PathBuf::from(&entry.original_path);
        let stored_path = PathBuf::from(&entry.stored_path);
        if original_path.exists() || symlink_exists(&original_path) {
            remove_path(&original_path)?;
        }
        copy_path(&stored_path, &original_path)?;
    }
    Ok(())
}

pub fn backup_path(context: &HostContext, path: &Path, reason: &str) -> Result<()> {
    if !path.exists() && !symlink_exists(path) {
        return Ok(());
    }

    let mut generation = BackupGeneration::create(context, reason)?;
    generation.backup_copy(path)?;
    generation.finish()?;
    prune_backups(context)
}

pub fn import_legacy_backups(context: &HostContext) -> Result<()> {
    let mut legacy_entries = Vec::new();
    for entry in WalkDir::new(&context.target_home)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.starts_with(&context.backup_root) {
            continue;
        }
        let Some(file_name) = path.file_name().and_then(OsStr::to_str) else {
            continue;
        };
        if let Some(original_name) = file_name.split_once(".hm-backup-").map(|value| value.0) {
            legacy_entries.push((path.to_path_buf(), path.with_file_name(original_name)));
            continue;
        }
        if let Some(original_name) = strip_bak_suffix(file_name) {
            legacy_entries.push((path.to_path_buf(), path.with_file_name(original_name)));
        }
    }

    if legacy_entries.is_empty() {
        return Ok(());
    }

    let mut generation = BackupGeneration::create(context, "legacy-import")?;
    for (legacy_path, original_path) in legacy_entries {
        generation.ingest_existing_backup(&legacy_path, &original_path)?;
    }
    generation.finish()?;
    prune_backups(context)
}

pub fn prune_backups(context: &HostContext) -> Result<()> {
    let generations_dir = generations_dir(context);
    if !generations_dir.exists() {
        return Ok(());
    }

    let mut generations = std_fs::read_dir(&generations_dir)
        .with_context(|| format!("failed to read {}", generations_dir.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    generations.sort();
    if generations.len() <= context.backup_limit {
        return Ok(());
    }

    let to_delete = generations.len() - context.backup_limit;
    for generation in generations.into_iter().take(to_delete) {
        std_fs::remove_dir_all(&generation)
            .with_context(|| format!("failed to remove old backup {}", generation.display()))?;
    }
    Ok(())
}

pub(super) fn generations_dir(context: &HostContext) -> PathBuf {
    context.backup_root.join("generations")
}

fn backup_home_manager_collisions(context: &HostContext) -> Result<()> {
    let candidates = MANAGED_TARGETS
        .iter()
        .map(|relative| context.target_home.join(relative))
        .chain(
            LEGACY_TARGETS
                .iter()
                .map(|relative| context.target_home.join(relative)),
        );

    let mut collisions = BTreeSet::new();
    for path in candidates {
        if !path.exists() && !symlink_exists(&path) {
            continue;
        }
        if is_nix_store_symlink(&path)? {
            continue;
        }
        collisions.insert(path);
    }

    if collisions.is_empty() {
        return Ok(());
    }

    let mut generation = BackupGeneration::create(context, "pre-home-manager")?;
    for path in collisions {
        generation.backup_and_remove(&path)?;
    }
    generation.finish()?;
    prune_backups(context)
}

fn cleanup_legacy_managed_symlinks(context: &HostContext) -> Result<()> {
    for relative in LEGACY_TARGETS {
        let path = context.target_home.join(relative);
        if is_nix_store_symlink(&path)? {
            remove_path(&path)?;
        }
    }
    Ok(())
}

fn select_manifest(context: &HostContext, generation: Option<&str>) -> Result<BackupManifest> {
    let manifests = list_backups(context)?;
    if manifests.is_empty() {
        bail!("no backups available");
    }

    if let Some(generation_id) = generation {
        manifests
            .into_iter()
            .find(|manifest| manifest.generation_id == generation_id)
            .with_context(|| format!("backup generation not found: {generation_id}"))
    } else {
        manifests
            .into_iter()
            .next()
            .context("no latest backup available")
    }
}

fn strip_bak_suffix(file_name: &str) -> Option<&str> {
    let (base, suffix) = file_name.rsplit_once(".bak.")?;
    suffix.chars().all(|ch| ch.is_ascii_digit()).then_some(base)
}
