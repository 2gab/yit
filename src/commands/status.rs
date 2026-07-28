use anyhow::Result;
use sqlx::SqlitePool;

pub async fn run(pool: &SqlitePool) -> Result<()> {
    let playlists = sqlx::query_as::<_, (String, String, i64, i64, i64, i64)>(
        "SELECT
             p.youtube_id,
             p.title,
             COUNT(t.id)                                               AS total,
             SUM(CASE WHEN d.status = 'done'    THEN 1 ELSE 0 END)    AS done,
             SUM(CASE WHEN d.status = 'pending' OR d.id IS NULL
                           THEN 1 ELSE 0 END)                          AS pending,
             SUM(CASE WHEN d.status = 'error'   THEN 1 ELSE 0 END)    AS errors
         FROM playlists p
         LEFT JOIN tracks t    ON t.playlist_id = p.id
         LEFT JOIN downloads d ON d.track_id    = t.id
         GROUP BY p.id
         ORDER BY p.created_at",
    )
    .fetch_all(pool)
    .await?;

    if playlists.is_empty() {
        println!("No playlists tracked. Use `yit add <url>` to add one.");
        return Ok(());
    }

    let mut total_tracks = 0i64;
    let mut total_done = 0i64;
    let mut total_pending = 0i64;
    let mut total_errors = 0i64;

    for (yt_id, title, total, done, pending, errors) in &playlists {
        println!("{yt_id}  {title}");
        println!("  tracks: {total}  done: {done}  pending: {pending}  errors: {errors}");

        total_tracks += total;
        total_done += done;
        total_pending += pending;
        total_errors += errors;
    }

    if playlists.len() > 1 {
        println!();
        println!(
            "total: {total_tracks} tracks — {total_done} done, {total_pending} pending, {total_errors} errors"
        );
    }

    Ok(())
}
