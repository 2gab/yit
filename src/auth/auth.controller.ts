import type { FastifyReply, FastifyRequest } from "fastify";
import { getMeService, googleAuthService } from "./auth.service.js";

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

export async function googleCallbackController(
  request: FastifyRequest,
  reply: FastifyReply
) {
  const { token } = await request.server.googleOAuth2.getAccessTokenFromAuthorizationCodeFlow(request);

  const profileResponse = await fetch("https://www.googleapis.com/oauth2/v3/userinfo", {
    headers: { Authorization: `Bearer ${token.access_token}` },
  });

  const profile = await profileResponse.json() as { sub: string; email: string; name: string };

  const user = await googleAuthService(profile, {
    access_token: token.access_token,
    refresh_token: token.refresh_token ?? undefined,
  });

  const jwt = await reply.jwtSign({ id: user.id, email: user.email });

  return reply.send({ token: jwt });
}
