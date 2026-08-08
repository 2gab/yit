use anyhow::Result;
use sqlx::SqlitePool;

use crate::youtube;

pub async fn run(pool: &SqlitePool) -> Result<()> {
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

        let info = youtube::fetch_playlist(playlist_yt_id)?;
        let mut new_count = 0u32;

        for (position, entry) in info.entries.iter().enumerate() {
            // Skip deleted/private videos
            if entry.title == "[Deleted video]" || entry.title == "[Private video]" {
                continue;
            }

            let thumbnail = youtube::thumbnail_url(&entry.thumbnails);

            let inserted = sqlx::query(
                "INSERT INTO tracks (youtube_id, title, artist, position, thumbnail, playlist_id)
                 VALUES (?, ?, ?, ?, ?, ?)
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
            .bind(playlist_db_id)
            .execute(pool)
            .await?;

            if inserted.rows_affected() == 1 {
                new_count += 1;
            }
        }

        println!("  {} tracks ({new_count} new)", info.entries.len());
    }

    Ok(())
}
