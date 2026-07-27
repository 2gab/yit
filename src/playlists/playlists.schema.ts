import * as z from "zod";

export const addPlaylistSchema = {
  body: z.object({
    url: z.string().min(1),
  }),
};
