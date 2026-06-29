import { CreateUserRequest } from "./users.types";

type CreateUserBody = CreateUserRequest["Body"];

export async function createUserService(data: CreateUserBody) {
    return data;
}