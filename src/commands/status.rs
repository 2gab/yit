use anyhow::Result;
use std::collections::HashSet;

use crate::{db, youtube};

pub async fn run() -> Result<()> {
    let dir = std::env::current_dir()?;
    db::require_existing(&dir)?;
    let pool = db::connect(&dir).await?;

    let (title, youtube_id, remote_url): (String, String, String) =
        sqlx::query_as("SELECT title, youtube_id, remote_url FROM playlist WHERE id = 1")
            .fetch_one(&pool)
            .await?;

    println!("Playlist: {title}");
    println!("Remote:   {remote_url}");
    println!();

    let info = youtube::fetch_playlist(&youtube_id)?;

    let local_ids: HashSet<String> = sqlx::query_scalar("SELECT youtube_id FROM tracks")
        .fetch_all(&pool)
        .await?
        .into_iter()
        .collect();

    let remote_entries: Vec<_> = info
        .entries
        .iter()
        .filter(|e| e.title != "[Deleted video]" && e.title != "[Private video]")
        .collect();
    let remote_ids: HashSet<&str> = remote_entries.iter().map(|e| e.id.as_str()).collect();

    let new_count = remote_entries
        .iter()
        .filter(|e| !local_ids.contains(&e.id))
        .count();
    let removed_count = local_ids.iter().filter(|id| !remote_ids.contains(id.as_str())).count();

    let local_done: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tracks WHERE status = 'done'")
        .fetch_one(&pool)
        .await?;

    println!("Tracks");
    println!("  {local_done} local");
    println!("  {} remote", remote_entries.len());
    println!();
    println!("Changes:");
    println!();
    println!("  + {new_count} new");
    println!("  - {removed_count} removed");
    println!();

    if new_count > 0 || removed_count > 0 {
        println!("Use 'yit sync' to update.");
    } else {
        println!("Up to date.");
    }

    Ok(())
}
