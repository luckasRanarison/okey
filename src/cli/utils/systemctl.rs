use std::process::{Command, Stdio};

use anyhow::Result;
use nix::unistd;

fn systemctl(args: &[&str]) -> Result<()> {
    let args = match unistd::geteuid().is_root() {
        true => args.to_vec(),
        false => [&["--user"], args].concat(),
    };

    Command::new("systemctl")
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Ok(())
}

pub fn reload_daemon() -> Result<()> {
    systemctl(&["daemon-reload"])
}

pub fn start() -> Result<()> {
    systemctl(&["enable", "okey"])?;
    systemctl(&["start", "okey"])
}

pub fn restart() -> Result<()> {
    systemctl(&["restart", "okey"])
}

pub fn stop() -> Result<()> {
    systemctl(&["stop", "okey"])?;
    systemctl(&["disable", "okey"])
}

pub fn status() -> Result<()> {
    let args = match unistd::geteuid().is_root() {
        true => vec!["status", "okey"],
        false => vec!["--user", "status", "okey"],
    };

    Command::new("systemctl").args(args).status()?;

    Ok(())
}
