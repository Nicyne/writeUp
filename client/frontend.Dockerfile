# -------------------- Dependency Stage --------------------
FROM node:17-alpine AS deps
RUN apk add --no-cache libc6-compat

# prepare the project
WORKDIR /app
COPY package.json yarn.lock ./

# install the dependencies
RUN yarn install --frozen-lockfile

# -------------------- Build Stage --------------------
FROM node:17-alpine AS builder

WORKDIR /app

ENV NEXT_TELEMETRY_DISABLED 1

# collect compiled depencies and project-files
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# build the project
RUN yarn build

# -------------------- Deploy Stage --------------------
FROM node:17-alpine

WORKDIR /app

ENV NODE_ENV production
ENV NEXT_TELEMETRY_DISABLED 1

# create user
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs

# collect prepared project files
COPY --from=builder /app/public ./public
COPY --from=builder /app/package.json ./package.json
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs

# init the port with it's default value
ENV PORT 3000

# startup frontend
CMD ["node", "server.js"]