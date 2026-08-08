CREATE TABLE IF NOT EXISTS playlist (
    id          INTEGER PRIMARY KEY CHECK (id = 1),
    youtube_id  TEXT NOT NULL,
    title       TEXT NOT NULL,
    thumbnail   TEXT,
    remote_url  TEXT NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS tracks (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    youtube_id  TEXT NOT NULL UNIQUE,
    title       TEXT NOT NULL,
    uploader    TEXT,
    position    INTEGER NOT NULL,
    thumbnail   TEXT,
    filename    TEXT,
    status      TEXT NOT NULL DEFAULT 'pending',
    error       TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);
