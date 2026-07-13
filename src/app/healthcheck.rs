use super::system;
use crate::context::HostContext;
use anyhow::{Result, bail};

const HOST_COMMANDS: &[&str] = &[
    "cargo", "nix", "node", "nvim", "pixi", "rustc", "dotfiles", "x11vnc",
];
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
    if let Err(error) = super::remote::status(context) {
        eprintln!("[fail] remote desktop status check failed: {error:#}");
        ok = false;
    }

    if ok {
        Ok(())
    } else {
        bail!("healthcheck failed")
    }
}
