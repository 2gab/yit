import { FastifyReply, FastifyRequest } from "fastify";
import { CreateUserRequest } from "./users.types";

export async function createUser(
  request: FastifyRequest<CreateUserRequest>,
  reply: FastifyReply
) {
  const { email, name, password } = request.body;

  return reply.send({
    email,
    name,
    password,
  });
}