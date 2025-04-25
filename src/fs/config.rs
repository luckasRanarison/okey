use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};

use crate::config::schema::Config;

pub fn get_config_dir_path() -> Result<String> {
    let home_path = env::var("HOME")?;
    let default_path = Path::new(&home_path).join(".config/okey");
    let default_path_str = default_path.to_string_lossy().to_string();

    Ok(default_path_str)
}

pub fn get_default_config_path() -> Result<String> {
    let config_dir_path = get_config_dir_path()?;
    let default_path = Path::new(&config_dir_path).join("config.yaml");
    let default_path_str = default_path.to_string_lossy().to_string();

    Ok(default_path_str)
}

pub fn read_config(path: Option<String>) -> Result<Config> {
    let config_path = path
        .unwrap_or_else(|| get_default_config_path().expect("Failed to get default config path"));

    let parsed = fs::read_to_string(&config_path)
        .map_err(|err| match err.kind() {
            io::ErrorKind::NotFound => anyhow!("Configuration file not found at {config_path}"),
            _ => err.into(),
        })
        .and_then(|config| Ok(serde_yaml::from_str(&config)?))?;

    Ok(parsed)
}

pub fn write_systemd_service(config: &str) -> Result<()> {
    let home_path = env::var("HOME")?;
    let dir_path = Path::new(&home_path).join(".config/systemd/user");
    let file_path = dir_path.join("okey.service");

    fs::create_dir_all(dir_path)?;
    fs::write(file_path, config)?;

    Ok(())
}

pub fn get_systemd_service_path() -> Result<PathBuf> {
    let home_path = env::var("HOME")?;
    let file_path = Path::new(&home_path).join(".config/systemd/user/okey.service");

    Ok(file_path)
}

pub fn remove_systemd_service() -> Result<()> {
    let file_path = get_systemd_service_path()?;

    fs::remove_file(file_path)?;

    Ok(())
}
