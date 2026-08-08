use anyhow::{Context, Result};

use crate::db;

pub fn run() -> Result<()> {
    let dir = std::env::current_dir()?;
    let yit_dir = dir.join(db::DB_DIR);

    if !yit_dir.exists() {
        anyhow::bail!("Not a yit playlist (no {} here).", db::DB_DIR);
    }

    std::fs::remove_dir_all(&yit_dir)
        .with_context(|| format!("Failed to remove {}", yit_dir.display()))?;

    println!("Untracked. Downloaded files were left in place.");
    Ok(())
}
