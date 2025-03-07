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

# Install Foundry
RUN curl -L https://foundry.paradigm.xyz | bash && \
    /bin/bash -c "source /root/.bashrc && ~/.foundry/bin/foundryup"

# Add foundry binaries to PATH
ENV PATH="/root/.foundry/bin:${PATH}"

# Create and set working directory
WORKDIR /app

# Install Starknet and Ethers.js
RUN npm install starknet ethers

# Copy files into the image
COPY MockGPSVerifier.sol /app/
COPY Starknet.json /app/
COPY core_contract_deployment.sh /app/
COPY madara_state_update.sh /app/

# Set the entrypoint
CMD ["bash"]
