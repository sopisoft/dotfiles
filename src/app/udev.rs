use super::system;
use crate::context::HostContext;
use anyhow::{Context, Result, bail};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

const REPO_RULES_DIR: &str = "udev/rules.d";
const SYSTEM_RULES_DIR: &str = "/etc/udev/rules.d";

pub fn apply(context: &HostContext) -> Result<()> {
    let repo_rules = repo_rules(context)?;
    let repo_names = repo_rules
        .iter()
        .map(|path| file_name(path))
        .collect::<BTreeSet<_>>();

    for path in &repo_rules {
        let temp_dir = tempdir().context("failed to create temporary directory")?;
        let temp_file = temp_dir.path().join(file_name(path));
        fs::copy(path, &temp_file)
            .with_context(|| format!("failed to stage {}", path.display()))?;
        system::run_sudo([
            "install",
            "-D",
            "-m",
            "0644",
            temp_file
                .to_str()
                .context("temporary udev rule path is not valid utf-8")?,
            system_rule_path(path)?
                .to_str()
                .context("system udev rule path is not valid utf-8")?,
        ])?;
    }

    for stale in installed_managed_rules()? {
        if !repo_names.contains(&file_name(&stale)) {
            system::run_sudo([
                "rm",
                "-f",
                stale
                    .to_str()
                    .context("stale udev rule path is not valid utf-8")?,
            ])?;
        }
    }

    system::run_sudo(["udevadm", "control", "--reload-rules"])?;
    system::run_sudo(["udevadm", "trigger", "--subsystem-match=tty"])
}

pub fn status(context: &HostContext) -> Result<()> {
    let repo_rules = repo_rules(context)?;
    if repo_rules.is_empty() {
        bail!(
            "no managed udev rules found in {}",
            context.repo_root.join(REPO_RULES_DIR).display()
        );
    }

    for rule in repo_rules {
        let target = system_rule_path(&rule)?;
        let source_contents =
            fs::read(&rule).with_context(|| format!("failed to read {}", rule.display()))?;
        let target_contents = fs::read(&target)
            .with_context(|| format!("missing installed udev rule {}", target.display()))?;
        if source_contents != target_contents {
            bail!(
                "installed udev rule differs from repo copy: {}",
                target.display()
            );
        }
    }

    println!("[ok] managed udev rules installed");
    Ok(())
}

fn repo_rules(context: &HostContext) -> Result<Vec<PathBuf>> {
    let rules_dir = context.repo_root.join(REPO_RULES_DIR);
    let mut rules = fs::read_dir(&rules_dir)
        .with_context(|| format!("failed to read {}", rules_dir.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("rules"))
        .collect::<Vec<_>>();
    rules.sort();

    if rules.iter().any(|path| !is_managed_rule_name(path)) {
        bail!("all managed udev rule files must include `dotfiles-` in the name");
    }

    Ok(rules)
}

fn installed_managed_rules() -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(SYSTEM_RULES_DIR)
        .with_context(|| format!("failed to read {SYSTEM_RULES_DIR}"))?
    {
        let path = entry?.path();
        if path.is_file() && is_managed_rule_name(&path) {
            paths.push(path);
        }
    }
    Ok(paths)
}

fn system_rule_path(source: &Path) -> Result<PathBuf> {
    Ok(Path::new(SYSTEM_RULES_DIR).join(file_name(source)))
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_string()
}

fn is_managed_rule_name(path: &Path) -> bool {
    file_name(path).contains("dotfiles-")
}
