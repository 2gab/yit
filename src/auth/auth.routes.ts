import type { FastifyInstance } from "fastify";
import { getMeController, googleCallbackController } from "./auth.controller.js";

export async function authRoutes(app: FastifyInstance) {
  app.get("/me", { preHandler: [app.authenticate] }, getMeController);
  app.get("/auth/google/callback", googleCallbackController);
}
