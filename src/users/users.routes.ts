import { FastifyInstance } from "fastify";
import { createUserController } from "./users.controller";

export async function usersRoutes(app: FastifyInstance) {
  app.post("/users", createUserController);
}
