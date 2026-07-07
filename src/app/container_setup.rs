use super::{hooks, system};
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, Stdio};
use tempfile::tempdir;

const MINIMAL_PACKAGES: &[&str] = &["ca-certificates", "curl", "git", "sudo", "zsh"];

const BASE_PACKAGES: &[&str] = &[
    "bash-completion",
    "build-essential",
    "cmake",
    "gnupg",
    "iputils-ping",
    "less",
    "locales",
    "lsb-release",
    "pkg-config",
    "software-properties-common",
    "udev",
    "usbutils",
];

const ROS_TOOL_PACKAGES: &[&str] = &[
    "python3-argcomplete",
    "python3-colcon-common-extensions",
    "python3-rosdep",
    "python3-vcstool",
];

pub fn install_ros_jazzy() -> Result<()> {
    hooks::run_init_hook()?;
    ensure_ros_apt_source()?;
    ensure_universe_repo()?;
    ensure_minimal_packages()?;
    ensure_base_packages()?;
    ensure_ros_tool_packages()?;
    ensure_locale()?;
    ensure_ros_packages()?;
    ensure_ros_profile()?;
    ensure_rosdep()
}

pub fn update_ros_jazzy() -> Result<()> {
    ensure_ros_apt_source()?;
    ensure_universe_repo()?;
    ensure_minimal_packages()?;
    ensure_base_packages()?;
    ensure_ros_tool_packages()?;
    apt_update()?;
    system::run_sudo(["apt-get", "upgrade", "-y"])?;
    ensure_locale()?;
    ensure_ros_packages()?;
    ensure_ros_profile()?;
    ensure_rosdep()
}

pub fn cleanup_ros_jazzy() -> Result<()> {
    apt_update()?;
    system::run_sudo(["apt-get", "autoremove", "-y"])?;
    system::run_sudo(["apt-get", "clean"])
}

fn ensure_minimal_packages() -> Result<()> {
    apt_install_if_missing(MINIMAL_PACKAGES)
}

fn ensure_base_packages() -> Result<()> {
    apt_install_if_missing(BASE_PACKAGES)
}

fn ensure_ros_tool_packages() -> Result<()> {
    apt_install_if_missing(ROS_TOOL_PACKAGES)
}

fn ensure_locale() -> Result<()> {
    let locales = system::read_command_stdout(["locale", "-a"]).unwrap_or_default();
    if !locales
        .lines()
        .any(|line| line.eq_ignore_ascii_case("en_US.utf8"))
    {
        system::run_sudo(["locale-gen", "en_US.UTF-8"])?;
    }
    system::run_sudo(["update-locale", "LANG=en_US.UTF-8", "LC_ALL=en_US.UTF-8"])
}

fn ensure_universe_repo() -> Result<()> {
    let codename = os_release_value("UBUNTU_CODENAME")?;
    let universe_present = grep_any(
        &["/etc/apt/sources.list", "/etc/apt/sources.list.d"],
        &format!(" {codename} universe"),
    )?;
    if !universe_present {
        system::run_sudo(["add-apt-repository", "-y", "universe"])?;
    }
    Ok(())
}

fn ensure_ros_apt_source() -> Result<()> {
    remove_conflicting_ros_sources()?;
    Ok(())
}

fn ensure_ros_packages() -> Result<()> {
    apt_install_if_missing(&["ros-jazzy-desktop"])
}

fn ensure_ros_profile() -> Result<()> {
    let profile_contents = concat!(
        "if [ -f /opt/ros/jazzy/setup.sh ]; then\n",
        "    if [ -n \"${ROS_DISTRO:-}\" ] && [ \"${ROS_DISTRO:-}\" != \"jazzy\" ]; then\n",
        "        unset AMENT_PREFIX_PATH\n",
        "        unset CMAKE_PREFIX_PATH\n",
        "        unset COLCON_PREFIX_PATH\n",
        "        unset LD_LIBRARY_PATH\n",
        "        unset PKG_CONFIG_PATH\n",
        "        unset PYTHONPATH\n",
        "        unset ROS_DISTRO\n",
        "        unset ROS_PYTHON_VERSION\n",
        "        unset ROS_VERSION\n",
        "    fi\n\n",
        "    . /opt/ros/jazzy/setup.sh\n",
        "fi\n",
    );
    let profile_path = PathBuf::from("/etc/profile.d/ros-jazzy.sh");
    let current = fs::read_to_string(&profile_path).unwrap_or_default();
    if current == profile_contents {
        return Ok(());
    }

    let temp_dir = tempdir().context("failed to create temporary directory")?;
    let temp_file = temp_dir.path().join("ros-jazzy.sh");
    fs::write(&temp_file, profile_contents)
        .with_context(|| format!("failed to write {}", temp_file.display()))?;
    system::run_sudo([
        "install",
        "-D",
        "-m",
        "0644",
        temp_file
            .to_str()
            .context("temporary profile path is not valid utf-8")?,
        "/etc/profile.d/ros-jazzy.sh",
    ])
}

fn ensure_rosdep() -> Result<()> {
    if !Path::new("/etc/ros/rosdep/sources.list.d/20-default.list").exists() {
        system::run_sudo(["rosdep", "init"])?;
    }

    let status = ProcessCommand::new("rosdep")
        .arg("update")
        .status()
        .context("failed to update rosdep")?;
    if status.success() {
        Ok(())
    } else {
        bail!("rosdep update failed with status {status}")
    }
}

fn apt_update() -> Result<()> {
    system::run_sudo(["apt-get", "update"])
}

fn apt_install_if_missing(packages: &[&str]) -> Result<()> {
    let missing = packages
        .iter()
        .filter_map(|package| match package_installed(package) {
            Ok(true) => None,
            Ok(false) => Some(Ok(*package)),
            Err(error) => Some(Err(error)),
        })
        .collect::<Result<Vec<_>>>()?;
    if missing.is_empty() {
        return Ok(());
    }

    apt_update()?;
    let mut args = vec!["apt-get", "install", "-y"];
    args.extend(missing);
    system::run_sudo(args)
}

fn remove_conflicting_ros_sources() -> Result<()> {
    let ros2_list = PathBuf::from("/etc/apt/sources.list.d/ros2.list");
    let ros_keyring_asc = Path::new("/etc/apt/keyrings/ros-archive-keyring.asc");
    let ros_keyring_gpg = Path::new("/etc/apt/keyrings/ros-archive-keyring.gpg");
    if package_installed("ros2-apt-source")? {
        remove_path_if_exists_as_root(&ros2_list)?;
        remove_path_if_exists_as_root(ros_keyring_asc)?;
        remove_path_if_exists_as_root(ros_keyring_gpg)?;
        return Ok(());
    }

    let sources_dir = Path::new("/etc/apt/sources.list.d");
    if sources_dir.exists() {
        for entry in fs::read_dir(sources_dir).context("failed to read /etc/apt/sources.list.d")? {
            let path = entry?.path();
            if path == ros2_list || !path.is_file() {
                continue;
            }
            let contents = fs::read_to_string(&path).unwrap_or_default();
            if contents.contains("packages.ros.org/ros2/ubuntu") {
                remove_path_if_exists_as_root(&path)?;
            }
        }
    }

    let codename = os_release_value("UBUNTU_CODENAME")?;
    let contents = format!(
        "deb [arch={} signed-by=/etc/apt/keyrings/ros-archive-keyring.gpg] http://packages.ros.org/ros2/ubuntu {} main\n",
        system::read_command_stdout(["dpkg", "--print-architecture"])?.trim(),
        codename.trim()
    );
    let current = fs::read_to_string(&ros2_list).unwrap_or_default();
    if current == contents && ros_keyring_gpg.exists() {
        return Ok(());
    }

    if !ros_keyring_gpg.exists() {
        system::run_sudo(["install", "-d", "-m", "0755", "/etc/apt/keyrings"])?;
        remove_path_if_exists_as_root(ros_keyring_asc)?;
        system::run_sudo([
            "curl",
            "-fsSL",
            "https://raw.githubusercontent.com/ros/rosdistro/master/ros.key",
            "-o",
            "/etc/apt/keyrings/ros-archive-keyring.gpg",
        ])?;
    }

    let temp_dir = tempdir().context("failed to create temporary directory")?;
    let temp_file = temp_dir.path().join("ros2.list");
    fs::write(&temp_file, contents)
        .with_context(|| format!("failed to write {}", temp_file.display()))?;
    system::run_sudo([
        "install",
        "-D",
        "-m",
        "0644",
        temp_file
            .to_str()
            .context("temporary ros2.list path is not valid utf-8")?,
        "/etc/apt/sources.list.d/ros2.list",
    ])
}

fn remove_path_if_exists_as_root(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    system::run_sudo([
        "rm",
        "-rf",
        path.to_str().context("path is not valid utf-8")?,
    ])
}

fn package_installed(package: &str) -> Result<bool> {
    let status = ProcessCommand::new("dpkg")
        .arg("-s")
        .arg(package)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .with_context(|| format!("failed to query package {package}"))?;
    Ok(status.success())
}

fn grep_any(paths: &[&str], needle: &str) -> Result<bool> {
    for path in paths.iter().map(Path::new) {
        if path.is_file()
            && fs::read_to_string(path)
                .unwrap_or_default()
                .contains(needle)
        {
            return Ok(true);
        }
        if !path.is_dir() {
            continue;
        }
        for entry in
            fs::read_dir(path).with_context(|| format!("failed to read {}", path.display()))?
        {
            let path = entry?.path();
            if path.is_file()
                && fs::read_to_string(path)
                    .unwrap_or_default()
                    .contains(needle)
            {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn os_release_value(key: &str) -> Result<String> {
    for line in fs::read_to_string("/etc/os-release")
        .context("failed to read /etc/os-release")?
        .lines()
    {
        if let Some(value) = line.strip_prefix(&format!("{key}=")) {
            return Ok(value.trim_matches('"').to_string());
        }
    }
    bail!("missing {key} in /etc/os-release")
}
