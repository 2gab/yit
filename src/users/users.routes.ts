import type { FastifyInstance } from "fastify";
import { createUserController } from "./users.controller.js";

export async function usersRoutes(app: FastifyInstance) {
  app.post("/users", createUserController);
}
