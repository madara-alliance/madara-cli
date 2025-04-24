# Madara Node CLI

This Command Line Interface (CLI) is designed to simplify the process of setting up a Madara node. Madara is a high-performance StarkNet sequencer implementation designed for scalability and efficiency in the Starknet ecosystem. With this tool, you can easily spin up a Madara node configured for one of four different operational modes:

* **FullNode**: A node that maintains a complete copy of the blockchain state and can independently verify all transactions and blocks.
* **Sequencer**: A node responsible for ordering transactions, creating blocks, and maintaining consensus within the network.
* **Devnet**: A local development network that functions like a Sequencer but comes with predeployed and funded accounts, as well as test contracts for development purposes. Ideal for developers building and testing applications on StarkNet.
* **AppChain**: A comprehensive suite of tools (including Madara, Orchestrator, Bootstrapper, and others) that enables hassle-free deployment of L2 or L3 solutions with settlement in Ethereum or Starknet. This mode provides everything needed for a production-ready blockchain deployment.

## Prerequisites

Before using the Madara CLI, ensure you have the following installed:

- **Docker** (version 20.10.0 or higher) and Docker Compose
- **Rust** (version 1.70.0 or higher)
- Minimum hardware requirements:
  - 4 CPU cores
  - 8GB RAM
  - 50GB free disk space (more recommended for FullNode and Sequencer modes)
- Open ports: 9944 (P2P), 9933 (RPC), 9615 (Metrics)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/madara-alliance/madara-cli.git
   cd madara-cli
   ```

2. Build the CLI:
   ```bash
   cargo build
   ```

## Containerized Solution

Madara CLI runs in a containerized environment, providing all necessary Dockerfiles to build the images locally. The containerization approach ensures:

- Consistent deployment across different environments (development, testing, production)
- Isolation from the host system to prevent dependency conflicts
- Simplified management of complex runtime dependencies
- Easy scaling and orchestration capabilities

For quicker testing, you can use pre-built Docker images instead of building locally, which can save significant time as the building process is resource-intensive due to Rust compilation and optimization processes.

**Pre-built Docker Images:** [https://hub.docker.com/u/mslmadara](https://hub.docker.com/u/mslmadara)

By using this CLI, you can streamline the node setup process, customize your deployment configuration, and quickly get started with your Madara project without dealing with complex manual setup procedures.

## Usage

The Madara CLI provides two main commands:

### `init` - Create Configuration File

The `init` command is used to generate a configuration file for AppChain deployment. This process dumps all necessary parameters into a single TOML file:

```bash
cargo run init [--default]
```

When you run this command, you'll be guided through an interactive menu where you'll need to complete all required parameters for your AppChain. The configuration is divided into four main sections:

1. **L1 Configuration** - Settlement layer settings
2. **ETH Wallet Configuration** - Wallet settings for transactions
3. **Madara Configuration** - Core node settings
4. **Orchestrator Configuration** - Orchestration service settings

For a better understanding of all available parameters and their descriptions, you can refer to the [local.toml](./local.toml) file which contains comments for each parameter.

Once this process is completed, your new configuration file will be saved under the `deps/data` directory. You can then use this file to spin up your AppChain with the `create` command.

Use the `--default` flag to skip user interaction and create a configuration file that clones the template.

### `create` - Spin Up Madara Node

The `create` command is used to spin up a Madara node with your chosen configuration:

```bash
cargo run create [--config-file <path_to_config_file>]
```

This command will:
- Ask you to select a mode (Devnet, FullNode, Sequencer, or AppChain)
- Request all necessary parameters based on the selected mode
- Spin up Madara with the provided configuration

For FullNode, Sequencer, and Devnet modes, the process is straightforward. The AppChain mode requires more time as it involves deploying multiple services.

For AppChain deployments:
- If no config file is provided, the CLI will use the `local.toml` configuration, which is designed for local deployments with all services running on your machine
- If you provide a config file, you can customize settings like using Sepolia testnet, modifying Madara/Orchestrator parameters, or using different accounts to deploy contracts

#### Fully Interactive Mode

For a fully guided experience, you can simply run:

```bash
cargo run
```

This will launch an interactive menu that guides you through all available options.

#### Non-Interactive Mode

If you want to automate usage and avoid interactive prompts, you can specify the mode directly:

```bash
cargo run create MODE
```

Where `MODE` is one of: `devnet`, `full-node`, `sequencer`, or `app-chain`.

This will execute the CLI with the specified mode using default configurations. While this approach might not fully adapt to all specific needs, it provides a quick way to test the CLI.

## Examples

### Running a Local Devnet

For local development and testing:

```bash
# Start a devnet with default settings
cargo run create devnet

# Access the RPC endpoint at http://localhost:9944
```

### Setting Up a Full Node

To synchronize with an existing network:

```bash
# Start a full node connected to the Sepolia testnet
cargo run create full-node

# Specify network and RPC endpoint
cargo run create full-node --network testnet --rpc-api-url https://valid-sepolia-rpc
```

### Deploying an AppChain with Custom Configuration

To deploy a custom L2/L3 solution:

```bash
# First create a configuration file
cargo run init

# Edit parameters as needed, then deploy
cargo run create app-chain --config-file deps/data/my-config.toml
```

## Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| Docker permission errors | Run with `sudo` or add your user to the Docker group |
| Port conflicts | Ensure ports 9944, 9933, and 9615 are available or change them in the configuration |
| Out of disk space | Free up disk space or use a volume with sufficient capacity |
| AppChain deployment fails | Check L1 connection, wallet balance, and network configuration |
| Connection refused | Ensure Docker is running and network settings are correct |
