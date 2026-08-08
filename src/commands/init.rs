use anyhow::{Context, Result};

use crate::{db, playlist, youtube};

pub async fn run(url: &str) -> Result<()> {
    let dir = std::env::current_dir()?;

    if db::db_path(&dir).exists() {
        anyhow::bail!("Already a yit playlist ({}/{} exists).", db::DB_DIR, db::DB_FILENAME);
    }

    let playlist_id =
        youtube::extract_playlist_id(url).context("Invalid YouTube playlist URL or ID.")?;

    let pool = db::connect(&dir).await?;
    let info = playlist::track_remote(&pool, url, &playlist_id).await?;
    let new_count = playlist::upsert_tracks(&pool, &info).await?;
    pool.close().await;

    println!("Initialized yit playlist: {}", info.title);
    println!("  {new_count} track(s) found. Run `yit sync` to download.");
    Ok(())
}
