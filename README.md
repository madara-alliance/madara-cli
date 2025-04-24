# Madara Node CLI

This Command Line Interface (CLI) is designed to simplify the process of setting up a Madara node. Madara is a high-performance StarkNet sequencer implementation designed for scalability and efficiency in the Starknet ecosystem. With this tool, you can easily spin up a Madara node configured for one of four different operational modes:

* **FullNode**: A node that maintains a complete copy of the blockchain state and can independently verify all transactions and blocks.
* **Sequencer**: A node responsible for ordering transactions, creating blocks, and maintaining consensus within the network.
* **Devnet**: A local development network that functions like a Sequencer but comes with predeployed and funded accounts, as well as test contracts for development purposes. Ideal for developers building and testing applications on StarkNet.
* **AppChain**: A comprehensive suite of tools (including Madara, Orchestrator, Bootstrapper, and others) that enables hassle-free deployment of L2 or L3 solutions with settlement in Ethereum or Starknet. This mode provides everything needed for a production-ready blockchain deployment.

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
To create a custom configuration for AppChain, use:
  ```bash
  cargo run init [--default]
  ```
This command will allow the user to start from the template configuration to spin up a local AppChain and change all the parameters.
Use `default` flag to skip user interaction in this command. This will basically create a configuration file that clones the template.

To run the CLI and spin up Madara, use:
  ```bash
  cargo run create [--config-file <path_to_config_file>]
  ```

Use `config-file` param to set your custom configuration for AppChain

