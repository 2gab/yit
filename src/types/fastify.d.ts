import "fastify";
import type { FastifyRequest, FastifyReply } from "fastify";
import type { OAuth2Namespace } from "@fastify/oauth2";

declare module "fastify" {
  interface FastifyInstance {
    authenticate(request: FastifyRequest, reply: FastifyReply): Promise<void>;
    googleOAuth2: OAuth2Namespace;
  }
}
