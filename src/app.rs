mod healthcheck;
mod hooks;
mod host;
mod nvim;
mod remote;
mod system;
mod udev;

use crate::backup;
use crate::cli::{
    BackupCommand, Cli, Command, HookCommand, InternalCommand, RemoteCommand, UdevCommand,
};
use crate::context::HostContext;
use anyhow::Result;
use clap::Parser;
use std::ffi::OsString;

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    dispatch_cli(cli)
}

fn dispatch_cli(cli: Cli) -> Result<()> {
    let context = HostContext::detect()?;
    match cli.command {
        Command::Install { skip_host_packages } => host::install(&context, skip_host_packages),
        Command::Update => host::update(&context),
        Command::Switch => host::switch(&context),
        Command::Healthcheck => healthcheck::host(&context),
        Command::Cleanup => host::cleanup(&context),
        Command::UpdateFlakeInputs => host::update_flake_inputs(&context),
        Command::UpdateNeovimPlugins => nvim::update_neovim_plugins(),
        Command::SyncNvimPack => nvim::sync_nvim_pack(),
        Command::Remote { command } => dispatch_remote(&context, command),
        Command::Udev { command } => dispatch_udev(&context, command),
        Command::Backups { command } => backups(&context, command),
        Command::Rollback { generation } => backup::rollback(&context, generation.as_deref()),
        Command::Internal { command } => dispatch_internal(&context, command),
    }
}

fn dispatch_udev(context: &HostContext, command: UdevCommand) -> Result<()> {
    match command {
        UdevCommand::Apply => udev::apply(context),
        UdevCommand::Status => udev::status(context),
    }
}

fn dispatch_remote(context: &HostContext, command: RemoteCommand) -> Result<()> {
    match command {
        RemoteCommand::Apply => remote::apply(context),
        RemoteCommand::Status => remote::status(context),
    }
}

fn backups(context: &HostContext, command: BackupCommand) -> Result<()> {
    match command {
        BackupCommand::List => backup::print_backup_list(context),
        BackupCommand::Rollback { generation } => backup::rollback(context, generation.as_deref()),
        BackupCommand::ImportLegacy => {
            backup::import_legacy_backups(context)?;
            backup::prune_backups(context)
        }
        BackupCommand::Prune => backup::prune_backups(context),
    }
}

fn dispatch_internal(_context: &HostContext, command: InternalCommand) -> Result<()> {
    match command {
        InternalCommand::Hook { command } => match command {
            HookCommand::DispatchEnter { args } | HookCommand::EnterLogin { args } => {
                hooks::run_enter_login(&args.iter().map(OsString::from).collect::<Vec<_>>())
            }
            HookCommand::InitHook => hooks::run_init_hook(),
        },
    }
}
