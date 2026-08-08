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
```

## usage

```
yit add <url>   track a playlist
yit ls          list tracked playlists
yit fetch       sync track list from YouTube
yit status      show download status
yit pull        download pending tracks
```

## workflow

```bash
yit add "https://youtube.com/playlist?list=PLxxxxxxx"
yit fetch
yit pull
```
