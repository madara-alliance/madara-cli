FROM rust:1.81 AS builder

# Install system dependencies
RUN apt update && apt install -y  \
    git \
    libgmp3-dev \
    software-properties-common \
    wget \
    bash \
    nodejs \
    npm

# Set the working directory
WORKDIR /usr/src

# Clone the specific branch of the repository
RUN git clone --branch unignore_cargo_lock --depth 1 https://github.com/madara-alliance/madara-orchestrator.git

WORKDIR /usr/src/madara-orchestrator

# Install Python 3.9
RUN wget https://www.python.org/ftp/python/3.9.16/Python-3.9.16.tgz \
    && tar xzf Python-3.9.16.tgz \
    && cd Python-3.9.16 \
    && ./configure --enable-optimizations \
    && make altinstall \
    && cd .. \
    && rm -rf Python-3.9.16 Python-3.9.16.tgz

# Install pip and venv for Python 3.9
RUN wget https://bootstrap.pypa.io/get-pip.py \
    && python3.9 get-pip.py \
    && rm get-pip.py \
    && python3.9 -m pip install virtualenv

# Check rust version (this also installs version from rust-toolchain file)
RUN rustup show

# #############################################################
# TODO : remove this step after snos build is sorted
# Build cairo lang
RUN cargo fetch
RUN python3.9 -m venv orchestrator_venv
RUN . ./orchestrator_venv/bin/activate
RUN pip install cairo-lang==0.13.2 "sympy<1.13.0"
RUN mkdir -p build
RUN git submodule update --init --recursive
RUN cd cairo-lang
RUN cd ..
RUN cairo-compile cairo-lang/src/starkware/starknet/core/os/os.cairo --output build/os_latest.json --cairo_path cairo-lang/src
# #############################################################

# Build the project
RUN cargo build --release

# Install Node.js dependencies for migrations
RUN npm install


FROM debian:bookworm

# Install runtime dependencies
RUN apt-get -y update && \
    apt-get install -y openssl ca-certificates nodejs npm curl jq && \
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/madara-orchestrator/target/release/orchestrator .

# Copy Node.js files and dependencies
COPY --from=builder /usr/src/madara-orchestrator/node_modules ./node_modules
COPY --from=builder /usr/src/madara-orchestrator/package.json .
COPY --from=builder /usr/src/madara-orchestrator/migrate-mongo-config.js .
COPY --from=builder /usr/src/madara-orchestrator/migrations ./migrations

# To be fixed by this https://github.com/keep-starknet-strange/snos/issues/404
RUN mkdir -p /usr/local/cargo/git/checkouts/snos-59fe8329bb16fe65/662d170/crates/starknet-os/kzg
RUN mkdir -p /usr/local/cargo/git/checkouts/snos-59fe8329bb16fe65/ea7d63f/crates/starknet-os/kzg
RUN mkdir -p /usr/local/cargo/git/checkouts/snos-59fe8329bb16fe65/2c47281/crates/starknet-os/kzg

COPY --from=builder /usr/src/madara-orchestrator/crates/da-clients/ethereum/trusted_setup.txt /usr/local/cargo/git/checkouts/snos-59fe8329bb16fe65/662d170/crates/starknet-os/kzg/trusted_setup.txt
COPY --from=builder /usr/src/madara-orchestrator/crates/da-clients/ethereum/trusted_setup.txt /usr/local/cargo/git/checkouts/snos-59fe8329bb16fe65/ea7d63f/crates/starknet-os/kzg/trusted_setup.txt
COPY --from=builder /usr/src/madara-orchestrator/crates/da-clients/ethereum/trusted_setup.txt /usr/local/cargo/git/checkouts/snos-59fe8329bb16fe65/2c47281/crates/starknet-os/kzg/trusted_setup.txt
COPY --from=builder /usr/src/madara-orchestrator/crates/da-clients/ethereum/trusted_setup.txt /usr/src/madara-orchestrator/crates/settlement-clients/ethereum/src/trusted_setup.txt

ENTRYPOINT ["/bin/bash"]
