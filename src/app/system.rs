use crate::context::HostContext;
use anyhow::{Context, Result, bail};
use std::env;
use std::fs;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

pub fn exec_replace(mut command: ProcessCommand) -> Result<()> {
    let error = command.exec();
    bail!("failed to exec command: {error}")
}

pub fn run_sudo<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let args = args
        .into_iter()
        .map(|arg| arg.as_ref().to_string())
        .collect::<Vec<_>>();
    let program = args.first().context("missing command")?.clone();

    let mut command = if is_current_root()? {
        let mut command = ProcessCommand::new(&program);
        for arg in args.iter().skip(1) {
            command.arg(arg);
        }
        command
    } else {
        let mut command = ProcessCommand::new("sudo");
        command.arg(&program);
        for arg in args.iter().skip(1) {
            command.arg(arg);
        }
        command
    };

    let status = command
        .status()
        .context("failed to run privileged command")?;
    if status.success() {
        Ok(())
    } else {
        bail!("privileged command failed with status {status}")
    }
}

pub fn read_command_stdout<const N: usize>(args: [&str; N]) -> Result<String> {
    let mut iter = args.into_iter();
    let program = iter.next().context("missing program")?;
    let output = ProcessCommand::new(program)
        .args(iter)
        .output()
        .with_context(|| format!("failed to execute {program}"))?;
    if !output.status.success() {
        bail!("{program} failed with status {}", output.status);
    }
    String::from_utf8(output.stdout).context("command output is not valid utf-8")
}

pub fn command_exists(command: &str) -> bool {
    ProcessCommand::new("bash")
        .arg("-lc")
        .arg("command -v -- \"$1\" >/dev/null 2>&1")
        .arg("bash")
        .arg(command)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn command_exists_as_target(context: &HostContext, command: &str) -> Result<bool> {
    let status = context
        .command_as_target("bash")
        .arg("-lc")
        .arg("command -v -- \"$1\" >/dev/null 2>&1")
        .arg("bash")
        .arg(command)
        .status()
        .with_context(|| format!("failed to check host command {command}"))?;
    Ok(status.success())
}

pub fn user_in_group(user: &str, group: &str) -> Result<bool> {
    let output = ProcessCommand::new("id")
        .arg("-nG")
        .arg(user)
        .output()
        .with_context(|| format!("failed to query groups for {user}"))?;
    if !output.status.success() {
        bail!("failed to query groups for {user}");
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .any(|value| value == group))
}

pub fn remove_path_if_exists(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || metadata.is_file() {
                fs::remove_file(path)
                    .with_context(|| format!("failed to remove {}", path.display()))?;
            } else {
                fs::remove_dir_all(path)
                    .with_context(|| format!("failed to remove {}", path.display()))?;
            }
            Ok(())
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("failed to stat {}", path.display())),
    }
}

pub fn copy_dir_contents(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create {}", destination.display()))?;
    for entry in
        fs::read_dir(source).with_context(|| format!("failed to read {}", source.display()))?
    {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let metadata = fs::symlink_metadata(&source_path)
            .with_context(|| format!("failed to stat {}", source_path.display()))?;
        if metadata.is_dir() {
            copy_dir_contents(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path).with_context(|| {
                format!(
                    "failed to copy {} to {}",
                    source_path.display(),
                    destination_path.display()
                )
            })?;
        }
    }
    Ok(())
}

pub fn home_dir() -> PathBuf {
    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/"))
}

fn is_current_root() -> Result<bool> {
    let output = ProcessCommand::new("id")
        .arg("-u")
        .output()
        .context("failed to determine effective uid")?;
    if !output.status.success() {
        bail!("failed to determine effective uid");
    }
    Ok(String::from_utf8(output.stdout)
        .context("effective uid is not valid utf-8")?
        .trim()
        == "0")
}
