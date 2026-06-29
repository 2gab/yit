import type { FastifyReply, FastifyRequest } from "fastify";
import type { z } from "zod";
import type { loginSchema } from "./auth.schema.js";
import { loginService, getMeService } from "./auth.service.js";

type LoginBody = z.infer<typeof loginSchema.body>;

export async function getMeController(
  request: FastifyRequest,
  reply: FastifyReply
) {
  const user = await getMeService(request.user.id);

  if (!user) {
    return reply.status(404).send({ message: "User not found." });
  }

  return reply.send(user);
}

export async function loginController(
  request: FastifyRequest<{ Body: LoginBody }>,
  reply: FastifyReply
) {
  const user = await loginService(request.body);

  if (!user) {
    return reply.status(401).send({ message: "Invalid email or password." });
  }

  const token = await reply.jwtSign({ id: user.id, email: user.email });

  return reply.send({ token });
}
