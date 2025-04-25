use std::process::{Command, Stdio};

use anyhow::Result;

fn systemctl(args: &[&str]) -> Result<()> {
    Command::new("systemctl")
        .arg("--user")
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
    Command::new("systemctl")
        .args(&["--user", "status", "okey"])
        .status()?;

    Ok(())
}
