use super::{BackupEntry, BackupManifest, TIMESTAMP_FORMAT, fs as backup_fs, generations_dir};
use crate::context::HostContext;
use anyhow::{Context, Result};
use std::fs as std_fs;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;

pub(super) struct BackupGeneration {
    generation_dir: PathBuf,
    payload_dir: PathBuf,
    manifest: BackupManifest,
}

impl BackupGeneration {
    pub(super) fn create(context: &HostContext, reason: &str) -> Result<Self> {
        let timestamp = OffsetDateTime::now_utc()
            .format(TIMESTAMP_FORMAT)
            .context("failed to format backup timestamp")?;
        let generation_id = format!("{timestamp}-{reason}");
        let generation_dir = generations_dir(context).join(&generation_id);
        let payload_dir = generation_dir.join("payload");
        std_fs::create_dir_all(&payload_dir)
            .with_context(|| format!("failed to create {}", payload_dir.display()))?;

        Ok(Self {
            generation_dir,
            payload_dir,
            manifest: BackupManifest {
                version: 1,
                generation_id,
                created_at: timestamp,
                reason: reason.to_string(),
                entries: Vec::new(),
            },
        })
    }

    pub(super) fn backup_and_remove(&mut self, source: &Path) -> Result<()> {
        let stored_path = self.stored_path_for(source)?;
        backup_fs::copy_path(source, &stored_path)?;
        backup_fs::remove_path(source)?;
        self.record_entry(source, stored_path);
        Ok(())
    }

    pub(super) fn ingest_existing_backup(
        &mut self,
        source: &Path,
        original_path: &Path,
    ) -> Result<()> {
        let stored_path = self.stored_path_for(source)?;
        backup_fs::move_path(source, &stored_path)?;
        self.record_entry(original_path, stored_path);
        Ok(())
    }

    pub(super) fn finish(self) -> Result<()> {
        let manifest_path = self.generation_dir.join("manifest.json");
        let bytes =
            serde_json::to_vec_pretty(&self.manifest).context("failed to serialize backup")?;
        std_fs::write(&manifest_path, bytes)
            .with_context(|| format!("failed to write {}", manifest_path.display()))
    }

    fn stored_path_for(&self, source: &Path) -> Result<PathBuf> {
        Ok(self.payload_dir.join(
            source
                .strip_prefix(Path::new("/"))
                .context("failed to relativize backup source")?,
        ))
    }

    fn record_entry(&mut self, original_path: &Path, stored_path: PathBuf) {
        self.manifest.entries.push(BackupEntry {
            original_path: original_path.display().to_string(),
            stored_path: stored_path.display().to_string(),
        });
    }
}
