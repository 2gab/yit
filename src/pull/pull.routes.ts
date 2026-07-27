import type { FastifyInstance } from "fastify";
import { pullController } from "./pull.controller.js";

export async function pullRoutes(app: FastifyInstance) {
  app.addHook("preHandler", app.authenticate);

  app.post("/pull", pullController);
}
