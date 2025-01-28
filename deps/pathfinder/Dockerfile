# Stage 1: Build the pathfinder binary
FROM rust:slim-bookworm AS builder

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive

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


# Clone and build Pathfinder
RUN git clone https://github.com/Moonsong-Labs/pathfinder.git && \
    cd pathfinder && \
    git checkout notlesh/snos-requirements-2024-12-30 && \
    cargo build --release --bin pathfinder

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
WORKDIR /usr/local/bin
COPY --from=builder /pathfinder/target/release/pathfinder /usr/local/bin/pathfinder

# Set ownership and permissions
RUN chown -R pathfinder:pathfinder /usr/local/bin/pathfinder
USER pathfinder

# Expose RPC port
EXPOSE 9545

# No default entrypoint or command
ENTRYPOINT ["./pathfinder"]
