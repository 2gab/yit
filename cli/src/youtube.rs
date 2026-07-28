use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlaylistSnippet {
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
}

#[derive(Debug, Deserialize)]
pub struct Thumbnails {
    pub default: Option<Thumbnail>,
}

#[derive(Debug, Deserialize)]
pub struct Thumbnail {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistItem {
    pub id: String,
    pub snippet: PlaylistSnippet,
}

#[derive(Debug, Deserialize)]
struct PlaylistResponse {
    items: Option<Vec<PlaylistItem>>,
}

pub async fn fetch_playlist(playlist_id: &str, access_token: &str) -> Result<PlaylistItem> {
    let client = reqwest::Client::new();

    let response: PlaylistResponse = client
        .get("https://www.googleapis.com/youtube/v3/playlists")
        .query(&[("part", "snippet"), ("id", playlist_id)])
        .bearer_auth(access_token)
        .send()
        .await?
        .json()
        .await?;

    response
        .items
        .and_then(|mut items| if items.is_empty() { None } else { Some(items.remove(0)) })
        .with_context(|| format!("Playlist '{playlist_id}' not found on YouTube"))
}

pub fn extract_playlist_id(input: &str) -> Option<String> {
    if let Ok(url) = url::Url::parse(input) {
        if let Some(list) = url.query_pairs().find(|(k, _)| k == "list") {
            return Some(list.1.to_string());
        }
    }

    if input.starts_with("PL") && input.len() > 10 {
        return Some(input.to_string());
    }

    None
}
