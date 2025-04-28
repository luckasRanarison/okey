use anyhow::Result;
use clap::Parser;
use okey::cli::{Cli, Command, SystemdSubcommand, commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Start {
            config,
            daemon,
            systemd: _,
        } => match daemon {
            true => commands::start::start_daemon(config),
            false => commands::start::start(config),
        },

        Command::Service { command } => match command {
            SystemdSubcommand::Start => commands::service::start(),
            SystemdSubcommand::Restart => commands::service::restart(),
            SystemdSubcommand::Stop => commands::service::stop(),
            SystemdSubcommand::Status => commands::service::status(),
            SystemdSubcommand::Install => commands::service::install(),
            SystemdSubcommand::Uninstall => commands::service::uninstall(),
        },
    }
}
