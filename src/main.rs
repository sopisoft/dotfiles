mod app;
mod backup;
mod cli;
mod context;
mod home_manager;

use anyhow::Result;

fn main() -> Result<()> {
    app::run()
}
