import type { FastifyReply, FastifyRequest } from "fastify";
import { pullService } from "./pull.service.js";

export async function pullController(
  request: FastifyRequest,
  reply: FastifyReply
) {
  const result = await pullService(request.user.id);
  return reply.send(result);
}
