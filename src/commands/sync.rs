use anyhow::Result;

use crate::config::AppConfig;
use crate::{db, playlist};

pub async fn run(cfg: &AppConfig) -> Result<()> {
    let dir = std::env::current_dir()?;
    db::require_existing(&dir)?;
    let pool = db::connect(&dir).await?;

    let (playlist_id, remote_url): (String, String) =
        sqlx::query_as("SELECT youtube_id, remote_url FROM playlist WHERE id = 1")
            .fetch_one(&pool)
            .await?;

    println!("Fetching playlist...");
    let info = playlist::track_remote(&pool, &remote_url, &playlist_id).await?;

    println!("Comparing tracks...");
    let new_count = playlist::upsert_tracks(&pool, &info).await?;
    if new_count > 0 {
        println!("  {new_count} new track(s)");
    }

    playlist::download_pending(&pool, &dir, cfg).await?;

    pool.close().await;
    println!("Sync complete.");
    Ok(())
}
