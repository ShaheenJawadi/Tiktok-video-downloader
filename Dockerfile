# Builder stage
FROM rust:1.83-slim-bullseye as builder
LABEL authors="shaheen"

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /usr/src/app
RUN cargo new --bin tiktok-video-downloader

# Copy over manifests
COPY ./Cargo.lock ./tiktok-video-downloader/Cargo.lock
COPY ./Cargo.toml ./tiktok-video-downloader/Cargo.toml

# Change working directory
WORKDIR /usr/src/app/tiktok-video-downloader

# Build dependencies - this is the caching Docker layer for dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/tiktok_video_downloader*
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    openssl \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/tiktok-video-downloader/target/release/tiktok-video-downloader /usr/local/bin/

# Create a non-root user
RUN useradd -ms /bin/bash appuser
USER appuser

# Set the entrypoint
ENTRYPOINT ["tiktok-video-downloader"]