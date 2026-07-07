mod container;
mod container_setup;
mod hazkey;
mod healthcheck;
mod hooks;
mod host;
mod nvim;
mod system;
mod udev;

use crate::backup;
use crate::cli::{
    BackupCommand, Cli, Command, ContainerCommand, HookCommand, InternalCommand, UdevCommand,
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
        Command::Install {
            skip_host_packages,
            skip_ros_jazzy,
        } => host::install(&context, skip_host_packages, skip_ros_jazzy),
        Command::Update { skip_ros_jazzy } => host::update(&context, skip_ros_jazzy),
        Command::Switch => host::switch(&context),
        Command::Jazzy { args } => host::jazzy(&context, &args),
        Command::Healthcheck => healthcheck::host(&context),
        Command::Cleanup => host::cleanup(&context),
        Command::InstallRosJazzy => host::install_ros_jazzy(&context),
        Command::UpdateRosJazzy => host::update_ros_jazzy(&context),
        Command::InstallHazkey => hazkey::install(&context),
        Command::UpdateFlakeInputs => host::update_flake_inputs(&context),
        Command::UpdateNeovimPlugins => nvim::update_neovim_plugins(),
        Command::SyncNvimPack => nvim::sync_nvim_pack(),
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

fn dispatch_internal(context: &HostContext, command: InternalCommand) -> Result<()> {
    match command {
        InternalCommand::Container { command } => match command {
            ContainerCommand::Install => container::install_ros_jazzy_internal(),
            ContainerCommand::Update => container::update_ros_jazzy_internal(),
            ContainerCommand::Healthcheck => healthcheck::container_ros_jazzy(context),
            ContainerCommand::Cleanup => container::cleanup_ros_jazzy_internal(),
        },
        InternalCommand::Hook { command } => match command {
            HookCommand::DispatchEnter { args } | HookCommand::EnterLogin { args } => {
                hooks::run_enter_login(&args.iter().map(OsString::from).collect::<Vec<_>>())
            }
            HookCommand::InitHook => hooks::run_init_hook(),
        },
    }
}
