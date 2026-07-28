use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub download_path: String,
    pub format: String,
}

impl Default for Config {
    fn default() -> Self {
        let music_dir = dirs::audio_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap().join("Music"))
            .join("yit");

        Self {
            download_path: music_dir.to_string_lossy().to_string(),
            format: "opus".to_string(),
        }
    }
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .expect("could not find config directory")
        .join("yit")
        .join("config.toml")
}

pub fn load() -> Result<Config> {
    let path = config_path();

    if !path.exists() {
        let config = Config::default();
        save(&config)?;
        return Ok(config);
    }

    let content = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save(config: &Config) -> Result<()> {
    let path = config_path();
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, toml::to_string_pretty(config)?)?;
    Ok(())
}
