import { prisma } from "../lib/prisma.js";

export async function statusService(userId: number) {
  const playlists = await prisma.playlist.findMany({
    where: { userId },
    include: {
      tracks: {
        include: { download: true },
      },
    },
  });

  const summary = playlists.map((playlist) => {
    const total = playlist.tracks.length;
    const done = playlist.tracks.filter((t) => t.download?.status === "done").length;
    const failed = playlist.tracks.filter((t) => t.download?.status === "error").length;
    const downloading = playlist.tracks.filter((t) => t.download?.status === "downloading").length;
    const pending = playlist.tracks.filter((t) => !t.download).length;

    return {
      playlist: playlist.title,
      youtubeId: playlist.youtubeId,
      total,
      done,
      pending,
      downloading,
      failed,
    };
  });

  const totals = summary.reduce(
    (acc, p) => ({
      total: acc.total + p.total,
      done: acc.done + p.done,
      pending: acc.pending + p.pending,
      downloading: acc.downloading + p.downloading,
      failed: acc.failed + p.failed,
    }),
    { total: 0, done: 0, pending: 0, downloading: 0, failed: 0 }
  );

  return { totals, playlists: summary };
}
