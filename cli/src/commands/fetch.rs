use anyhow::{Context, Result};
use sqlx::SqlitePool;

use crate::auth;
use crate::youtube;

pub async fn run(pool: &SqlitePool) -> Result<()> {
    let tokens = auth::load_tokens().context("Not logged in. Run `yit login` first.")?;

    let playlists = sqlx::query_as::<_, (i64, String, String)>(
        "SELECT id, youtube_id, title FROM playlists ORDER BY created_at",
    )
    .fetch_all(pool)
    .await?;

    if playlists.is_empty() {
        println!("No playlists tracked. Use `yit add <url>` to add one.");
        return Ok(());
    }

    for (playlist_db_id, playlist_yt_id, playlist_title) in &playlists {
        println!("Fetching: {playlist_title}");

        let items = youtube::fetch_playlist_items(playlist_yt_id, &tokens.access_token).await?;
        let mut new_count = 0u32;

        for item in &items {
            let s = &item.snippet;
            let video_id = &s.resource_id.video_id;

            // Skip deleted/private videos (YouTube returns "[Deleted video]")
            if s.title == "[Deleted video]" || s.title == "[Private video]" {
                continue;
            }

            let thumbnail = s
                .thumbnails
                .as_ref()
                .and_then(|t| t.default.as_ref())
                .map(|t| t.url.clone());

            let inserted = sqlx::query(
                "INSERT INTO tracks (youtube_id, title, artist, position, thumbnail, playlist_id)
                 VALUES (?, ?, ?, ?, ?, ?)
                 ON CONFLICT(youtube_id) DO UPDATE SET
                     title = excluded.title,
                     position = excluded.position,
                     thumbnail = excluded.thumbnail,
                     updated_at = datetime('now')",
            )
            .bind(video_id)
            .bind(&s.title)
            .bind(&s.video_owner_channel_title)
            .bind(s.position as i64)
            .bind(&thumbnail)
            .bind(playlist_db_id)
            .execute(pool)
            .await?;

            if inserted.rows_affected() == 1 {
                new_count += 1;
            }
        }

        println!("  {} tracks ({new_count} new)", items.len());
    }

    Ok(())
}
