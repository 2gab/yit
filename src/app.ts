import Fastify from "fastify";
import { serializerCompiler, validatorCompiler } from "@fastify/type-provider-zod";
import fastifyJwt from "@fastify/jwt";
import fastifyOauth2 from "@fastify/oauth2";
import { authRoutes } from "./auth/auth.routes.js";
import { playlistsRoutes } from "./playlists/playlists.routes.js";
import { fetchRoutes } from "./fetch/fetch.routes.js";
import { pullRoutes } from "./pull/pull.routes.js";

const app = Fastify();

app.setValidatorCompiler(validatorCompiler);
app.setSerializerCompiler(serializerCompiler);

app.register(fastifyJwt, {
  secret: process.env["JWT_SECRET"] ?? "dev-secret",
});

app.register(fastifyOauth2, {
  name: "googleOAuth2",
  scope: [
    "openid",
    "email",
    "profile",
    "https://www.googleapis.com/auth/youtube.readonly",
  ],
  credentials: {
    client: {
      id: process.env["GOOGLE_CLIENT_ID"] ?? "",
      secret: process.env["GOOGLE_CLIENT_SECRET"] ?? "",
    },
    auth: fastifyOauth2.GOOGLE_CONFIGURATION,
  },
  startRedirectPath: "/auth/google",
  callbackUri: process.env["GOOGLE_CALLBACK_URL"] ?? "http://localhost:3333/auth/google/callback",
});

app.decorate("authenticate", async (request, reply) => {
  try {
    await request.jwtVerify();
  } catch {
    return reply.status(401).send({ message: "Unauthorized." });
  }
});

app.setErrorHandler((error, _request, reply) => {
  return reply.send(error);
});

app.register(authRoutes);
app.register(playlistsRoutes);
app.register(fetchRoutes);
app.register(pullRoutes);

app.get("/", async () => {
  return { message: "yit api" };
});

export default app;
