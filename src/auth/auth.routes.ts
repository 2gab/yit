import type { FastifyInstance } from "fastify";
import type { ZodTypeProvider } from "@fastify/type-provider-zod";
import { loginController } from "./auth.controller.js";
import { loginSchema } from "./auth.schema.js";

export async function authRoutes(app: FastifyInstance) {
  const route = app.withTypeProvider<ZodTypeProvider>();

  route.post("/login", { schema: loginSchema }, loginController);
}
