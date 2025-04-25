use std::process;

use anyhow::Result;
use clap::Parser;
use okey::{
    cli::{Cli, Command, SystemdSubcommand, commands},
    log,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Start { config, daemon } => match daemon {
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
    };

    if let Err(error) = result {
        eprintln!("Error: {error}");
        log::write_log("ERROR", &error)?;
        process::exit(1);
    }

    Ok(())
}
