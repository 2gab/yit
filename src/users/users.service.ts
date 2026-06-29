import type { CreateUserRequest } from "./users.types.js";

type CreateUserBody = CreateUserRequest["Body"];

export async function createUserService(data: CreateUserBody) {
    return data;
}