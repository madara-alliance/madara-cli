 # Madara Node CLI

This Command Line Interface (CLI) is designed to simplify the process of setting up a Madara node. With this tool, you can easily spin up a Madara node under four different types:

* Devnet
* FullNode
* Sequencer
* AppChain

By using this CLI, you can streamline the node setup process and quickly get started with your Madara project.

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

