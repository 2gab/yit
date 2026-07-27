import type { FastifyInstance } from "fastify";
import type { ZodTypeProvider } from "@fastify/type-provider-zod";
import { addPlaylistController, listPlaylistsController } from "./playlists.controller.js";
import { addPlaylistSchema } from "./playlists.schema.js";

export async function playlistsRoutes(app: FastifyInstance) {
  const route = app.withTypeProvider<ZodTypeProvider>();

  route.addHook("preHandler", app.authenticate);

  route.get("/playlists", listPlaylistsController);
  route.post("/playlists", { schema: addPlaylistSchema }, addPlaylistController);
}
