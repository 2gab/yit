import type { FastifyInstance } from "fastify";
import type { ZodTypeProvider } from "@fastify/type-provider-zod";
import { loginController, getMeController } from "./auth.controller.js";
import { loginSchema } from "./auth.schema.js";

export async function authRoutes(app: FastifyInstance) {
  const route = app.withTypeProvider<ZodTypeProvider>();

  route.post("/login", { schema: loginSchema }, loginController);

  route.get("/me", {
    preHandler: [app.authenticate],
  }, getMeController);
}
