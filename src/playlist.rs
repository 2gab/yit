use anyhow::{Context, Result};
use sqlx::SqlitePool;
use std::collections::HashSet;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::config::AppConfig;
use crate::youtube::{self, PlaylistInfo};

pub struct DiffEntry {
    pub youtube_id: String,
    pub title: String,
    pub position: i64,
}

pub struct TrackDiff {
    pub new: Vec<DiffEntry>,
    pub removed: Vec<DiffEntry>,
}

/// Compares the local `tracks` table against a freshly fetched remote
/// playlist. Read-only: does not touch the database or the filesystem.
pub async fn diff_remote(pool: &SqlitePool, info: &PlaylistInfo) -> Result<TrackDiff> {
    let local: Vec<(String, String, i64)> =
        sqlx::query_as("SELECT youtube_id, title, position FROM tracks")
            .fetch_all(pool)
            .await?;
    let local_ids: HashSet<&str> = local.iter().map(|(id, _, _)| id.as_str()).collect();

    let mut remote_ids: HashSet<String> = HashSet::new();
    let mut new = Vec::new();

    for (position, entry) in info.entries.iter().enumerate() {
        if entry.title == "[Deleted video]" || entry.title == "[Private video]" {
            continue;
        }

        remote_ids.insert(entry.id.clone());

        if !local_ids.contains(entry.id.as_str()) {
            new.push(DiffEntry {
                youtube_id: entry.id.clone(),
                title: entry.title.clone(),
                position: position as i64,
            });
        }
    }

    let removed = local
        .into_iter()
        .filter(|(id, _, _)| !remote_ids.contains(id))
        .map(|(youtube_id, title, position)| DiffEntry { youtube_id, title, position })
        .collect();

    Ok(TrackDiff { new, removed })
}

pub async fn track_remote(
    pool: &SqlitePool,
    remote_url: &str,
    playlist_id: &str,
) -> Result<PlaylistInfo> {
    let info = youtube::fetch_playlist(playlist_id)?;
    let thumbnail = youtube::thumbnail_url(&info.thumbnails);

    sqlx::query(
        "INSERT INTO playlist (id, youtube_id, title, thumbnail, remote_url)
         VALUES (1, ?, ?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
             youtube_id = excluded.youtube_id,
             title = excluded.title,
             thumbnail = excluded.thumbnail,
             remote_url = excluded.remote_url",
    )
    .bind(&info.id)
    .bind(&info.title)
    .bind(&thumbnail)
    .bind(remote_url)
    .execute(pool)
    .await?;

    Ok(info)
}

pub async fn upsert_tracks(pool: &SqlitePool, info: &PlaylistInfo) -> Result<u32> {
    let before: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tracks")
        .fetch_one(pool)
        .await?;

    for (position, entry) in info.entries.iter().enumerate() {
        if entry.title == "[Deleted video]" || entry.title == "[Private video]" {
            continue;
        }

        let thumbnail = youtube::thumbnail_url(&entry.thumbnails);

        sqlx::query(
            "INSERT INTO tracks (youtube_id, title, uploader, position, thumbnail)
             VALUES (?, ?, ?, ?, ?)
             ON CONFLICT(youtube_id) DO UPDATE SET
                 title = excluded.title,
                 position = excluded.position,
                 thumbnail = excluded.thumbnail,
                 updated_at = datetime('now')",
        )
        .bind(&entry.id)
        .bind(&entry.title)
        .bind(&entry.uploader)
        .bind(position as i64)
        .bind(&thumbnail)
        .execute(pool)
        .await?;
    }

    let after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tracks")
        .fetch_one(pool)
        .await?;

    Ok((after - before) as u32)
}

pub async fn download_pending(pool: &SqlitePool, dir: &Path, cfg: &AppConfig) -> Result<()> {
    let pending = sqlx::query_as::<_, (i64, String, String, i64)>(
        "SELECT id, youtube_id, title, position FROM tracks
         WHERE status = 'pending' OR status = 'error'
         ORDER BY position",
    )
    .fetch_all(pool)
    .await?;

    if pending.is_empty() {
        return Ok(());
    }

    println!("Downloading:");

    for (track_id, video_id, title, position) in &pending {
        print!("  {title} ... ");
        std::io::stdout().flush().ok();

        let url = format!("https://www.youtube.com/watch?v={video_id}");
        let output_template = dir.join(format!("{:02} - %(title)s.%(ext)s", position + 1));

        let result = Command::new("yt-dlp")
            .args([
                "-x",
                "--audio-format",
                &cfg.format,
                "--audio-quality",
                "0",
                "--quiet",
                "--no-warnings",
                "--print",
                "after_move:filepath",
                "-o",
                output_template.to_str().unwrap(),
                &url,
            ])
            .output()
            .context("yt-dlp not found. Install it with: pip install yt-dlp")?;

        if result.status.success() {
            let filename = String::from_utf8_lossy(&result.stdout).trim().to_string();

            sqlx::query(
                "UPDATE tracks SET status = 'done', filename = ?, error = NULL, updated_at = datetime('now')
                 WHERE id = ?",
            )
            .bind(&filename)
            .bind(track_id)
            .execute(pool)
            .await?;

            println!("done");
        } else {
            let stderr = String::from_utf8_lossy(&result.stderr);
            let error_msg = stderr.lines().last().unwrap_or("unknown error").to_string();

            sqlx::query(
                "UPDATE tracks SET status = 'error', error = ?, updated_at = datetime('now') WHERE id = ?",
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
