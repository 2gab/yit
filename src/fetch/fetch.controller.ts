import type { FastifyReply, FastifyRequest } from "fastify";
import { fetchService } from "./fetch.service.js";

export async function fetchController(
  request: FastifyRequest,
  reply: FastifyReply
) {
  const result = await fetchService(request.user.id);

  if ("error" in result) {
    return reply.status(400).send({ message: result.error });
  }

  return reply.send(result);
}
