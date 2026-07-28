use anyhow::{Context, Result};
use sqlx::SqlitePool;

use crate::auth;
use crate::youtube;

pub async fn add(pool: &SqlitePool, url: &str) -> Result<()> {
    let tokens = auth::load_tokens().context("Not logged in. Run `yit login` first.")?;

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

    let user_id = get_or_create_user(pool, &tokens).await?;
    let yt = youtube::fetch_playlist(&playlist_id, &tokens.access_token).await?;
    let thumbnail = yt.snippet.thumbnails.default.map(|t| t.url);

    sqlx::query(
        "INSERT INTO playlists (youtube_id, title, description, thumbnail, user_id)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&yt.id)
    .bind(&yt.snippet.title)
    .bind(&yt.snippet.description)
    .bind(&thumbnail)
    .bind(user_id)
    .execute(pool)
    .await?;

    println!("Tracking: {}", yt.snippet.title);
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

async fn get_or_create_user(pool: &SqlitePool, tokens: &auth::Tokens) -> Result<i64> {
    let existing = sqlx::query_as::<_, (i64,)>(
        "SELECT id FROM users WHERE email = ?",
    )
    .bind(&tokens.email)
    .fetch_optional(pool)
    .await?;

    if let Some((id,)) = existing {
        sqlx::query("UPDATE users SET access_token = ? WHERE id = ?")
            .bind(&tokens.access_token)
            .bind(id)
            .execute(pool)
            .await?;
        return Ok(id);
    }

    let result = sqlx::query(
        "INSERT INTO users (email, name, access_token, refresh_token)
         VALUES (?, ?, ?, ?)",
    )
    .bind(&tokens.email)
    .bind(&tokens.name)
    .bind(&tokens.access_token)
    .bind(&tokens.refresh_token)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
