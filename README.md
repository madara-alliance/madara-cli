# Madara Node CLI

This is a Command Line Interface (CLI) designed to simplify the process of setting up a Madara node. With this tool, you can easily spin up a Madara node under four different types:

* Devnet
* FullNode
* Sequencer
* AppChain

By using this CLI, you can streamline the node setup process and get started with your Madara project quickly and efficiently.

## Usage
```bash
git submodule update --init --recursive
```
### Running in cli mode
You usually will know what mode you want to run. For example, to run Devnet:

```bash
madara create devnet
```
To learn more, checkout the help command:
```
madara --help
```
This results in:
```
Madara CLI to easily spin up nodes

Usage: madara [OPTIONS] [COMMAND]

Commands:
  create        Create a Madara node
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

Global options:
  -v, --verbose  Verbose mode
```

### Running in interactive mode
If you are unsure what node config you want to run, you can use the interactive mode:
```bash
madara
```
This will allow you to select the config you want:
```
●  No commands entered, switching to interactive mode...
│  
◆  Select Madara mode:
│  ● Devnet 
│  ○ Sequencer 
│  ○ FullNode 
│  ○ AppChain 
```


