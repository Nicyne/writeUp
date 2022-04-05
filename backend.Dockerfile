# -------------------- Build Stage --------------------
FROM rust:slim-bullseye AS builder

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

# copy the build artifact from the build stage
COPY --from=builder /writeUp/target/release/writeUp .

# init the port with it's default value
ENV API_PORT=8080

# startup backend-binary
CMD ["./writeUp"]
