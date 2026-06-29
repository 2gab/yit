import Fastify from "fastify";
import { serializerCompiler, validatorCompiler } from "@fastify/type-provider-zod";
import fastifyJwt from "@fastify/jwt";
import { usersRoutes } from "./users/users.routes.js";
import { authRoutes } from "./auth/auth.routes.js";

const app = Fastify();

app.setValidatorCompiler(validatorCompiler);
app.setSerializerCompiler(serializerCompiler);

app.register(fastifyJwt, {
  secret: process.env["JWT_SECRET"] ?? "dev-secret",
});

app.setErrorHandler((error, _request, reply) => {
  if (error.code === "P2002") {
    return reply.status(409).send({ message: "Email already exists." });
  }

  return reply.send(error);
});

app.register(usersRoutes);
app.register(authRoutes);

app.get("/", async () => {
  return {
    message: "Api",
  };
});

export default app;
