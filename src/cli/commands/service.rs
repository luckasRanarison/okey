use std::env;

use anyhow::{Result, anyhow};

use crate::{cli::utils::systemctl, fs::config as fs};

pub use systemctl::{restart, status, stop};

pub fn start() -> Result<()> {
    let file_path = fs::get_systemd_service_path()?;

    if !file_path.exists() {
        return Err(anyhow!(
            "The systemd service is not installed, run 'okey service install'",
        ));
    }

    systemctl::reload_daemon()?;
    systemctl::start()?;

    Ok(())
}

pub fn install() -> Result<()> {
    let exe_path = env::current_exe()?;
    let exe_path_str = exe_path.to_string_lossy();

    let config = format!(
        r#"[Unit]
Description=Okey Service

[Service]
ExecStart={exe_path_str} start
Restart=on-failure
Nice=-10
IOSchedulingClass=real-time
IOSchedulingPriority=0

[Install]
WantedBy=multi-user.target"#
    );

    fs::write_systemd_service(&config)?;

    println!("The systemd service has been installed, run 'okey service start' to start it");

    Ok(())
}

pub fn uninstall() -> Result<()> {
    let file_path = fs::get_systemd_service_path()?;

    if file_path.exists() {
        systemctl::stop()?;
        fs::remove_systemd_service()?;
        systemctl::reload_daemon()?;

        println!("The systemd service has been removed");
    } else {
        println!("The systemd service does not exist");
    }

    Ok(())
}
