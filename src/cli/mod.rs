pub mod commands;

mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Start the keyboard remapping hook
    Start {
        #[arg(short, long)]
        config: Option<String>,
    },

    /// Utility commands for the systemd service
    Service {
        #[command(subcommand)]
        command: SystemdSubcommand,
    },
}

#[derive(Parser, Debug)]
pub enum SystemdSubcommand {
    /// Shorthand for 'systemctl --user start okey'
    Start,
    /// Shorthand for 'systemctl --user restart okey'
    Restart,
    /// Shorthand for 'systemctl --user stop okey'
    Stop,
    /// Shorthand for 'systemctl --user status okey'
    Status,
    /// Create the systemd service file
    Install,
    /// Disable and remove the service file
    Uninstall,
}
