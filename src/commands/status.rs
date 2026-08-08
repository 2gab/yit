use anyhow::Result;

use crate::{db, playlist, youtube};

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
    let diff = playlist::diff_remote(&pool, &info).await?;

    let local_done: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tracks WHERE status = 'done'")
        .fetch_one(&pool)
        .await?;
    let remote_total = info
        .entries
        .iter()
        .filter(|e| e.title != "[Deleted video]" && e.title != "[Private video]")
        .count();

    println!("Tracks");
    println!("  {local_done} local");
    println!("  {remote_total} remote");
    println!();
    println!("Changes:");
    println!();
    println!("  + {} new", diff.new.len());
    println!("  - {} removed", diff.removed.len());
    println!();

    if diff.new.is_empty() && diff.removed.is_empty() {
        println!("Up to date.");
    } else {
        println!("Use 'yit diff' to see details, 'yit sync' to update.");
    }

    Ok(())
}
