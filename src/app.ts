import Fastify from "fastify";
import { usersRoutes } from "./users/users.routes.js";

const app = Fastify();

app.register(usersRoutes);

app.get("/", async () => {
  return {
    message: "Api",
  };
});

export default app;
