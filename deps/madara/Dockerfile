# Stage 1: Build the application
FROM rust:1.81 AS builder

ENV MADARA_VER="bab56d9a6ef3c39aa82767c8530eeff43a3e0dd3"

# Install build dependencies
RUN apt-get -y update && \
    apt-get install -y clang && \
    apt-get autoremove -y; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/*
# Set the working directory
WORKDIR /usr/src/madara/

RUN echo Cloning Madara at commit ${MADARA_VER}
RUN git clone https://github.com/madara-alliance/madara . && \
    git checkout ${MADARA_VER} && \
    git submodule update --init --recursive

# Build the application in release mode
RUN cargo build --release --bin madara
# Stage 2: Create the final runtime image
FROM debian:bookworm
# Install runtime dependencies
RUN apt-get -y update && \
    apt-get install -y openssl ca-certificates tini curl &&\
    apt-get autoremove -y; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/*
# Set the working directory
WORKDIR /usr/local/bin
# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/madara/target/release/madara .

# Set the entrypoint
ENTRYPOINT ["./madara"]
