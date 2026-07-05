use crate::backup;
use crate::context::HostContext;
use anyhow::{Context, Result, bail};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn apply_home_manager(context: &HostContext) -> Result<()> {
    backup::prepare_for_activation(context)?;

    let nix_bin = resolve_nix_bin(context)?;
    let nix_config = nix_config_from_environment();
    let mut build = context.command_as_target(&nix_bin);
    build.env("NIX_CONFIG", &nix_config);
    build.arg("build");
    build.arg(format!(
        "path:{}#homeConfigurations.{}.activationPackage",
        context.repo_root.display(),
        context.target_user
    ));
    build.arg("--no-link");
    build.arg("--print-out-paths");
    build.stdout(Stdio::piped());
    let output = build
        .output()
        .context("failed to build activation package")?;
    if !output.status.success() {
        bail!(
            "failed to build activation package:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let activation_package = String::from_utf8(output.stdout)
        .context("activation package path is not valid utf-8")?
        .lines()
        .last()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(PathBuf::from)
        .context("activation package path is missing")?;

    let mut activate = context.command_as_target(activation_package.join("activate"));
    let status = activate
        .status()
        .context("failed to run Home Manager activation")?;
    context.status_ok(status, "Home Manager activation")
}

pub fn nix_config_from_environment() -> String {
    let max_jobs = std::env::var("NIX_MAX_JOBS").unwrap_or_else(|_| String::from("1"));
    let build_cores = std::env::var("NIX_BUILD_CORES").unwrap_or_else(|_| String::from("1"));
    let eval_cores = std::env::var("NIX_EVAL_CORES").unwrap_or_else(|_| String::from("1"));
    let managed = format!(
        "experimental-features = nix-command flakes\naccept-flake-config = true\nmax-jobs = {max_jobs}\ncores = {build_cores}\neval-cores = {eval_cores}"
    );
    match std::env::var("NIX_CONFIG") {
        Ok(existing) if !existing.trim().is_empty() => format!("{existing}\n{managed}"),
        _ => managed,
    }
}

pub fn flake_ref(context: &HostContext) -> String {
    format!("path:{}", context.repo_root.display())
}

pub fn resolve_nix_bin(context: &HostContext) -> Result<PathBuf> {
    let candidates = [
        std::env::var_os("NIX_BIN").map(PathBuf::from),
        Some(PathBuf::from("/nix/var/nix/profiles/default/bin/nix")),
        Some(context.target_home.join(".nix-profile/bin/nix")),
    ];

    for candidate in candidates.into_iter().flatten() {
        if candidate.is_file() {
            return Ok(candidate);
        }
    }

    if context.is_target_user() {
        let output = Command::new("sh")
            .arg("-lc")
            .arg("command -v nix")
            .output()
            .context("failed to locate nix")?;
        if output.status.success() {
            let path = String::from_utf8(output.stdout).context("nix path is not valid utf-8")?;
            let path = path.trim();
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
    }

    bail!("nix binary not found")
}
