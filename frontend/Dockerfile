FROM node:22-bookworm-slim AS builder

WORKDIR /app
RUN corepack enable

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY frontend/package.json frontend/package.json
RUN pnpm install --frozen-lockfile

COPY frontend frontend
ARG PUBLIC_API_URL=http://localhost:8080
ENV PUBLIC_API_URL=${PUBLIC_API_URL}
WORKDIR /app/frontend
RUN pnpm run build
RUN pnpm --filter blisseymeter deploy --prod --legacy /prod

FROM node:22-bookworm-slim

WORKDIR /app
ENV NODE_ENV=production
ENV HOST=0.0.0.0
ENV PORT=3000

COPY --from=builder /app/frontend/build ./build
COPY --from=builder /prod/node_modules ./node_modules
COPY --from=builder /prod/package.json ./package.json

EXPOSE 3000


CMD ["node", "build"]
