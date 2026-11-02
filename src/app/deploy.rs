use crate::backup;
use crate::context::HostContext;
use anyhow::{Context, Result, bail};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

const LINKS: &[(&str, &str)] = &[
    ("zsh/.zshenv", ".zshenv"),
    ("zsh/.zshrc", ".zshrc"),
    ("zsh/config", ".config/zsh"),
    ("config/alacritty", ".config/alacritty"),
    ("config/direnv", ".config/direnv"),
    ("config/environment.d", ".config/environment.d"),
    (
        "config/fcitx5/org.fcitx.Fcitx5.desktop",
        ".config/autostart/org.fcitx.Fcitx5.desktop",
    ),
    ("config/fcitx5/profile", ".config/fcitx5/profile"),
    ("config/nvim", ".config/nvim"),
    ("config/starship.toml", ".config/starship.toml"),
    (
        "config/vnc/start-vnc-session.sh",
        ".local/bin/start-vnc-session",
    ),
    (
        "config/vnc/x11vnc.desktop",
        ".config/autostart/x11vnc.desktop",
    ),
    ("config/zellij", ".config/zellij"),
    (
        "config/desktop/dbus-session.conf",
        ".config/dbus-1/session.conf",
    ),
    (
        "config/desktop/user-dirs.locale",
        ".config/user-dirs.locale",
    ),
    (
        "config/desktop/xfce4-keyboard-shortcuts.xml",
        ".config/xfce4/xfconf/xfce-perchannel-xml/xfce4-keyboard-shortcuts.xml",
    ),
];

const OBSOLETE_TARGETS: &[&str] = &[".config/nix"];

pub fn apply(context: &HostContext) -> Result<()> {
    let mut links = LINKS
        .iter()
        .map(|(source, target)| {
            (
                context.repo_root.join(source),
                context.target_home.join(target),
            )
        })
        .collect::<Vec<_>>();
    links.push((
        std::env::current_exe().context("failed to locate dotfiles executable")?,
        context.target_home.join(".local/bin/dotfiles"),
    ));
    links.push((
        PathBuf::from("nvim"),
        context.target_home.join(".local/bin/vim"),
    ));

    let mut targets = links
        .iter()
        .filter(|(source, target)| source != target)
        .map(|(_, target)| target.clone())
        .collect::<Vec<_>>();
    targets.extend(
        OBSOLETE_TARGETS
            .iter()
            .map(|target| context.target_home.join(target)),
    );
    backup::prepare_for_deployment(context, &targets)?;

    for (source, target) in links {
        if source == target || link_matches(&source, &target) {
            continue;
        }
        replace_with_symlink(&source, &target)?;
    }

    for target in OBSOLETE_TARGETS {
        let target = context.target_home.join(target);
        if target.exists() || backup::symlink_exists(&target) {
            fs::remove_dir_all(&target)
                .or_else(|_| fs::remove_file(&target))
                .with_context(|| format!("failed to remove obsolete {}", target.display()))?;
        }
    }

    backup::prune_backups(context)
}

fn replace_with_symlink(source: &Path, target: &Path) -> Result<()> {
    if !source.exists() && source != Path::new("nvim") {
        bail!("deployment source does not exist: {}", source.display());
    }
    if target.exists() || backup::symlink_exists(target) {
        fs::remove_dir_all(target)
            .or_else(|_| fs::remove_file(target))
            .with_context(|| format!("failed to replace {}", target.display()))?;
    }
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    symlink(source, target).with_context(|| {
        format!(
            "failed to link {} to {}",
            source.display(),
            target.display()
        )
    })
}

fn link_matches(source: &Path, target: &Path) -> bool {
    fs::read_link(target)
        .map(|link| link == source)
        .unwrap_or(false)
}
