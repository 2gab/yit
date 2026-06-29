import type { z } from "zod";
import type { loginSchema } from "./auth.schema.js";
import { prisma } from "../lib/prisma.js";
import * as argon2 from "argon2";

type LoginBody = z.infer<typeof loginSchema.body>;

export async function getMeService(id: number) {
  const user = await prisma.user.findUnique({
    where: { id },
    select: { id: true, email: true, name: true, createdAt: true },
  });

  return user;
}

export async function loginService(data: LoginBody) {
  const user = await prisma.user.findUnique({
    where: { email: data.email },
  });

  if (!user) {
    return null;
  }

  const validPassword = await argon2.verify(user.password, data.password);

  if (!validPassword) {
    return null;
  }

  return { id: user.id, email: user.email, name: user.name };
}
