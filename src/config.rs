use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub download_path: String,
    pub format: String,
    pub google: GoogleConfig,
}

impl AppConfig {
    pub fn download_path_expanded(&self) -> PathBuf {
        if self.download_path.starts_with("~/") {
            dirs::home_dir()
                .unwrap()
                .join(&self.download_path[2..])
        } else {
            PathBuf::from(&self.download_path)
        }
    }
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .expect("could not find config directory")
        .join("yit")
        .join("config.toml")
}

pub fn load() -> Result<AppConfig> {
    let path = config_path();

    if !path.exists() {
        create_default(&path)?;
        anyhow::bail!(
            "Config file created at {}.\nPlease fill in your Google credentials before continuing.",
            path.display()
        );
    }

    let content = fs::read_to_string(&path)?;
    let config: AppConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))?;

    Ok(config)
}

fn create_default(path: &Path) -> Result<()> {
    let default = r#"download_path = "~/Music/yit"
format = "opus"

[google]
client_id = ""
client_secret = ""
"#;
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, default)?;
    Ok(())
}
