# -------------------- Build Stage --------------------
FROM rust:slim-bullseye AS builder

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

# copy the build artifact from the build stage
COPY --from=builder /writeUp/target/release/writeUp .

# copy the logger-configuration
COPY ./log-config.yml .

# startup backend-binary
CMD ["./writeUp"]
