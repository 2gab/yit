import { execFile } from "node:child_process";
import { mkdir } from "node:fs/promises";
import { homedir } from "node:os";
import { join } from "node:path";
import { promisify } from "node:util";
import { prisma } from "../lib/prisma.js";

const execFileAsync = promisify(execFile);

function sanitizeFilename(name: string): string {
  return name.replace(/[/\\?%*:|"<>]/g, "-").trim();
}

async function downloadTrack(
  videoId: string,
  title: string,
  playlistTitle: string,
  format: string
): Promise<string> {
  const baseDir = join(homedir(), "Music", "yit", sanitizeFilename(playlistTitle));
  await mkdir(baseDir, { recursive: true });

  const outputPath = join(baseDir, `${sanitizeFilename(title)}.%(ext)s`);
  const url = `https://www.youtube.com/watch?v=${videoId}`;

  await execFileAsync("yt-dlp", [
    "--extract-audio",
    "--audio-format", format,
    "--audio-quality", "0",
    "--output", outputPath,
    "--no-playlist",
    url,
  ]);

  return join(baseDir, `${sanitizeFilename(title)}.${format}`);
}

export async function pullService(userId: number, format = "opus") {
  const pendingTracks = await prisma.track.findMany({
    where: {
      playlist: { userId },
      OR: [
        { download: null },
        { download: { status: "error" } },
      ],
    },
    include: { playlist: true },
  });

  if (pendingTracks.length === 0) {
    return { downloaded: 0, failed: 0, message: "Nothing to download." };
  }

  let downloaded = 0;
  let failed = 0;

  for (const track of pendingTracks) {
    const download = await prisma.download.upsert({
      where: { trackId: track.id },
      update: { status: "downloading", error: null, format },
      create: { trackId: track.id, status: "downloading", format },
    });

    try {
      const path = await downloadTrack(
        track.youtubeId,
        track.title,
        track.playlist.title,
        format
      );

      await prisma.download.update({
        where: { id: download.id },
        data: { status: "done", path },
      });

      downloaded++;
    } catch (err) {
      await prisma.download.update({
        where: { id: download.id },
        data: {
          status: "error",
          error: err instanceof Error ? err.message : String(err),
        },
      });

      failed++;
    }
  }

  return {
    downloaded,
    failed,
    message: `${downloaded} downloaded, ${failed} failed.`,
  };
}
