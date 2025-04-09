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
    # Install Node.js 16.x
    curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Create and set working directory
WORKDIR /app

# Install Starknet, Ethers.js and dotenv
RUN npm install starknet ethers dotenv

# Set the entrypoint
CMD ["bash"]
