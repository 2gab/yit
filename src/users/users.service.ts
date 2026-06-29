import type { CreateUserRequest } from "./users.types.js";
import { prisma } from "../lib/prisma.js";

type CreateUserBody = CreateUserRequest["Body"];

export async function createUserService(data: CreateUserBody) {
    const user = await prisma.user.create({
        data,
    });

    return user;
}