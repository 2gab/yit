use anyhow::{Context, Result};

use crate::db;

pub fn run() -> Result<()> {
    let dir = std::env::current_dir()?;
    let db_path = dir.join(db::DB_FILENAME);

    if !db_path.exists() {
        anyhow::bail!("Not a yit playlist (no {} here).", db::DB_FILENAME);
    }

    std::fs::remove_file(&db_path)
        .with_context(|| format!("Failed to remove {}", db_path.display()))?;

    println!("Untracked. Downloaded files were left in place.");
    Ok(())
}
