use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use sqlx::SqlitePool;
use tower::ServiceExt;
use tower_http::services::ServeFile;

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

pub async fn run(pool: SqlitePool, port: u16, title: String) -> anyhow::Result<()> {
    let state = AppState { pool };

    let app = Router::new()
        .route("/", get(index))
        .route("/playlist", get(get_playlist))
        .route("/tracks", get(get_tracks))
        .route("/tracks/{id}", get(get_track))
        .route("/tracks/{id}/audio", get(get_track_audio))
        .route("/artwork", get(get_artwork))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;

    let lan_ip = local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "0.0.0.0".to_string());

    println!("yit server started");
    println!();
    println!("Playlist: {title}");
    println!("Listening on:");
    println!("  http://{lan_ip}:{port}");
    println!("  http://localhost:{port}");
    println!();
    println!("Press Ctrl+C to stop.");

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Serialize)]
struct PlaylistResponse {
    title: String,
    youtube_id: String,
    remote_url: String,
    track_count: i64,
}

async fn get_playlist(State(state): State<AppState>) -> Result<Json<PlaylistResponse>, StatusCode> {
    let (title, youtube_id, remote_url): (String, String, String) =
        sqlx::query_as("SELECT title, youtube_id, remote_url FROM playlist WHERE id = 1")
            .fetch_one(&state.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let track_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tracks WHERE status = 'done'")
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PlaylistResponse {
        title,
        youtube_id,
        remote_url,
        track_count,
    }))
}

#[derive(Serialize, sqlx::FromRow)]
struct TrackResponse {
    youtube_id: String,
    position: i64,
    title: String,
    uploader: Option<String>,
}

const TRACK_FIELDS: &str = "youtube_id, position, title, uploader";

async fn get_tracks(State(state): State<AppState>) -> Result<Json<Vec<TrackResponse>>, StatusCode> {
    let tracks = sqlx::query_as::<_, TrackResponse>(&format!(
        "SELECT {TRACK_FIELDS} FROM tracks WHERE status = 'done' ORDER BY position"
    ))
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tracks))
}

async fn get_track(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TrackResponse>, StatusCode> {
    sqlx::query_as::<_, TrackResponse>(&format!(
        "SELECT {TRACK_FIELDS} FROM tracks WHERE youtube_id = ? AND status = 'done'"
    ))
    .bind(&id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map(Json)
    .ok_or(StatusCode::NOT_FOUND)
}

async fn get_track_audio(
    State(state): State<AppState>,
    Path(id): Path<String>,
    request: Request,
) -> Response {
    let filename: Option<String> = sqlx::query_scalar(
        "SELECT filename FROM tracks WHERE youtube_id = ? AND status = 'done'",
    )
    .bind(&id)
    .fetch_optional(&state.pool)
    .await
    .ok()
    .flatten();

    let Some(filename) = filename else {
        return StatusCode::NOT_FOUND.into_response();
    };

    match ServeFile::new(filename).oneshot(request).await {
        Ok(res) => res.into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn get_artwork(State(state): State<AppState>) -> Response {
    let thumbnail: Option<String> =
        sqlx::query_scalar("SELECT thumbnail FROM playlist WHERE id = 1")
            .fetch_one(&state.pool)
            .await
            .ok()
            .flatten();

    match thumbnail {
        Some(url) => Redirect::temporary(&url).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn index(State(state): State<AppState>) -> Html<String> {
    let title: String = sqlx::query_scalar("SELECT title FROM playlist WHERE id = 1")
        .fetch_one(&state.pool)
        .await
        .unwrap_or_else(|_| "yit".to_string());

    let tracks = sqlx::query_as::<_, (String, i64, String)>(
        "SELECT youtube_id, position, title FROM tracks WHERE status = 'done' ORDER BY position",
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    let mut rows = String::new();
    for (id, position, track_title) in &tracks {
        rows.push_str(&format!(
            "<li><span class=\"n\">{:02}</span><span class=\"t\">{}</span><audio controls preload=\"none\" src=\"/tracks/{}/audio\"></audio></li>\n",
            position + 1,
            html_escape(track_title),
            id,
        ));
    }

    let title = html_escape(&title);

    Html(format!(
        r#"<!doctype html>
<html>
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>
  body {{ font-family: system-ui, sans-serif; background: #111; color: #eee; margin: 0; padding: 2rem 1rem; }}
  h1 {{ font-size: 1.4rem; }}
  ul {{ list-style: none; padding: 0; margin: 1.5rem 0 0; }}
  li {{ display: flex; align-items: center; gap: 0.75rem; padding: 0.6rem 0; border-bottom: 1px solid #333; flex-wrap: wrap; }}
  .n {{ color: #888; width: 2ch; text-align: right; }}
  .t {{ flex: 1; min-width: 10rem; }}
  audio {{ height: 32px; }}
</style>
</head>
<body>
<h1>{title}</h1>
<ul>
{rows}</ul>
</body>
</html>"#
    ))
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
