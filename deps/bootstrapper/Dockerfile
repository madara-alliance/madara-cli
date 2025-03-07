# ==============================================
# Image that builds stargate_contracts_old
FROM python:3.7-slim-bullseye AS starkgate_contracts_legacy

# Set shell options for better error handling
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    make \
    libgmp-dev \
    g++ \
    python3-dev \
    npm \
    unzip \
    curl \
    cmake \
    git \
    # sudo \
    vim \
    # libboost-all-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Python and other dependencies in a single layer
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir cmake==3.22 && \
    npm install -g --unsafe-perm ganache-cli@6.12.2

# Install specific solc version
ENV SOLC_VERSION="0.6.12"
ENV SOLC_PATH="/usr/local/bin/solc-$SOLC_VERSION"

RUN if [ ! -f "$SOLC_PATH" ]; then \
    echo "Installing solc $SOLC_VERSION..." && \
    apt-get update && apt-get install -y wget && \
    wget https://github.com/ethereum/solidity/releases/download/v${SOLC_VERSION}/solc-static-linux -O "$SOLC_PATH" && \
    chmod +x "$SOLC_PATH"; \
    fi

# Set working directory
WORKDIR /app

# Clone repository
RUN git clone --branch update-cairo-0.9.0 --recurse-submodules https://github.com/starknet-io/starkgate-contracts.git .

# Build application
RUN rm -rf build && \
    ./build.sh

# ==============================================
# Image that builds the remaining contracts and bootstrapper
# starkgate-contracts-latest, braavos-account-cairo and argent-contracts-starknet
FROM ubuntu:22.04 AS builder

# Install basic dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    python3 \
    python3-pip \
    python3-venv \
    python3-dev \
    python3-cffi \
    libffi-dev \
    make \
    libgmp-dev \
    g++ \
    unzip \
    cmake \
    sudo \
    libboost-all-dev \
    wget \
    software-properties-common \
    && rm -rf /var/lib/apt/lists/*

# Build solidity from source
ENV SOLIDITY_VERSION=v0.8.20
RUN git clone https://github.com/ethereum/solidity.git && \
    cd solidity && \
    git checkout ${SOLIDITY_VERSION} && \
    ./scripts/build.sh && \
    echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc

# Setup Python virtual environment for main build
ENV VIRTUAL_ENV=/opt/venv
RUN python3 -m venv $VIRTUAL_ENV
ENV PATH="$VIRTUAL_ENV/bin:$PATH"

# Upgrade pip and install required Python packages
RUN python3 -m pip install --upgrade pip && \
    python3 -m pip install cffi && \
    python3 -m pip install solc-select && \
    solc-select install 0.8.19 && \
    solc-select use 0.8.19

# Install Foundry
SHELL ["/bin/bash", "-c"]
RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/root/.foundry/bin:${PATH}"
RUN source /root/.bashrc && foundryup

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install asdf and scarb
RUN git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.13.1 && \
    echo '. "$HOME/.asdf/asdf.sh"' >> ~/.bashrc && \
    echo '. "$HOME/.asdf/completions/asdf.bash"' >> ~/.bashrc && \
    . "$HOME/.asdf/asdf.sh" && \
    asdf plugin add scarb && \
    asdf install scarb 2.8.4 && \
    asdf global scarb 2.8.4

# Install Node.js 18.x
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get update && \
    apt-get install -y nodejs && \
    # Install a specific compatible version of npm
    npm install -g npm@9.8.1 && \
    # Clean up
    rm -rf /var/lib/apt/lists/*

# Install ganache
RUN npm install -g --unsafe-perm ganache@7.9.0

WORKDIR /app

RUN git clone --branch ft/makefile_fix --recurse-submodules https://github.com/madara-alliance/madara-bootstrapper.git .

RUN make setup-linux

# Generate other artifacts
RUN . "$HOME/.asdf/asdf.sh" && \
    export PATH="/app/.cairo/cairo/bin:$PATH" && \
    rm -rf venv \
    which starknet-compile && \
    SOLC=/usr/local/bin/solc make starkgate-contracts-latest && \
    SOLC=/usr/local/bin/solc make braavos-account-cairo && \
    asdf install scarb 2.6.3 && \
    SOLC=/usr/local/bin/solc make argent-contracts-starknet

COPY --from=starkgate_contracts_legacy /app/build/Release/src/starkware/starknet/apps/starkgate/artifacts/cairo/token_bridge_1.json /app/artifacts/legacy_token_bridge.json
COPY --from=starkgate_contracts_legacy /app/build/Release/src/starkware/starknet/std_contracts/upgradability_proxy/proxy.json /app/artifacts/proxy_starkgate.json
COPY --from=starkgate_contracts_legacy /app/build/Release/src/starkware/starknet/std_contracts/ERC20/ERC20.json /app/artifacts/ERC20.json

RUN export PATH="/usr/local/bin:$PATH" && cargo build --release --workspace --bin madara-bootstrapper

# Environment variables
ENV RUST_LOG=info RUST_BACKTRACE=1

ENTRYPOINT ["/bin/bash"]
