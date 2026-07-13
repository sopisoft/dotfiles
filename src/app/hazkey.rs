use crate::context::HostContext;
use crate::home_manager;
use anyhow::Result;

pub fn install(context: &HostContext) -> Result<()> {
    context.log("Hazkey is managed by Home Manager");
    home_manager::apply_home_manager(context)
}
