use super::system;
use crate::backup;
use crate::context::HostContext;
use anyhow::{Context, Result, bail};
use std::fs;
use std::process::Command as ProcessCommand;
use tempfile::tempdir;

pub fn install(context: &HostContext) -> Result<()> {
    let release_api = std::env::var("HAZKEY_RELEASE_API").unwrap_or_else(|_| {
        String::from("https://api.github.com/repos/7ka-Hiira/hazkey/releases/latest")
    });
    let architecture = system::read_command_stdout(["dpkg", "--print-architecture"])?;
    let release_json = system::read_command_stdout(["curl", "-fsSL", &release_api])?;
    let download_url = find_download_url(&release_json, architecture.trim())
        .with_context(|| format!("hazkey release asset not found for {}", architecture.trim()))?;

    let temp_dir = tempdir().context("failed to create temporary directory")?;
    let deb_path = temp_dir.path().join(
        download_url
            .rsplit('/')
            .next()
            .context("failed to determine hazkey asset name")?,
    );

    context.log(format!("Downloading {download_url}"));
    let status = ProcessCommand::new("curl")
        .arg("-fL")
        .arg(&download_url)
        .arg("-o")
        .arg(&deb_path)
        .status()
        .context("failed to download hazkey package")?;
    if !status.success() {
        bail!("failed to download hazkey package");
    }

    system::run_sudo([
        "apt-get",
        "install",
        "-y",
        deb_path
            .to_str()
            .context("hazkey package path is not valid utf-8")?,
    ])?;

    let profile_path = context.target_config_home.join("fcitx5/profile");
    if profile_path.exists() {
        backup::backup_path(context, &profile_path, "hazkey-profile")?;
    }
    let profile = match fs::read_to_string(&profile_path) {
        Ok(profile) if profile.contains("[Groups/0]") => ensure_profile(profile),
        Ok(_) | Err(_) => default_profile(),
    };

    if let Some(parent) = profile_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(&profile_path, profile)
        .with_context(|| format!("failed to write {}", profile_path.display()))?;

    let _ = ProcessCommand::new("pkill")
        .arg("-x")
        .arg("hazkey-server")
        .status();
    let _ = ProcessCommand::new("fcitx5-remote").arg("-r").status();
    Ok(())
}

fn ensure_profile(profile: String) -> String {
    let mut lines = profile.lines().map(String::from).collect::<Vec<_>>();
    if let Some(group_index) = lines.iter().position(|line| line == "[Groups/0]") {
        set_default_im(&mut lines, group_index);
    }

    if !lines.iter().any(|line| line == "Name=hazkey") {
        lines.push(String::new());
        lines.push(String::from("[Groups/0/Items/1]"));
        lines.push(String::from("Name=hazkey"));
        lines.push(String::from("Layout="));
    }

    if !lines.iter().any(|line| line == "[GroupOrder]") {
        lines.push(String::new());
        lines.push(String::from("[GroupOrder]"));
        lines.push(String::from("0=Default"));
    }

    format!("{}\n", lines.join("\n"))
}

fn set_default_im(lines: &mut Vec<String>, group_index: usize) {
    for line in lines
        .iter_mut()
        .skip(group_index + 1)
        .take_while(|line| !line.starts_with('['))
    {
        if line.starts_with("DefaultIM=") {
            *line = String::from("DefaultIM=hazkey");
            return;
        }
    }
    lines.insert(group_index + 1, String::from("DefaultIM=hazkey"));
}

fn default_profile() -> String {
    String::from(
        "[Groups/0]\nName=Default\nDefault Layout=jp\nDefaultIM=hazkey\n\n[Groups/0/Items/0]\nName=keyboard-jp\nLayout=\n\n[Groups/0/Items/1]\nName=hazkey\nLayout=\n\n[GroupOrder]\n0=Default\n",
    )
}

fn find_download_url(release_json: &str, architecture: &str) -> Option<String> {
    let value = serde_json::from_str::<serde_json::Value>(release_json).ok()?;
    value
        .get("assets")?
        .as_array()?
        .iter()
        .filter_map(|asset| {
            asset.get("name").and_then(|value| value.as_str()).zip(
                asset
                    .get("browser_download_url")
                    .and_then(|value| value.as_str()),
            )
        })
        .find_map(|(name, url)| {
            name.ends_with(&format!("_{architecture}.deb"))
                .then(|| url.to_string())
        })
}
