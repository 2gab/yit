import type { FastifyReply, FastifyRequest } from "fastify";
import type { z } from "zod";
import type { addPlaylistSchema } from "./playlists.schema.js";
import { addPlaylistService, listPlaylistsService } from "./playlists.service.js";

type AddPlaylistBody = z.infer<typeof addPlaylistSchema.body>;

export async function addPlaylistController(
  request: FastifyRequest<{ Body: AddPlaylistBody }>,
  reply: FastifyReply
) {
  const result = await addPlaylistService(request.user.id, request.body.url);

  if ("error" in result) {
    return reply.status(400).send({ message: result.error });
  }

  return reply.status(201).send(result.playlist);
}

export async function listPlaylistsController(
  request: FastifyRequest,
  reply: FastifyReply
) {
  const playlists = await listPlaylistsService(request.user.id);
  return reply.send(playlists);
}
