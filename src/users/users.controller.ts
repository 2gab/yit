import type { FastifyReply, FastifyRequest } from "fastify";
import type { CreateUserRequest } from "./users.types.js";
import { createUserService } from "./users.service.js";

export async function createUserController(
  request: FastifyRequest<CreateUserRequest>,
  reply: FastifyReply
) {
  const user = await createUserService(request.body);

  return reply.send(user);
}