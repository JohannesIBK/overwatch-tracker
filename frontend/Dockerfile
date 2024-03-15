FROM node:20-alpine AS deps

RUN apk add --no-cache libc6-compat

WORKDIR /app

# Install dependencies based on the preferred package manager
COPY package.json yarn.lock ./

RUN yarn install --immutable

# 2. Rebuild the source code only when needed
FROM node:20-alpine AS builder

WORKDIR /app

COPY --from=deps /app/node_modules ./node_modules

COPY . .

RUN ls

ENV NEXT_TELEMETRY_DISABLED 1

RUN yarn build

FROM gcr.io/distroless/nodejs20

WORKDIR /app

ENV NODE_ENV=production

COPY --from=builder --chown=1000 /app/.next/standalone ./
COPY --from=builder --chown=1000 /app/.next/static ./.next/static/
COPY --chown=1000 public ./public

USER 1000

CMD ["server.js"]
