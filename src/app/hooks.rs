use anyhow::{Result, bail};
use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::process::Command as ProcessCommand;

pub fn run_enter_login(args: &[OsString]) -> Result<()> {
    run_init_hook()?;
    if args.is_empty() {
        let error = ProcessCommand::new("zsh").arg("-l").exec();
        bail!("failed to exec zsh: {error}");
    }

    let error = ProcessCommand::new(&args[0]).args(&args[1..]).exec();
    bail!("failed to exec command: {error}")
}

pub fn run_init_hook() -> Result<()> {
    Ok(())
}
