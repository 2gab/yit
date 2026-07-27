import type { FastifyReply, FastifyRequest } from "fastify";
import { statusService } from "./status.service.js";

export async function statusController(
  request: FastifyRequest,
  reply: FastifyReply
) {
  const status = await statusService(request.user.id);
  return reply.send(status);
}
