import type { z } from "zod";
import type { createUserSchema } from "./users.schema.js";
import { prisma } from "../lib/prisma.js";

type CreateUserBody = z.infer<typeof createUserSchema.body>;

export async function createUserService(data: CreateUserBody) {
    const user = await prisma.user.create({
        data: {
            email: data.email,
            password: data.password,
            name: data.name ?? null,
        },
    });

    return user;
}
