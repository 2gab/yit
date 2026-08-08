use anyhow::{Context, Result};
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct Thumbnail {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistEntry {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub uploader: Option<String>,
    #[serde(default)]
    pub thumbnails: Option<Vec<Thumbnail>>,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistInfo {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub thumbnails: Option<Vec<Thumbnail>>,
    #[serde(default)]
    pub entries: Vec<PlaylistEntry>,
}

pub fn thumbnail_url(thumbnails: &Option<Vec<Thumbnail>>) -> Option<String> {
    thumbnails.as_ref().and_then(|t| t.last()).map(|t| t.url.clone())
}

pub fn fetch_playlist(playlist_id: &str) -> Result<PlaylistInfo> {
    let url = format!("https://www.youtube.com/playlist?list={playlist_id}");

    let output = Command::new("yt-dlp")
        .args(["-J", "--flat-playlist", "--no-warnings", &url])
        .output()
        .context("yt-dlp not found. Install it with: pip install yt-dlp")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "yt-dlp failed to fetch playlist '{playlist_id}': {}",
            stderr.lines().last().unwrap_or("unknown error")
        );
    }

    serde_json::from_slice(&output.stdout)
        .with_context(|| format!("Failed to parse yt-dlp output for playlist '{playlist_id}'"))
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
