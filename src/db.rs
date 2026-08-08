use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::path::Path;
use std::str::FromStr;

pub const DB_FILENAME: &str = "yithub.db";

pub async fn connect(dir: &Path) -> Result<SqlitePool> {
    let db_path = dir.join(DB_FILENAME);
    let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path.display()))?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub fn require_existing(dir: &Path) -> Result<()> {
    if !dir.join(DB_FILENAME).exists() {
        anyhow::bail!(
            "Not a yit playlist (no {DB_FILENAME} here). Run `yit init <url>` or `yit clone <url>` first."
        );
    }
    Ok(())
}
