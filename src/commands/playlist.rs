use anyhow::{Context, Result};
use sqlx::SqlitePool;

use crate::youtube;

pub async fn add(pool: &SqlitePool, url: &str) -> Result<()> {
    let playlist_id = youtube::extract_playlist_id(url)
        .context("Invalid YouTube playlist URL or ID.")?;

    let existing = sqlx::query("SELECT id FROM playlists WHERE youtube_id = ?")
        .bind(&playlist_id)
        .fetch_optional(pool)
        .await?;

    if existing.is_some() {
        println!("Playlist already tracked.");
        return Ok(());
    }

    let yt = youtube::fetch_playlist(&playlist_id)?;
    let thumbnail = youtube::thumbnail_url(&yt.thumbnails);

    sqlx::query(
        "INSERT INTO playlists (youtube_id, title, thumbnail)
         VALUES (?, ?, ?)",
    )
    .bind(&yt.id)
    .bind(&yt.title)
    .bind(&thumbnail)
    .execute(pool)
    .await?;

    println!("Tracking: {}", yt.title);
    Ok(())
}

pub async fn ls(pool: &SqlitePool) -> Result<()> {
    let rows = sqlx::query_as::<_, (String, String, i64, i64)>(
        "SELECT p.youtube_id, p.title,
                COUNT(t.id),
                SUM(CASE WHEN d.status = 'done' THEN 1 ELSE 0 END)
         FROM playlists p
         LEFT JOIN tracks t ON t.playlist_id = p.id
         LEFT JOIN downloads d ON d.track_id = t.id
         GROUP BY p.id
         ORDER BY p.created_at DESC",
    )
    .fetch_all(pool)
    .await?;

    if rows.is_empty() {
        println!("No playlists tracked. Use `yit add <url>` to add one.");
        return Ok(());
    }

    for (youtube_id, title, total, done) in &rows {
        println!("{youtube_id} [{done}/{total}] {title}");
    }

    Ok(())
}
