use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub const DB_DIR: &str = ".yit";
pub const DB_FILENAME: &str = "yithub.db";

pub fn db_path(dir: &Path) -> PathBuf {
    dir.join(DB_DIR).join(DB_FILENAME)
}

pub async fn connect(dir: &Path) -> Result<SqlitePool> {
    let db_path = db_path(dir);
    std::fs::create_dir_all(db_path.parent().unwrap())?;

    let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path.display()))?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub fn require_existing(dir: &Path) -> Result<()> {
    if !db_path(dir).exists() {
        anyhow::bail!(
            "Not a yit playlist (no {DB_DIR}/{DB_FILENAME} here). Run `yit init <url>` or `yit clone <url>` first."
        );
    }
    Ok(())
}
