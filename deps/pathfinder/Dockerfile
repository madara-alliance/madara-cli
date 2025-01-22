# Stage 1: Build the pathfinder binary
FROM rust:slim-bookworm AS builder

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive
WORKDIR /pathfinder

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    git \
    build-essential \
    libssl-dev \
    pkg-config \
    unzip \
    && rm -rf /var/lib/apt/lists/*

# Install protobuf (specific version)
RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v28.3/protoc-28.3-linux-x86_64.zip && \
    unzip protoc-28.3-linux-x86_64.zip -d /usr/local && \
    chmod 755 /usr/local/bin/protoc && \
    rm protoc-28.3-linux-x86_64.zip && \
    protoc --version

# Copy the pathfinder source code
COPY pathfinder/ .

# Build the pathfinder binary
RUN cargo build --release --locked --bin pathfinder

# Verify binary presence
RUN ls -la target/release

# Stage 2: Create the runtime image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libzstd1 \
    libgmp10 \
    tini \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN groupadd --gid 1000 pathfinder && \
    useradd --no-log-init --uid 1000 --gid pathfinder --no-create-home pathfinder

# Set up working directory
# WORKDIR /usr/share/pathfinder
WORKDIR /usr/local/bin
COPY --from=builder /pathfinder/target/release/pathfinder /usr/local/bin/pathfinder

# Set ownership and permissions
# RUN chown -R pathfinder:pathfinder /usr/share/pathfinder
RUN chown -R pathfinder:pathfinder /usr/local/bin/pathfinder
USER pathfinder

# Expose RPC port
EXPOSE 9545

# No default entrypoint or command
ENTRYPOINT ["./pathfinder"]
