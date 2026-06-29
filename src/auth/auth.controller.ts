import type { FastifyReply, FastifyRequest } from "fastify";
import type { z } from "zod";
import type { loginSchema } from "./auth.schema.js";
import { loginService } from "./auth.service.js";

type LoginBody = z.infer<typeof loginSchema.body>;

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
