import type { FastifyInstance } from "fastify";
import type { ZodTypeProvider } from "@fastify/type-provider-zod";
import { createUserController } from "./users.controller.js";
import { createUserSchema } from "./users.schema.js";

export async function usersRoutes(app: FastifyInstance) {
  const route = app.withTypeProvider<ZodTypeProvider>();

  route.post("/users", { schema: createUserSchema }, createUserController);
}
