# yit

git for YouTube.

Each playlist lives in its own directory, tracked by a local `yithub.db` —
no login, no global state. Copy the folder, `tar` it, put it in its own git
repo, whatever: it's portable.

## install

```
cargo install --path .
```

requires [yt-dlp](https://github.com/yt-dlp/yt-dlp) for downloads.

## config

on first run, a config file is created at `~/.config/yit/config.toml`:

```toml
format = "opus"
```

## usage

```
yit clone <url>   clone a remote playlist into a new directory
yit init <url>    turn the current directory into a tracked playlist
yit status        show diff between local and remote playlist state
yit sync          fetch remote changes and download new tracks
yit pull          alias for sync
yit untrack       stop tracking (removes yithub.db, keeps downloaded files)
```

## workflow

```bash
yit clone "https://youtube.com/playlist?list=PLxxxxxxx"
cd PLxxxxxxx
yit status
yit sync
```

or, starting from an existing directory:

```bash
mkdir my-playlist && cd my-playlist
yit init "https://youtube.com/playlist?list=PLxxxxxxx"
yit sync
```
