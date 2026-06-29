import { FastifyReply, FastifyRequest } from "fastify";
import { CreateUserRequest } from "./users.types";
import { createUserService } from "./users.service";

export async function createUserController(
  request: FastifyRequest<CreateUserRequest>,
  reply: FastifyReply
) {
  const user = await createUserService(request.body);

  return reply.send(user);
}