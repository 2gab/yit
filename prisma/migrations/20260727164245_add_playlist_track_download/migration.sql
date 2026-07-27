-- CreateTable
CREATE TABLE "Playlist" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "youtubeId" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT,
    "thumbnail" TEXT,
    "userId" INTEGER NOT NULL,
    "createdAt" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" DATETIME NOT NULL,
    CONSTRAINT "Playlist_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "Track" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "youtubeId" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "artist" TEXT,
    "duration" INTEGER,
    "thumbnail" TEXT,
    "position" INTEGER NOT NULL,
    "playlistId" INTEGER NOT NULL,
    "createdAt" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" DATETIME NOT NULL,
    CONSTRAINT "Track_playlistId_fkey" FOREIGN KEY ("playlistId") REFERENCES "Playlist" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "Download" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "status" TEXT NOT NULL DEFAULT 'pending',
    "path" TEXT,
    "format" TEXT,
    "error" TEXT,
    "trackId" INTEGER NOT NULL,
    "createdAt" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" DATETIME NOT NULL,
    CONSTRAINT "Download_trackId_fkey" FOREIGN KEY ("trackId") REFERENCES "Track" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX "Playlist_youtubeId_userId_key" ON "Playlist"("youtubeId", "userId");

-- CreateIndex
CREATE UNIQUE INDEX "Track_youtubeId_key" ON "Track"("youtubeId");

-- CreateIndex
CREATE UNIQUE INDEX "Download_trackId_key" ON "Download"("trackId");
