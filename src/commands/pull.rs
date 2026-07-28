use anyhow::{Context, Result};
use sqlx::SqlitePool;
use std::process::Command;

use crate::config::AppConfig;

pub async fn run(pool: &SqlitePool, cfg: &AppConfig) -> Result<()> {
    let pending = sqlx::query_as::<_, (i64, String, String)>(
        "SELECT t.id, t.youtube_id, t.title
         FROM tracks t
         LEFT JOIN downloads d ON d.track_id = t.id
         WHERE d.id IS NULL OR d.status = 'pending' OR d.status = 'error'
         ORDER BY t.position",
    )
    .fetch_all(pool)
    .await?;

    if pending.is_empty() {
        println!("Nothing to download.");
        return Ok(());
    }

    println!("{} track(s) to download.", pending.len());

    let download_dir = cfg.download_path_expanded();
    std::fs::create_dir_all(&download_dir)?;

    for (track_id, video_id, title) in &pending {
        print!("Downloading: {title} ... ");

        // Ensure a downloads row exists
        sqlx::query(
            "INSERT INTO downloads (track_id, status)
             VALUES (?, 'pending')
             ON CONFLICT(track_id) DO UPDATE SET status = 'pending', error = NULL, updated_at = datetime('now')",
        )
        .bind(track_id)
        .execute(pool)
        .await?;

        let url = format!("https://www.youtube.com/watch?v={video_id}");
        let output_template = download_dir.join("%(title)s.%(ext)s");

        let result = Command::new("yt-dlp")
            .args([
                "-x",
                "--audio-format",
                &cfg.format,
                "--audio-quality",
                "0",
                "-o",
                output_template.to_str().unwrap(),
                &url,
            ])
            .output()
            .context("yt-dlp not found. Install it with: pip install yt-dlp")?;

        if result.status.success() {
            sqlx::query(
                "UPDATE downloads SET status = 'done', updated_at = datetime('now') WHERE track_id = ?",
            )
            .bind(track_id)
            .execute(pool)
            .await?;
            println!("done");
        } else {
            let stderr = String::from_utf8_lossy(&result.stderr);
            let error_msg = stderr.lines().last().unwrap_or("unknown error").to_string();

            sqlx::query(
                "UPDATE downloads SET status = 'error', error = ?, updated_at = datetime('now') WHERE track_id = ?",
            )
            .bind(&error_msg)
            .bind(track_id)
            .execute(pool)
            .await?;
            println!("error: {error_msg}");
        }
    }

    Ok(())
}
