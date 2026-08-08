# yit

git for YouTube.

Each playlist lives in its own directory, tracked by a local `.yit/yithub.db`
— no login, no global state. Copy the folder, `tar` it, put it in its own
git repo, whatever: it's portable.

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
yit status        show a summary of local vs remote playlist state
yit diff          show detailed changes between local and remote playlist state
yit sync          fetch remote changes and download new tracks
yit pull          alias for sync
yit untrack       stop tracking (removes .yit/, keeps downloaded files)
yit serve         serve the playlist over HTTP on your local network
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

## serving a playlist

```bash
yit serve --port 8080
```

prints a LAN URL (`http://192.168.x.x:8080`) that any device on the same
network can open to browse and play the tracks. It binds locally with no
authentication — treat it as trusted-network-only for now; public exposure
is a deliberate, separate step, not a default.

`yit` does not host, catalog, or distribute media itself — it only serves
files that already exist on your machine, to whoever you choose to give
that address to. You're responsible for what you download and make
available through your own `yit serve`.
