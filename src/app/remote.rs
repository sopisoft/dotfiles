use super::system;
use crate::context::HostContext;
use anyhow::{Context, Result, bail};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const XRDP_CONFIG_DIR: &str = "config/xrdp";
const SYSTEM_XRDP_DIR: &str = "/etc/xrdp";
const SYSTEM_PAM_PATH: &str = "/etc/pam.d/xrdp-sesman";
const SYSTEM_XORG_XRDP_CONFIG: &str = "/etc/X11/xrdp/xorg.conf";
const SYSTEM_XRDP_SERVICE: &str = "/etc/systemd/system/xrdp.service";
const SYSTEM_SESMAN_SERVICE: &str = "/etc/systemd/system/xrdp-sesman.service";

pub fn apply(context: &HostContext) -> Result<()> {
    if !system::can_run_privileged_command() {
        eprintln!("warning: skipping xrdp system integration; sudo is unavailable");
        return Ok(());
    }

    cleanup_nix_xrdp_shims()?;
    ensure_host_xrdp_packages()?;
    ensure_host_xfce_packages()?;

    require_host_entry(Path::new("/usr/bin/xrdp"), "xrdp")?;
    require_host_entry(Path::new("/usr/bin/xrdp-sesman"), "xrdp-sesman")?;
    require_host_entry(&xorg_server_path()?, "Xorg")?;
    require_host_entry(Path::new("/usr/bin/xfce4-session"), "xfce4-session")?;
    require_host_entry(Path::new("/usr/bin/xfce4-panel"), "xfce4-panel")?;
    require_host_entry(Path::new("/usr/bin/exo-open"), "exo-open")?;

    ensure_dir(Path::new(SYSTEM_XRDP_DIR))?;

    install_repo_file(
        context,
        "session-env.sh",
        &Path::new(SYSTEM_XRDP_DIR).join("session-env.sh"),
        "0755",
    )?;
    install_repo_file(
        context,
        "startwm.sh",
        &Path::new(SYSTEM_XRDP_DIR).join("startwm.sh"),
        "0755",
    )?;
    install_repo_file(
        context,
        "reconnectwm.sh",
        &Path::new(SYSTEM_XRDP_DIR).join("reconnectwm.sh"),
        "0755",
    )?;
    install_repo_file(
        context,
        "xfce4-panel.wsl.xml",
        &Path::new(SYSTEM_XRDP_DIR).join("xfce4-panel.wsl.xml"),
        "0644",
    )?;
    install_repo_file(
        context,
        sesman_source_file()?.as_str(),
        &Path::new(SYSTEM_XRDP_DIR).join("sesman.ini"),
        "0644",
    )?;
    install_repo_file(context, "xrdp.ini", Path::new("/etc/xrdp/xrdp.ini"), "0644")?;
    install_repo_file(
        context,
        "xorg.conf",
        Path::new(SYSTEM_XORG_XRDP_CONFIG),
        "0644",
    )?;
    install_repo_file(
        context,
        pam_source_file()?.as_str(),
        Path::new(SYSTEM_PAM_PATH),
        "0644",
    )?;
    install_repo_file(
        context,
        "xrdp.service",
        Path::new(SYSTEM_XRDP_SERVICE),
        "0644",
    )?;
    install_repo_file(
        context,
        "xrdp-sesman.service",
        Path::new(SYSTEM_SESMAN_SERVICE),
        "0644",
    )?;

    stop_existing_xrdp()?;
    system::run_sudo(["systemctl", "daemon-reload"])?;
    open_firewall_port()?;
    system::run_sudo(["systemctl", "enable", "xrdp-sesman"])?;
    system::run_sudo(["systemctl", "enable", "xrdp"])?;
    system::run_sudo(["systemctl", "restart", "xrdp-sesman"])?;
    system::run_sudo(["systemctl", "restart", "xrdp"])?;

    Ok(())
}

pub fn status(context: &HostContext) -> Result<()> {
    let expected_files = BTreeMap::from([
        (
            PathBuf::from(format!("{SYSTEM_XRDP_DIR}/session-env.sh")),
            repo_file(context, "session-env.sh"),
        ),
        (
            PathBuf::from(format!("{SYSTEM_XRDP_DIR}/startwm.sh")),
            repo_file(context, "startwm.sh"),
        ),
        (
            PathBuf::from(format!("{SYSTEM_XRDP_DIR}/reconnectwm.sh")),
            repo_file(context, "reconnectwm.sh"),
        ),
        (
            PathBuf::from(format!("{SYSTEM_XRDP_DIR}/xfce4-panel.wsl.xml")),
            repo_file(context, "xfce4-panel.wsl.xml"),
        ),
        (
            PathBuf::from(format!("{SYSTEM_XRDP_DIR}/sesman.ini")),
            repo_file(context, sesman_source_file()?.as_str()),
        ),
        (
            PathBuf::from(SYSTEM_PAM_PATH),
            repo_file(context, pam_source_file()?.as_str()),
        ),
        (
            PathBuf::from(SYSTEM_XORG_XRDP_CONFIG),
            repo_file(context, "xorg.conf"),
        ),
        (
            PathBuf::from(SYSTEM_XRDP_SERVICE),
            repo_file(context, "xrdp.service"),
        ),
        (
            PathBuf::from(SYSTEM_SESMAN_SERVICE),
            repo_file(context, "xrdp-sesman.service"),
        ),
    ]);

    for (installed, source) in expected_files {
        assert_file_matches(&installed, &source)?;
    }

    let xrdp_ini = fs::read_to_string(format!("{SYSTEM_XRDP_DIR}/xrdp.ini"))
        .context("failed to read installed xrdp.ini")?;
    for expected in ["autorun=Xorg", "username=ask", "password=ask"] {
        if !xrdp_ini.lines().any(|line| line == expected) {
            bail!("installed xrdp.ini is missing `{expected}`");
        }
    }

    require_host_entry(Path::new("/usr/bin/xrdp"), "xrdp")?;
    require_host_entry(Path::new("/usr/bin/xrdp-sesman"), "xrdp-sesman")?;
    require_host_entry(&xorg_server_path()?, "Xorg")?;

    for service in ["xrdp-sesman", "xrdp"] {
        let enabled = system::read_command_stdout(["systemctl", "is-enabled", service])
            .with_context(|| format!("failed to query {service} enablement"))?;
        if enabled.trim() != "enabled" {
            bail!("{service} service is not enabled");
        }

        let active = system::read_command_stdout(["systemctl", "is-active", service])
            .with_context(|| format!("failed to query {service} state"))?;
        if active.trim() != "active" {
            bail!("{service} service is not active");
        }
    }

    println!("[ok] xrdp integration installed");
    Ok(())
}

fn cleanup_nix_xrdp_shims() -> Result<()> {
    for command in [
        "rm -f /usr/local/bin/xrdp /usr/local/bin/xrdp-sesman /usr/local/bin/Xorg",
        "rm -f /usr/local/sbin/xrdp /usr/local/sbin/xrdp-sesman /usr/local/sbin/Xorg",
    ] {
        system::run_sudo(["sh", "-c", command])?;
    }
    Ok(())
}

fn ensure_host_xrdp_packages() -> Result<()> {
    if Path::new("/usr/bin/xrdp").exists()
        && Path::new("/usr/bin/xrdp-sesman").exists()
        && xorg_server_path().is_ok_and(|path| path.exists())
    {
        return Ok(());
    }

    if system::command_exists("dnf") {
        system::run_sudo(["dnf", "install", "-y", "xrdp", "xorgxrdp"])?;
        return Ok(());
    }

    if system::command_exists("apt-get") {
        system::run_sudo(["apt-get", "update"])?;
        system::run_sudo([
            "apt-get",
            "install",
            "-y",
            "xrdp",
            "xorgxrdp",
            "xserver-xorg",
        ])?;
        return Ok(());
    }

    bail!("xrdp is missing and no supported host package manager was found")
}

fn ensure_host_xfce_packages() -> Result<()> {
    if Path::new("/usr/bin/xfce4-session").exists()
        && Path::new("/usr/bin/xfce4-panel").exists()
        && Path::new("/usr/bin/exo-open").exists()
    {
        return Ok(());
    }

    if system::command_exists("dnf") {
        system::run_sudo([
            "dnf",
            "install",
            "-y",
            "exo",
            "xfce4-panel",
            "xfce4-session",
            "xfce4-whiskermenu-plugin",
        ])?;
        return Ok(());
    }

    if system::command_exists("apt-get") {
        system::run_sudo(["apt-get", "update"])?;
        system::run_sudo([
            "apt-get",
            "install",
            "-y",
            "exo",
            "xfce4-panel",
            "xfce4-session",
            "xfce4-whiskermenu-plugin",
        ])?;
        return Ok(());
    }

    bail!("xfce desktop components are missing and no supported host package manager was found")
}

fn stop_existing_xrdp() -> Result<()> {
    for command in [
        "systemctl stop xrdp >/dev/null 2>&1 || true",
        "systemctl stop xrdp-sesman >/dev/null 2>&1 || true",
        "pkill -x xrdp >/dev/null 2>&1 || true",
        "pkill -x xrdp-sesman >/dev/null 2>&1 || true",
        "pkill -x xrdp-sesrun >/dev/null 2>&1 || true",
        "pkill -x xrdp-sesexec >/dev/null 2>&1 || true",
        "systemctl reset-failed xrdp xrdp-sesman >/dev/null 2>&1 || true",
    ] {
        system::run_sudo(["sh", "-c", command])?;
    }
    Ok(())
}

fn open_firewall_port() -> Result<()> {
    if !system::command_exists("firewall-cmd") {
        return Ok(());
    }

    let state = system::read_command_stdout(["firewall-cmd", "--state"]).unwrap_or_default();
    if state.trim() != "running" {
        return Ok(());
    }

    system::run_sudo(["firewall-cmd", "--permanent", "--add-port=3390/tcp"])?;
    system::run_sudo(["firewall-cmd", "--reload"])?;
    Ok(())
}

fn require_host_entry(path: &Path, label: &str) -> Result<()> {
    if path.exists() {
        Ok(())
    } else {
        bail!(
            "{label} is missing from the host system: {}",
            path.display()
        )
    }
}

fn repo_file(context: &HostContext, name: &str) -> PathBuf {
    context.repo_root.join(XRDP_CONFIG_DIR).join(name)
}

fn pam_source_file() -> Result<String> {
    Ok(match distro_family()?.as_str() {
        "debian" => String::from("pam.debian"),
        "fedora" => String::from("pam.fedora"),
        family => bail!("unsupported PAM layout for xrdp integration: {family}"),
    })
}

fn sesman_source_file() -> Result<String> {
    Ok(match distro_family()?.as_str() {
        "debian" => String::from("sesman.debian.ini"),
        "fedora" => String::from("sesman.fedora.ini"),
        family => bail!("unsupported sesman layout for xrdp integration: {family}"),
    })
}

fn xorg_server_path() -> Result<PathBuf> {
    Ok(match distro_family()?.as_str() {
        "debian" => PathBuf::from("/usr/lib/xorg/Xorg"),
        "fedora" => PathBuf::from("/usr/libexec/Xorg"),
        family => bail!("unsupported Xorg layout for xrdp integration: {family}"),
    })
}

fn distro_family() -> Result<String> {
    let os_release =
        fs::read_to_string("/etc/os-release").context("failed to read /etc/os-release")?;
    let fields = os_release
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| (key.to_string(), value.trim_matches('"').to_string()))
        .collect::<BTreeMap<_, _>>();

    let id = fields
        .get("ID")
        .map(|value| value.as_str())
        .unwrap_or_default();
    let id_like = fields
        .get("ID_LIKE")
        .map(|value| value.as_str())
        .unwrap_or_default();

    let family = format!("{id} {id_like}");
    if family.contains("debian") || family.contains("ubuntu") {
        return Ok(String::from("debian"));
    }
    if family.contains("fedora") || family.contains("rhel") || family.contains("centos") {
        return Ok(String::from("fedora"));
    }

    bail!("unsupported distribution for xrdp integration: {family}")
}

fn assert_file_matches(installed: &Path, source: &Path) -> Result<()> {
    let source_contents =
        fs::read(source).with_context(|| format!("failed to read {}", source.display()))?;
    let installed_contents = fs::read(installed)
        .with_context(|| format!("missing installed file {}", installed.display()))?;
    if source_contents != installed_contents {
        bail!(
            "installed xrdp file differs from repo copy: {}",
            installed.display()
        );
    }
    Ok(())
}

fn install_repo_file(
    context: &HostContext,
    source_name: &str,
    target: &Path,
    mode: &str,
) -> Result<()> {
    let source = repo_file(context, source_name);
    install_staged_file(&source, target, mode)
}

fn install_staged_file(source: &Path, target: &Path, mode: &str) -> Result<()> {
    ensure_dir(target.parent().context("missing target parent directory")?)?;
    system::run_sudo([
        "install",
        "-D",
        "-m",
        mode,
        source.to_str().context("source path is not valid utf-8")?,
        target.to_str().context("target path is not valid utf-8")?,
    ])
}

fn ensure_dir(path: &Path) -> Result<()> {
    system::run_sudo([
        "install",
        "-d",
        "-m",
        "0755",
        path.to_str().context("directory path is not valid utf-8")?,
    ])
}
