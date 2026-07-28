# yit

git for YouTube.

## install

```
cargo install --path .
```

requires [yt-dlp](https://github.com/yt-dlp/yt-dlp) for downloads.

## config

on first run, a config file is created at `~/.config/yit/config.toml`:

```toml
download_path = "~/Music/yit"
format = "opus"

[google]
client_id = ""
client_secret = ""
```

fill in your [Google OAuth credentials](https://console.cloud.google.com/) before continuing.  
scopes needed: `youtube.readonly`

## usage

```
yit login       authenticate with Google
yit add <url>   track a playlist
yit ls          list tracked playlists
yit fetch       sync track list from YouTube
yit status      show download status
yit pull        download pending tracks
```

## workflow

```bash
yit login
yit add "https://youtube.com/playlist?list=PLxxxxxxx"
yit fetch
yit pull
```
