import { prisma } from "../lib/prisma.js";

export async function getMeService(id: number) {
  return prisma.user.findUnique({
    where: { id },
    select: { id: true, email: true, name: true, createdAt: true },
  });
}

type GoogleProfile = {
  sub: string;
  email: string;
  name: string;
};

type GoogleTokens = {
  access_token: string;
  refresh_token?: string;
};

export async function googleAuthService(profile: GoogleProfile, tokens: GoogleTokens) {
  const user = await prisma.user.upsert({
    where: { googleId: profile.sub },
    update: {
      accessToken: tokens.access_token,
      refreshToken: tokens.refresh_token ?? null,
    },
    create: {
      email: profile.email,
      name: profile.name,
      googleId: profile.sub,
      accessToken: tokens.access_token,
      refreshToken: tokens.refresh_token ?? null,
    },
  });

  return { id: user.id, email: user.email };
}
