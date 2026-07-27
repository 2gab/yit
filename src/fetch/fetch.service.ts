import { prisma } from "../lib/prisma.js";

type PlaylistItemsResponse = {
  nextPageToken?: string;
  items: Array<{
    snippet: {
      title: string;
      position: number;
      resourceId: { videoId: string };
      videoOwnerChannelTitle?: string;
      thumbnails: { default?: { url: string } };
    };
  }>;
};

async function fetchPlaylistItems(
  playlistId: string,
  accessToken: string
): Promise<PlaylistItemsResponse["items"]> {
  const items: PlaylistItemsResponse["items"] = [];
  let pageToken: string | undefined;

  do {
    const params = new URLSearchParams({
      part: "snippet",
      playlistId,
      maxResults: "50",
      ...(pageToken ? { pageToken } : {}),
    });

    const response = await fetch(
      `https://www.googleapis.com/youtube/v3/playlistItems?${params}`,
      { headers: { Authorization: `Bearer ${accessToken}` } }
    );

    const data = await response.json() as PlaylistItemsResponse;
    items.push(...data.items);
    pageToken = data.nextPageToken;
  } while (pageToken);

  return items;
}

export async function fetchService(userId: number) {
  const user = await prisma.user.findUnique({ where: { id: userId } });

  if (!user?.accessToken) {
    return { error: "No YouTube access token found. Please login again." };
  }

  const playlists = await prisma.playlist.findMany({ where: { userId } });

  if (playlists.length === 0) {
    return { synced: 0, message: "No playlists tracked. Use POST /playlists to add one." };
  }

  let totalNew = 0;

  for (const playlist of playlists) {
    const items = await fetchPlaylistItems(playlist.youtubeId, user.accessToken);

    for (const item of items) {
      const { videoId } = item.snippet.resourceId;

      if (!videoId) continue;

      const existing = await prisma.track.findUnique({ where: { youtubeId: videoId } });

      if (!existing) {
        await prisma.track.create({
          data: {
            youtubeId: videoId,
            title: item.snippet.title,
            artist: item.snippet.videoOwnerChannelTitle ?? null,
            thumbnail: item.snippet.thumbnails.default?.url ?? null,
            position: item.snippet.position,
            playlistId: playlist.id,
          },
        });
        totalNew++;
      }
    }
  }

  return { synced: totalNew, message: `${totalNew} new track(s) synced.` };
}
