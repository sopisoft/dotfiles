use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(name = "dotfiles")]
#[command(about = "Dotfiles task runner and runtime helper")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Install {
        #[arg(long, default_value_t = false)]
        skip_host_packages: bool,
    },
    Update,
    Switch,
    Healthcheck,
    Cleanup,
    UpdateFlakeInputs,
    UpdateNeovimPlugins,
    SyncNvimPack,
    Remote {
        #[command(subcommand)]
        command: RemoteCommand,
    },
    Udev {
        #[command(subcommand)]
        command: UdevCommand,
    },
    Backups {
        #[command(subcommand)]
        command: BackupCommand,
    },
    Rollback {
        generation: Option<String>,
    },
    #[command(hide = true)]
    Internal {
        #[command(subcommand)]
        command: InternalCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum BackupCommand {
    List,
    Rollback { generation: Option<String> },
    ImportLegacy,
    Prune,
}

#[derive(Debug, Subcommand)]
pub enum UdevCommand {
    Apply,
    Status,
}

#[derive(Debug, Subcommand)]
pub enum RemoteCommand {
    Apply,
    Status,
}

#[derive(Debug, Subcommand)]
pub enum InternalCommand {
    Hook {
        #[command(subcommand)]
        command: HookCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum HookCommand {
    DispatchEnter {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    EnterLogin {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    InitHook,
}
