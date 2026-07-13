use crate::app::system;
use crate::context::HostContext;
use anyhow::{Result, bail};

const AUTOSTART_FILE: &str = ".config/autostart/x11vnc.desktop";
const START_SCRIPT: &str = ".local/bin/start-vnc-session";

pub fn apply(_context: &HostContext) -> Result<()> {
    Ok(())
}

pub fn status(context: &HostContext) -> Result<()> {
    let autostart = context.target_home.join(AUTOSTART_FILE);
    if !autostart.exists() {
        bail!("missing VNC autostart file: {}", autostart.display());
    }

    let script = context.target_home.join(START_SCRIPT);
    if !script.exists() {
        bail!("missing VNC launch script: {}", script.display());
    }

    if !system::command_exists_as_target(context, "x11vnc")? {
        bail!("x11vnc is not available in the user environment");
    }

    println!("[ok] VNC integration installed");
    Ok(())
}
