use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use nix::unistd;

pub fn write_systemd_service(config: &str) -> Result<()> {
    let dir_path = get_systemd_dir_path()?;
    let file_path = resolve_service_file_path(&dir_path);

    fs::create_dir_all(dir_path)?;
    fs::write(file_path, config)?;

    Ok(())
}

pub fn remove_systemd_service() -> Result<()> {
    let dir_path = get_systemd_dir_path()?;
    let file_path = resolve_service_file_path(&dir_path);

    fs::remove_file(file_path)?;

    Ok(())
}

pub fn get_systemd_dir_path() -> Result<PathBuf> {
    let dir_path = match unistd::geteuid().is_root() {
        true => Path::new("/etc/systemd/system/").to_path_buf(),
        false => Path::new(&env::var("HOME")?).join(".config/systemd/user"),
    };

    Ok(dir_path)
}

pub fn resolve_service_file_path(dir_path: &Path) -> PathBuf {
    dir_path.join("okey.service")
}
