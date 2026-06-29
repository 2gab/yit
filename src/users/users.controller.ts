import type { FastifyReply, FastifyRequest } from "fastify";
import type { ZodTypeProvider } from "@fastify/type-provider-zod";
import type { createUserSchema } from "./users.schema.js";
import type { z } from "zod";
import { createUserService } from "./users.service.js";

type CreateUserBody = z.infer<typeof createUserSchema.body>;

export async function createUserController(
  request: FastifyRequest<{ Body: CreateUserBody }>,
  reply: FastifyReply
) {
  const user = await createUserService(request.body);

  return reply.send(user);
}
