use anyhow::{Context, Result, bail};
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

pub const ROS_JAZZY_CONTAINER_NAME: &str = "ros-jazzy";

#[derive(Debug, Clone)]
pub struct HostContext {
    pub repo_root: PathBuf,
    pub current_user: String,
    pub target_user: String,
    pub target_home: PathBuf,
    pub target_uid: u32,
    pub target_config_home: PathBuf,
    pub backup_root: PathBuf,
    pub backup_limit: usize,
}

impl HostContext {
    pub fn detect() -> Result<Self> {
        let current_user = current_unix_user()?;
        let target_user = env::var("DOTFILES_TARGET_USER")
            .ok()
            .filter(|value| !value.is_empty())
            .or_else(|| {
                if current_user == "root" {
                    env::var("SUDO_USER").ok().filter(|value| !value.is_empty())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| current_user.clone());
        let passwd = read_passwd_entry(&target_user)?;
        let repo_root = resolve_repo_root(&passwd.home)?;
        let backup_limit = env::var("DOTFILES_BACKUP_LIMIT")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(10);
        let backup_root = resolve_backup_root(&passwd.home);

        Ok(Self {
            repo_root,
            current_user,
            target_user,
            target_home: passwd.home.clone(),
            target_uid: passwd.uid,
            target_config_home: passwd.home.join(".config"),
            backup_root,
            backup_limit,
        })
    }

    pub fn is_target_user(&self) -> bool {
        self.current_user == self.target_user
    }

    pub fn preferred_container_launcher(&self) -> Result<PathBuf> {
        let exe = host_current_exe()?;
        host_path_for_container(&exe)
    }

    pub fn host_current_exe(&self) -> Result<PathBuf> {
        host_current_exe()
    }

    pub fn target_path(&self) -> OsString {
        let mut path = OsString::new();
        path.push(self.target_home.join(".cargo/bin"));
        path.push(OsStr::new(":"));
        path.push(self.target_home.join(".local/bin"));
        path.push(OsStr::new(":"));
        path.push(self.target_home.join(".nix-profile/bin"));
        path.push(OsStr::new(":"));
        path.push("/nix/var/nix/profiles/default/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin");
        path
    }

    pub fn command_as_target<S: AsRef<OsStr>>(&self, program: S) -> Command {
        if self.is_target_user() {
            let mut command = Command::new(program);
            command.env("PATH", self.target_path());
            return command;
        }

        let mut command = if self.current_user == "root" {
            let mut command = Command::new("runuser");
            command.arg("-u");
            command.arg(&self.target_user);
            command.arg("--");
            command.arg("env");
            command
        } else {
            let mut command = Command::new("sudo");
            command.arg("-H");
            command.arg("-u");
            command.arg(&self.target_user);
            command.arg("env");
            command
        };
        command.env_remove("SUDO_USER");
        command.env_remove("SUDO_UID");
        command.env_remove("SUDO_GID");
        command.env_remove("SUDO_HOME");
        command.env_remove("SUDO_COMMAND");
        command.arg(format!("HOME={}", self.target_home.display()));
        command.arg(format!("USER={}", self.target_user));
        command.arg(format!("LOGNAME={}", self.target_user));
        command.arg(format!(
            "XDG_CONFIG_HOME={}",
            self.target_config_home.display()
        ));
        command.arg(format!("XDG_RUNTIME_DIR=/run/user/{}", self.target_uid));
        command.arg(format!("PATH={}", self.target_path().to_string_lossy()));
        command.arg(program);
        command
    }

    pub fn status_ok(&self, status: ExitStatus, action: &str) -> Result<()> {
        if status.success() {
            Ok(())
        } else {
            bail!("{action} failed with status {status}");
        }
    }

    pub fn log(&self, message: impl AsRef<str>) {
        println!("==> {}", message.as_ref());
    }
}

fn resolve_repo_root(target_home: &Path) -> Result<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(explicit) = env::var("DOTFILES_DIR") {
        candidates.push(PathBuf::from(explicit));
    }

    let current_dir = env::current_dir().context("failed to determine current directory")?;
    candidates.push(current_dir.clone());
    candidates.extend(current_dir.ancestors().skip(1).map(PathBuf::from));

    candidates.push(target_home.join("dotfiles"));

    let current_exe = host_current_exe().ok();
    if let Some(current_exe) = current_exe
        && let Some(parent) = current_exe.parent()
    {
        candidates.push(parent.to_path_buf());
        candidates.extend(parent.ancestors().skip(1).map(PathBuf::from));
    }

    for candidate in candidates {
        if is_dotfiles_root(&candidate) {
            return candidate
                .canonicalize()
                .with_context(|| format!("failed to canonicalize {}", candidate.display()));
        }
    }

    bail!("failed to locate dotfiles repository root")
}

fn is_dotfiles_root(path: &Path) -> bool {
    path.join("flake.nix").is_file()
        && path.join("home/home.nix").is_file()
        && path.join("Cargo.toml").is_file()
}

fn resolve_backup_root(target_home: &Path) -> PathBuf {
    let preferred = target_home.join(".local/state/dotfiles/backups");
    if is_writable_directory(&preferred) {
        return preferred;
    }

    target_home.join(".local/state/dotfiles-user/backups")
}

fn is_writable_directory(path: &Path) -> bool {
    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };
    if !metadata.is_dir() {
        return false;
    }

    let test_path = path.join(".dotfiles-write-test");
    match OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&test_path)
    {
        Ok(file) => {
            drop(file);
            let _ = fs::remove_file(test_path);
            true
        }
        Err(_) => false,
    }
}

pub fn host_current_exe() -> Result<PathBuf> {
    let current_exe = env::current_exe().context("failed to locate current executable")?;
    if let Ok(stripped) = current_exe.strip_prefix("/run/host") {
        return Ok(stripped.to_path_buf());
    }
    Ok(current_exe)
}

pub fn host_path_for_container(path: &Path) -> Result<PathBuf> {
    let resolved = match fs::canonicalize(path) {
        Ok(path) => path,
        Err(_) => {
            let metadata = fs::symlink_metadata(path)
                .with_context(|| format!("failed to stat {}", path.display()))?;
            if metadata.file_type().is_symlink() {
                let target = fs::read_link(path).with_context(|| {
                    format!("failed to read symlink target for {}", path.display())
                })?;
                if target.is_absolute() {
                    target
                } else {
                    path.parent()
                        .context("missing symlink parent")?
                        .join(target)
                }
            } else {
                path.to_path_buf()
            }
        }
    };

    if resolved.starts_with("/run/host") {
        return Ok(resolved);
    }

    Ok(PathBuf::from("/run/host").join(
        resolved
            .strip_prefix(Path::new("/"))
            .context("failed to relativize host path for container")?,
    ))
}

#[derive(Debug, Clone)]
struct PasswdEntry {
    uid: u32,
    home: PathBuf,
}

fn read_passwd_entry(user: &str) -> Result<PasswdEntry> {
    let line = read_passwd_line(user)?;
    let fields: Vec<_> = line.split(':').collect();
    if fields.len() < 7 {
        bail!("unexpected passwd entry for {user}: {line}");
    }

    let uid = fields[2]
        .parse::<u32>()
        .with_context(|| format!("failed to parse uid for {user}"))?;

    Ok(PasswdEntry {
        uid,
        home: PathBuf::from(fields[5]),
    })
}

fn read_passwd_line(user: &str) -> Result<String> {
    let getent_program = if Path::new("/usr/bin/getent").is_file() {
        PathBuf::from("/usr/bin/getent")
    } else {
        PathBuf::from("getent")
    };

    match Command::new(&getent_program)
        .arg("passwd")
        .arg(user)
        .output()
    {
        Ok(output) if output.status.success() => {
            String::from_utf8(output.stdout).context("passwd entry is not valid utf-8")
        }
        Ok(_) | Err(_) => fs::read_to_string("/etc/passwd")
            .context("failed to read /etc/passwd")?
            .lines()
            .find(|line| line.starts_with(&format!("{user}:")))
            .map(str::to_string)
            .with_context(|| format!("passwd entry not found for {user}")),
    }
}

fn current_unix_user() -> Result<String> {
    let output = Command::new("id")
        .arg("-un")
        .output()
        .context("failed to determine current user")?;
    if !output.status.success() {
        bail!("failed to determine current user");
    }
    String::from_utf8(output.stdout)
        .context("current user is not valid utf-8")
        .map(|value| value.trim().to_string())
}
