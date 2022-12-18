# -------------------- Frontend | Dependency Stage --------------------
FROM node:18-alpine AS deps
RUN apk add --no-cache libc6-compat

# prepare the project
WORKDIR /app
COPY ./client/package.json ./client/yarn.lock ./

# install the dependencies
RUN yarn install --frozen-lockfile

# -------------------- Frontend | Build Stage --------------------
FROM node:18-alpine AS frontend-builder

WORKDIR /app

ENV NEXT_TELEMETRY_DISABLED 1
ENV NODE_ENV production

# collect compiled depencies and project-files
COPY --from=deps /app/node_modules ./node_modules
COPY client/. .

# build the project
RUN yarn build

# -------------------- Backend | Build Stage --------------------
FROM rust:slim-bullseye AS backend-builder

#Add clang-dependency (argonautica)
RUN apt-get update  \
    && apt-get install -y clang llvm-dev libclang-dev  \
    && rm -rf /var/lib/apt/lists/*

# create a new dummy project
RUN USER=root cargo new --bin writeUp
WORKDIR /writeUp

# copy over the manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# compile all dependencies
RUN cargo build --release

# remove dummy-project related files
RUN rm ./target/release/deps/writeUp*
RUN rm src/*.rs

# copy the source tree
COPY ./src ./src

# build for release
RUN cargo build --release

# -------------------- Deploy Stage --------------------
FROM debian:bullseye-slim

# copy the build artifacts from the build stages
COPY --from=backend-builder /writeUp/target/release/writeUp .
COPY --from=frontend-builder /app/build ./public

# copy the logger-configuration
COPY ./log-config.yml .

# startup writeup-binary
CMD ["./writeUp"]
