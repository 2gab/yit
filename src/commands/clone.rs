use anyhow::{Context, Result};
use std::path::Path;

use crate::config::AppConfig;
use crate::{db, playlist, youtube};

pub async fn run(url: &str, cfg: &AppConfig) -> Result<()> {
    let playlist_id =
        youtube::extract_playlist_id(url).context("Invalid YouTube playlist URL or ID.")?;

    let dir = Path::new(&playlist_id);
    if dir.exists() {
        anyhow::bail!("Directory '{playlist_id}' already exists.");
    }

    println!("Cloning {playlist_id}...");
    std::fs::create_dir(dir)?;

    if let Err(e) = clone_inner(dir, url, &playlist_id, cfg).await {
        let _ = std::fs::remove_dir_all(dir);
        return Err(e);
    }

    println!("Cloned into {playlist_id}/");
    Ok(())
}

async fn clone_inner(dir: &Path, url: &str, playlist_id: &str, cfg: &AppConfig) -> Result<()> {
    let pool = db::connect(dir).await?;
    let info = playlist::track_remote(&pool, url, playlist_id).await?;
    playlist::upsert_tracks(&pool, &info).await?;
    playlist::download_pending(&pool, dir, cfg).await?;
    pool.close().await;
    Ok(())
}
