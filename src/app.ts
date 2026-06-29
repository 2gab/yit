import Fastify from "fastify";

const app = Fastify();

app.get("/", async () => {
  return {
    message: "Api"
  };
});

export default app;
