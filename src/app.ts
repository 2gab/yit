import Fastify from "fastify";
import { serializerCompiler, validatorCompiler } from "@fastify/type-provider-zod";
import { usersRoutes } from "./users/users.routes.js";

const app = Fastify();

app.setValidatorCompiler(validatorCompiler);
app.setSerializerCompiler(serializerCompiler);

app.register(usersRoutes);

app.get("/", async () => {
  return {
    message: "Api",
  };
});

export default app;
