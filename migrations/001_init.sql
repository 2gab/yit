CREATE TABLE IF NOT EXISTS playlists (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    youtube_id  TEXT NOT NULL UNIQUE,
    title       TEXT NOT NULL,
    description TEXT,
    thumbnail   TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS tracks (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    youtube_id  TEXT NOT NULL UNIQUE,
    title       TEXT NOT NULL,
    artist      TEXT,
    duration    INTEGER,
    thumbnail   TEXT,
    position    INTEGER NOT NULL,
    playlist_id INTEGER NOT NULL REFERENCES playlists(id),
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS downloads (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    track_id   INTEGER NOT NULL UNIQUE REFERENCES tracks(id),
    status     TEXT NOT NULL DEFAULT 'pending',
    path       TEXT,
    format     TEXT,
    error      TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
