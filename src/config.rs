use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub format: String,
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
    }

    let content = fs::read_to_string(&path)?;
    let config: AppConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))?;

    Ok(config)
}

fn create_default(path: &Path) -> Result<()> {
    let default = r#"format = "opus"
"#;
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, default)?;
    Ok(())
}
