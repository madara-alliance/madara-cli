# Dockerfile
FROM ubuntu:22.04

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && \
    apt-get install -y \
    curl \
    git \
    jq \
    build-essential && \
    # Install Node.js 20.x (current LTS)
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Foundry
RUN curl -L https://foundry.paradigm.xyz | bash && \
    /bin/bash -c "source /root/.bashrc && ~/.foundry/bin/foundryup"

# Add foundry binaries to PATH
ENV PATH="/root/.foundry/bin:${PATH}"

# Create and set working directory
WORKDIR /app

# Install Starknet and Ethers.js
RUN npm install starknet ethers dotenv

# Set the entrypoint
CMD ["bash"]
