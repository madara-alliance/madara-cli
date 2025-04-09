#![allow(unused)]
use std::{
    default,
    path::{Path, PathBuf},
};

use clap::{
    builder::Str, error::ErrorKind, ArgMatches, Args, Command, FromArgMatches, Parser, Subcommand,
    ValueEnum,
};
use cliclack::Confirm;
use madara_cli_common::{validation::validate_url, Prompt, PromptSelect};
use madara_cli_types::madara::{MadaraMode, MadaraNetwork};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    bootstrapper::BootstrapperConfig, constants::MADARA_PRESETS_PATH,
    pathfinder::PathfinderRunnerConfigMode, prover::ProverRunnerConfig,
};

#[derive(Debug, Parser, Clone)]
pub struct MadaraRunnerConfigDevnet {
    #[arg(short, long, default_value = "../data/devnet-db")]
    pub base_path: String,
}

impl MadaraRunnerConfigDevnet {
    pub fn fill_values_with_prompt() -> anyhow::Result<MadaraRunnerConfigDevnet> {
        let base_path = Prompt::new("Input DB folder name:")
            .default("../data/devnet-db")
            .ask();

        Ok(MadaraRunnerConfigDevnet {
            base_path: base_path,
        })
    }
}

impl Default for MadaraRunnerConfigDevnet {
    fn default() -> Self {
        Self {
            base_path: "".to_owned(),
        }
    }
}

#[derive(Debug, Default, Parser, Clone)]
pub struct MadaraRunnerConfigFullNode {
    #[arg(short, long, default_value = "../data/fullnode-db")]
    pub base_path: String,
    #[arg(short, long)]
    pub network: MadaraNetwork,
    #[arg(short, long)]
    pub rpc_api_url: Option<String>,
}

impl MadaraRunnerConfigFullNode {
    pub fn fill_values_with_prompt() -> anyhow::Result<MadaraRunnerConfigFullNode> {
        let base_path = Prompt::new("Input DB folder name:")
            .default("../data/fullnode-db")
            .ask();

        let network = PromptSelect::new("Select Network:", MadaraNetwork::iter()).ask();

        Ok(MadaraRunnerConfigFullNode {
            base_path: base_path,
            network,
            rpc_api_url: None,
        })
    }
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, Default, EnumIter, strum::Display)]
pub enum MadaraPresetType {
    #[default]
    Sepolia,
    Mainnet,
    Devnet,
    Integration,
    Custom,
}

#[derive(Debug, Clone, Args)]
pub struct MadaraPreset {
    #[arg(short, long)]
    pub preset_type: MadaraPresetType,
    #[arg(long)]
    pub path: Option<String>,
}

#[derive(Debug, Clone)]
pub enum MadaraRunnerParams {
    Devnet(MadaraRunnerConfigDevnet),
    Sequencer(MadaraRunnerConfigSequencer),
    FullNode(MadaraRunnerConfigFullNode),
    AppChain(MadaraRunnerConfigAppChain),
}

impl Default for MadaraRunnerParams {
    fn default() -> Self {
        Self::Devnet(MadaraRunnerConfigDevnet::default())
    }
}

#[derive(Debug, Parser, Clone)]
pub struct MadaraRunnerConfigMode {
    #[arg(short, long, default_value = "Madara")]
    pub name: String,
    pub mode: Option<MadaraMode>,
    #[clap(subcommand)]
    pub params: MadaraRunnerParams,
}

impl Default for MadaraRunnerConfigMode {
    fn default() -> Self {
        Self {
            name: String::new(),
            mode: None,
            params: MadaraRunnerParams::default(),
        }
    }
}

impl MadaraRunnerConfigMode {
    /// Retrieve MadaraMode from current configuration
    pub fn mode(&self) -> MadaraMode {
        match &self.params {
            MadaraRunnerParams::Devnet(_) => MadaraMode::Devnet,
            MadaraRunnerParams::Sequencer(_) => MadaraMode::Sequencer,
            MadaraRunnerParams::FullNode(_) => MadaraMode::FullNode,
            MadaraRunnerParams::AppChain(_) => MadaraMode::AppChain,
        }
    }

    pub fn fill_values_with_prompt() -> anyhow::Result<MadaraRunnerConfigMode> {
        let name = "Madara".to_string();
        let mode = PromptSelect::new("Select Madara mode:", MadaraMode::iter()).ask();

        let params = match mode {
            MadaraMode::Devnet => {
                MadaraRunnerParams::Devnet(MadaraRunnerConfigDevnet::fill_values_with_prompt()?)
            }
            MadaraMode::Sequencer => MadaraRunnerParams::Sequencer(
                MadaraRunnerConfigSequencer::fill_values_with_prompt()?,
            ),
            MadaraMode::FullNode => {
                MadaraRunnerParams::FullNode(MadaraRunnerConfigFullNode::fill_values_with_prompt()?)
            }
            MadaraMode::AppChain => {
                MadaraRunnerParams::AppChain(MadaraRunnerConfigAppChain::fill_values_with_prompt()?)
            }
        };

        Ok(MadaraRunnerConfigMode {
            name,
            mode: Some(mode),
            params,
        })
    }
}

#[derive(Debug, Parser, Clone)]
pub struct MadaraRunnerConfigSequencer {
    #[arg(long, default_value = "../data/sequencer-db")]
    pub base_path: String,
    #[arg(long, default_value = "configs/presets/devnet.yaml")]
    pub chain_config_path: String,
    #[arg(long)]
    pub l1_endpoint: Option<String>,
}

impl MadaraRunnerConfigSequencer {
    pub fn fill_values_with_prompt() -> anyhow::Result<MadaraRunnerConfigSequencer> {
        let base_path = Prompt::new("Input DB folder name:")
            .default("../data/sequencer-db")
            .ask();

        let chain_config_path = Prompt::new("Input chain config path:")
            .default("configs/presets/devnet.yaml")
            .ask();

        let l1_endpoint: String = Prompt::new("L1 endpoint (leave empty for no-sync)")
            .allow_empty()
            .validate_interactively(validate_url)
            .ask();

        let l1_endpoint = if l1_endpoint.is_empty() {
            None
        } else {
            Some(l1_endpoint)
        };

        Ok(MadaraRunnerConfigSequencer {
            base_path,
            chain_config_path,
            l1_endpoint,
        })
    }
}

impl Default for MadaraRunnerConfigSequencer {
    fn default() -> Self {
        Self {
            base_path: "".to_string(),
            chain_config_path: "configs/presets/devnet.yaml".to_string(),
            l1_endpoint: None,
        }
    }
}

/// Configuration for Madara Appchain Runner
#[derive(Debug, Clone, Parser)]
pub struct MadaraRunnerConfigAppChain {
    #[arg(long, default_value = "configs/presets/devnet.yaml")]
    pub chain_config_path: String,

    #[clap(flatten)]
    pub prover_config: ProverRunnerConfig,

    #[clap(flatten)]
    pub pathfinder_config: PathfinderRunnerConfigMode,

    #[clap(flatten)]
    pub bootstrapper_config: BootstrapperConfig,
}

impl MadaraRunnerConfigAppChain {
    /// Fill AppChain configuration using interactive prompts
    pub fn fill_values_with_prompt() -> anyhow::Result<MadaraRunnerConfigAppChain> {
        let chain_config_path = Prompt::new("Input chain config path:")
            .default("configs/presets/devnet.yaml")
            .ask();

        let prover_config = ProverRunnerConfig::fill_values_with_prompt()?;

        let bootstrapper_config = BootstrapperConfig::fill_values_with_prompt()?;

        Ok(MadaraRunnerConfigAppChain {
            chain_config_path,
            prover_config,
            pathfinder_config: PathfinderRunnerConfigMode::default(),
            bootstrapper_config,
        })
    }
}

impl FromArgMatches for MadaraRunnerParams {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::error::Error> {
        match matches.subcommand() {
            Some(("devnet", args)) => Ok(Self::Devnet(MadaraRunnerConfigDevnet::from_arg_matches(
                args,
            )?)),
            Some(("full-node", args)) => Ok(Self::FullNode(
                MadaraRunnerConfigFullNode::from_arg_matches(args)?,
            )),
            Some(("sequencer", args)) => Ok(Self::Sequencer(
                MadaraRunnerConfigSequencer::from_arg_matches(args)?,
            )),
            Some(("app-chain", args)) => Ok(Self::AppChain(
                MadaraRunnerConfigAppChain::from_arg_matches(args)?,
            )),
            Some((_, _)) => Err(clap::error::Error::raw(
                ErrorKind::InvalidSubcommand,
                "Valid subcommands are `devnet`, `full-node` and `sequencer`",
            )),
            None => Err(clap::error::Error::raw(
                ErrorKind::MissingSubcommand,
                "Valid subcommands are `devnet`, `full-node`, `sequencer` and `app-chain`",
            )),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap::error::Error> {
        match matches.subcommand() {
            Some(("devnet", args)) => {
                *self = Self::Devnet(MadaraRunnerConfigDevnet::from_arg_matches(args)?)
            }
            Some(("full-node", args)) => {
                *self = Self::FullNode(MadaraRunnerConfigFullNode::from_arg_matches(args)?)
            }
            Some(("sequencer", args)) => {
                *self = Self::Sequencer(MadaraRunnerConfigSequencer::from_arg_matches(args)?)
            }
            Some((_, _)) => {
                return Err(clap::error::Error::raw(
                    ErrorKind::InvalidSubcommand,
                    "Valid subcommands are `devnet`, `full-node`, `sequencer` and `app-chain`",
                ))
            }
            None => (),
        };
        Ok(())
    }
}

impl Subcommand for MadaraRunnerParams {
    fn augment_subcommands(cmd: Command) -> Command {
        cmd.subcommand(MadaraRunnerConfigDevnet::augment_args(Command::new(
            "devnet",
        )))
        .subcommand(MadaraRunnerConfigFullNode::augment_args(Command::new(
            "full-node",
        )))
        .subcommand(MadaraRunnerConfigSequencer::augment_args(Command::new(
            "sequencer",
        )))
        .subcommand(MadaraRunnerConfigAppChain::augment_args(Command::new(
            "app-chain",
        )))
        .subcommand_required(true)
    }

    fn augment_subcommands_for_update(cmd: Command) -> Command {
        cmd.subcommand(MadaraRunnerConfigDevnet::augment_args(Command::new(
            "devnet",
        )))
        .subcommand(MadaraRunnerConfigFullNode::augment_args(Command::new(
            "full-node",
        )))
        .subcommand(MadaraRunnerConfigSequencer::augment_args(Command::new(
            "sequencer",
        )))
        .subcommand(MadaraRunnerConfigAppChain::augment_args(Command::new(
            "app-chain",
        )))
        .subcommand_required(true)
    }

    fn has_subcommand(name: &str) -> bool {
        matches!(name, "devnet" | "full-node" | "sequencer" | "app-chain")
    }
}
