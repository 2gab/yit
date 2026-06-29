import type { z } from "zod";
import type { createUserSchema } from "./users.schema.js";
import { prisma } from "../lib/prisma.js";
import * as argon2 from "argon2";

type CreateUserBody = z.infer<typeof createUserSchema.body>;

export async function createUserService(data: CreateUserBody) {
  const hashedPassword = await argon2.hash(data.password);

  const { password: _, ...user } = await prisma.user.create({
    data: {
      email: data.email,
      password: hashedPassword,
      name: data.name ?? null,
    },
  });

  return user;
}
