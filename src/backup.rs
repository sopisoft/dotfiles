mod fs;
mod generation;

use self::fs::{copy_path, remove_path};
use self::generation::BackupGeneration;
use crate::context::HostContext;
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs as std_fs;
use std::path::PathBuf;
use time::{format_description::FormatItem, macros::format_description};

pub(crate) use self::fs::symlink_exists;

pub(super) const TIMESTAMP_FORMAT: &[FormatItem<'static>] =
    format_description!("[year][month][day]T[hour][minute][second]Z");

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

pub fn prepare_for_deployment(context: &HostContext, targets: &[PathBuf]) -> Result<()> {
    std_fs::create_dir_all(generations_dir(context))
        .with_context(|| format!("failed to create {}", generations_dir(context).display()))?;
    backup_collisions(context, targets)?;
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

fn backup_collisions(context: &HostContext, targets: &[PathBuf]) -> Result<()> {
    let mut collisions = BTreeSet::new();
    for path in targets {
        if !path.exists() && !symlink_exists(&path) {
            continue;
        }
        if symlink_exists(path) {
            continue;
        }
        collisions.insert(path.clone());
    }

    if collisions.is_empty() {
        return Ok(());
    }

    let mut generation = BackupGeneration::create(context, "pre-deploy")?;
    for path in collisions {
        generation.backup_and_remove(&path)?;
    }
    generation.finish()?;
    prune_backups(context)
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
