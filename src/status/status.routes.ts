import type { FastifyInstance } from "fastify";
import { statusController } from "./status.controller.js";

export async function statusRoutes(app: FastifyInstance) {
  app.addHook("preHandler", app.authenticate);

  app.get("/status", statusController);
}
