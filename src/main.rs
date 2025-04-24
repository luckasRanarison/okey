use anyhow::Result;
use clap::Parser;
use okey::cli::{Cli, Command, commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Start { config } => commands::start(&config),
    }
}
