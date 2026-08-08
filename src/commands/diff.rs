use anyhow::Result;

use crate::{db, playlist, youtube};

pub async fn run() -> Result<()> {
    let dir = std::env::current_dir()?;
    db::require_existing(&dir)?;
    let pool = db::connect(&dir).await?;

    let youtube_id: String = sqlx::query_scalar("SELECT youtube_id FROM playlist WHERE id = 1")
        .fetch_one(&pool)
        .await?;

    let info = youtube::fetch_playlist(&youtube_id)?;
    let diff = playlist::diff_remote(&pool, &info).await?;

    if diff.new.is_empty() && diff.removed.is_empty() {
        println!("No changes.");
        return Ok(());
    }

    for entry in &diff.new {
        println!("+ {:02} - {}", entry.position + 1, entry.title);
        println!("  https://www.youtube.com/watch?v={}", entry.youtube_id);
        println!();
    }

    for entry in &diff.removed {
        println!("- {:02} - {}", entry.position + 1, entry.title);
        println!("  local file preserved");
        println!();
    }

    Ok(())
}
