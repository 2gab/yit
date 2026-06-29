import * as z from "zod";

export const createUserSchema = {
  body: z.object({
    email: z.string().email(),
    name: z.string().optional(),
    password: z.string().min(6),
  }),
};
