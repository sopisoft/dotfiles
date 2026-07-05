use super::{container, system};
use crate::context::{HostContext, ROS_JAZZY_CONTAINER_NAME, host_path_for_container};
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;
use std::process::{Command as ProcessCommand, Stdio};

const HOST_COMMANDS: &[&str] = &[
    "cargo",
    "distrobox",
    "distrobox-assemble",
    "nix",
    "node",
    "nvim",
    "podman",
    "rustc",
    "xtask",
];

const CONTAINER_COMMANDS: &[&str] = &["ros2", "colcon", "rosdep", "nvim", "distrobox-host-exec"];

pub fn host(context: &HostContext) -> Result<()> {
    let mut ok = true;

    for command in HOST_COMMANDS {
        if system::command_exists_as_target(context, command)? {
            println!("[ok] host command available: {command}");
        } else {
            eprintln!("[fail] host command missing: {command}");
            ok = false;
        }
    }

    if system::user_in_group(&context.target_user, "dialout")? {
        println!("[ok] user is in group: dialout");
    } else {
        eprintln!(
            "[fail] user is not in group: dialout (run: sudo usermod -aG dialout {})",
            context.target_user
        );
        ok = false;
    }

    if let Err(error) = super::udev::status(context) {
        eprintln!("[fail] udev status check failed: {error:#}");
        ok = false;
    }

    if container::container_exists(context)? {
        println!("[ok] distrobox container exists: {ROS_JAZZY_CONTAINER_NAME}");
        if let Err(error) =
            container::run_container_command(context, &["internal", "container", "healthcheck"])
        {
            eprintln!("[fail] container healthcheck failed: {error:#}");
            ok = false;
        } else {
            println!("[ok] container healthcheck passed");
        }
    } else {
        eprintln!("[fail] distrobox container missing: {ROS_JAZZY_CONTAINER_NAME}");
        ok = false;
    }

    if ok {
        Ok(())
    } else {
        bail!("healthcheck failed")
    }
}

pub fn container_ros_jazzy(context: &HostContext) -> Result<()> {
    let mut ok = true;

    for command in CONTAINER_COMMANDS {
        if system::command_exists(command) {
            println!("[ok] container command available: {command}");
        } else {
            eprintln!("[fail] container command missing: {command}");
            ok = false;
        }
    }

    let host_git = host_path_for_container(Path::new("/usr/bin/git"))?;
    let host_nvim = host_path_for_container(&context.target_home.join(".local/bin/nvim"))?;
    ok &= run_status_check("ros2 CLI works", {
        let mut command = ProcessCommand::new("bash");
        command.arg("-lc").arg("ros2 --help >/dev/null");
        command
    });
    ok &= run_status_check("colcon CLI works", {
        let mut command = ProcessCommand::new("bash");
        command.arg("-lc").arg("colcon --help >/dev/null");
        command
    });
    ok &= run_status_check("rosdep CLI works", {
        let mut command = ProcessCommand::new("bash");
        command
            .arg("-lc")
            .arg("python3 -m rosdep2 --help >/dev/null");
        command
    });
    ok &= run_status_check("host git path is callable", {
        let mut command = ProcessCommand::new(&host_git);
        command.arg("--version");
        command
    });
    ok &= run_status_check("host nvim is callable", {
        let mut command = ProcessCommand::new(&host_nvim);
        command.arg("--version");
        command
    });
    ok &= run_status_check("container nvim wrapper works", {
        let mut command = ProcessCommand::new("nvim");
        command.arg("--version");
        command
    });
    ok &= run_status_check("container login zsh loads managed config", {
        let mut command = ProcessCommand::new("zsh");
        command.arg("-lic").arg("exit");
        command
    });
    ok &= run_status_check("host Nix store is visible in container", {
        let mut command = ProcessCommand::new("test");
        command.arg("-d").arg("/nix/store");
        command
    });

    for prefix in ["ttyUSB", "ttyACM"] {
        report_device_visibility(prefix).context("failed to inspect /dev")?;
    }

    if ok {
        Ok(())
    } else {
        bail!("container healthcheck failed")
    }
}

fn run_status_check(label: &str, mut command: ProcessCommand) -> bool {
    command.stdout(Stdio::null());
    command.stderr(Stdio::null());
    match command.status() {
        Ok(status) if status.success() => {
            println!("[ok] {label}");
            true
        }
        _ => {
            eprintln!("[fail] {label}");
            false
        }
    }
}

fn report_device_visibility(prefix: &str) -> Result<()> {
    let matches = fs::read_dir("/dev")
        .context("failed to read /dev")?
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.starts_with(prefix))
        .collect::<Vec<_>>();

    if matches.is_empty() {
        println!("[info] /dev/{prefix}*: no devices currently connected");
    } else {
        println!("[ok] /dev/{prefix}*: {}", matches.join(" "));
    }
    Ok(())
}
