# Start from the latest Rust base image
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin rust-api
WORKDIR /rust-api

# Copy over your manifest
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release.
# This will be much slower, but the resulting artifact will be much smaller.
RUN rm ./target/release/deps/rust_api*
RUN cargo build --release

# Start a new stage. This is necessary for smaller image size.
FROM debian:bookworm-slim

# Install OpenSSL, needed by our application
RUN apt-get update && apt-get -y install openssl libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage and set the executable's permissions
COPY --from=builder /rust-api/target/release/rust-api /usr/local/bin

# Set the startup command to run your binary
CMD ["rust-api"]