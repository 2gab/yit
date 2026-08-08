use anyhow::Result;

use crate::db;

pub async fn run(port: u16) -> Result<()> {
    let dir = std::env::current_dir()?;
    db::require_existing(&dir)?;
    let pool = db::connect(&dir).await?;

    let title: String = sqlx::query_scalar("SELECT title FROM playlist WHERE id = 1")
        .fetch_one(&pool)
        .await?;

    crate::web::run(pool, port, title).await
}
