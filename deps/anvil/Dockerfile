# Dockerfile
FROM ubuntu:20.04

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && \
    apt-get install -y \
    curl \
    git \
    jq \
    build-essential && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Foundry
RUN curl -L https://foundry.paradigm.xyz | bash && \
    /bin/bash -c "source /root/.bashrc && ~/.foundry/bin/foundryup"

# Add foundry binaries to PATH
ENV PATH="/root/.foundry/bin:${PATH}"

# Create and set working directory
WORKDIR /app

# Copy files into the image
COPY MockGPSVerifier.sol /app/
COPY Starknet.json /app/
COPY core_contract_deployment.sh /app/

# Set the entrypoint
CMD ["bash"]
