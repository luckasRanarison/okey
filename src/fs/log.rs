use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use anyhow::Result;
use jiff::Timestamp;

pub fn write_log<T: std::fmt::Display>(level: &str, content: &T) -> Result<()> {
    let now = Timestamp::now();
    let home_path = env::var("HOME")?;
    let log_dir_path = Path::new(&home_path).join(".local/share/okey");
    let log_path = log_dir_path.join("okey.log");

    fs::create_dir_all(log_dir_path)?;

    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    writeln!(log_file, "[{level}][{now}] {content}")?;

    Ok(())
}
