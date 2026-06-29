import { FastifyInstance } from "fastify";
import { createUser } from "./users.controller";

export async function usersRoutes(app: FastifyInstance) {
  app.post("/users", createUser);
}
