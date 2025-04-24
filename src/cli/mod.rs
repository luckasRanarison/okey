pub mod commands;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Start the keyboard remapping service
    Start {
        #[arg(short, long)]
        config: String,
    },
}
