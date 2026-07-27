import type { FastifyInstance } from "fastify";
import { fetchController } from "./fetch.controller.js";

export async function fetchRoutes(app: FastifyInstance) {
  app.addHook("preHandler", app.authenticate);

  app.post("/fetch", fetchController);
}
