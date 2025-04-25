use anyhow::Result;
use clap::Parser;
use okey::cli::{Cli, Command, SystemdSubcommand, commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Start { config } => commands::start(config),

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
