mod app;
mod backup;
mod cli;
mod context;

use anyhow::Result;

fn main() -> Result<()> {
    app::run()
}
