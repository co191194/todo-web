# =============================================================================
# Base Stage
# =============================================================================
FROM node:22-slim AS base
WORKDIR /workspace/frontend

# Enable corepack for pnpm
RUN corepack enable && corepack prepare pnpm@latest --activate

# =============================================================================
# Development Stage
# =============================================================================
FROM base AS dev

ENV NODE_ENV=development

# Development entry point (overridden in compose.yaml)
CMD ["pnpm", "dev"]

# =============================================================================
# Dependencies Stage
# =============================================================================
FROM base AS deps

COPY frontend/package.json frontend/pnpm-lock.yaml* ./
RUN pnpm install --frozen-lockfile || pnpm install

# =============================================================================
# Builder Stage
# =============================================================================
FROM base AS builder

COPY --from=deps /workspace/frontend/node_modules ./node_modules
COPY frontend/ .

RUN pnpm build

# =============================================================================
# Production Stage
# =============================================================================
FROM base AS prod

ENV NODE_ENV=production

RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs

COPY --from=builder /workspace/frontend/public ./public
COPY --from=builder --chown=nextjs:nodejs /workspace/frontend/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /workspace/frontend/.next/static ./.next/static

USER nextjs

EXPOSE 3000

ENV PORT=3000
ENV HOSTNAME="0.0.0.0"

CMD ["node", "server.js"]
