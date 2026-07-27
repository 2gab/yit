import { prisma } from "../lib/prisma.js";

function extractPlaylistId(input: string): string | null {
  try {
    const url = new URL(input);
    const listParam = url.searchParams.get("list");
    if (listParam) return listParam;
  } catch {
    // not a URL — treat as raw ID
  }

  if (/^PL[\w-]{10,}$/.test(input)) return input;

  return null;
}

type YoutubePlaylistSnippet = {
  items: Array<{
    id: string;
    snippet: {
      title: string;
      description: string;
      thumbnails: { default?: { url: string } };
    };
  }>;
};

async function fetchPlaylistFromYoutube(playlistId: string, accessToken: string) {
  const params = new URLSearchParams({
    part: "snippet",
    id: playlistId,
  });

  const response = await fetch(
    `https://www.googleapis.com/youtube/v3/playlists?${params}`,
    { headers: { Authorization: `Bearer ${accessToken}` } }
  );

  const data = await response.json() as YoutubePlaylistSnippet;
  return data.items[0] ?? null;
}

export async function addPlaylistService(userId: number, url: string) {
  const playlistId = extractPlaylistId(url);

  if (!playlistId) {
    return { error: "Invalid YouTube playlist URL or ID." };
  }

  const user = await prisma.user.findUnique({ where: { id: userId } });

  if (!user?.accessToken) {
    return { error: "No YouTube access token found. Please login again." };
  }

  const ytPlaylist = await fetchPlaylistFromYoutube(playlistId, user.accessToken);

  if (!ytPlaylist) {
    return { error: "Playlist not found on YouTube." };
  }

  const playlist = await prisma.playlist.create({
    data: {
      youtubeId: ytPlaylist.id,
      title: ytPlaylist.snippet.title,
      description: ytPlaylist.snippet.description,
      thumbnail: ytPlaylist.snippet.thumbnails.default?.url ?? null,
      userId,
    },
  });

  return { playlist };
}

export async function listPlaylistsService(userId: number) {
  return prisma.playlist.findMany({
    where: { userId },
    include: { _count: { select: { tracks: true } } },
    orderBy: { createdAt: "desc" },
  });
}
