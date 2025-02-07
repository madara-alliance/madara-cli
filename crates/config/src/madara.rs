#![allow(unused)]
use std::{
    default,
    path::{Path, PathBuf},
};

use clap::{
    error::ErrorKind, ArgMatches, Args, Command, FromArgMatches, Parser, Subcommand, ValueEnum,
};
use cliclack::Confirm;
use madara_cli_common::{Prompt, PromptSelect};
use madara_cli_types::madara::{MadaraMode, MadaraNetwork};
use strum::{EnumIter, IntoEnumIterator};

use crate::constants::{MADARA_DOCKER_IMAGE, MADARA_PRESETS_PATH};

/// Configuration for Madara Devnet Runner
#[derive(Debug, Parser, Clone)]
pub struct MadaraRunnerConfigDevnet {
    #[arg(short, long, default_value = "./madara-devnet-db")]
    pub base_path: String,
}

/// Configuration for Madara Full Node Runner
#[derive(Debug, Default, Clone, Parser)]
pub struct MadaraRunnerConfigFullNode {
    #[arg(short, long, default_value = "./madara-fullnode-db")]
    pub base_path: String,
    #[arg(short, long)]
    pub network: MadaraNetwork,
}

/// Configuration for Madara Sequencer Runner
#[derive(Debug, Clone, Parser)]
pub struct MadaraRunnerConfigSequencer {
    #[arg(short, long, default_value = "configs/presets/devnet.yaml")]
    pub chain_config_path: String,
    #[arg(short, long, default_value = "./madara-sequencer-db")]
    pub base_path: String,
}

/// Madara preset type (e.g., Sepolia, Mainnet, etc.)
#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, Default, EnumIter, strum::Display)]
pub enum MadaraPresetType {
    #[default]
    Sepolia,
    Mainnet,
    Devnet,
    Integration,
    Custom,
}

/// Madara preset configuration (Type and optional path)
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
    AppChain(MadaraRunnerConfigSequencer),
}

/// Configuration for Madara Runner mode (with selected parameters)
#[derive(Debug, Clone, Args)]
pub struct MadaraRunnerConfigMode {
    #[arg(short, long, default_value = "Madara")]
    pub name: String,

    // TODO: @whichqua Replace this when we have an official image
    #[arg(short, long, default_value = MADARA_DOCKER_IMAGE)]
    pub image: String,

    #[clap(subcommand)]
    pub params: MadaraRunnerParams,
}

impl MadaraRunnerConfigFullNode {
    /// Fill FullNode configuration using interactive prompts
    pub fn fill_values_with_prompt() -> anyhow::Result<MadaraRunnerConfigFullNode> {
        let base_path = Prompt::new("Input DB path:")
            .default("./madara-fullnode-db")
            .ask();

        let network = PromptSelect::new("Select Network:", MadaraNetwork::iter()).ask();

        Ok(MadaraRunnerConfigFullNode { base_path, network })
    }
}

impl MadaraRunnerConfigSequencer {
    /// Fill Sequencer configuration using interactive prompts
    pub fn fill_values_with_prompt() -> anyhow::Result<MadaraRunnerConfigSequencer> {
        let base_path = {
            Prompt::new("Input DB path:")
                .default("./madara-sequencer-db")
                .ask()
        };

        let chain_config_path = {
            Prompt::new("Input chain config path:")
                .default("deps/madara/configs/presets/devnet.yaml")
                .ask()
        };

        Ok(MadaraRunnerConfigSequencer {
            base_path,
            chain_config_path,
        })
    }
}

impl MadaraRunnerConfigDevnet {
    /// Fill Devnet configuration using interactive prompts
    pub fn fill_values_with_prompt() -> anyhow::Result<MadaraRunnerConfigDevnet> {
        let base_path = Prompt::new("Input DB path:")
            .default("./madara-devnet-db")
            .ask();

        Ok(MadaraRunnerConfigDevnet { base_path })
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

    /// Fill MadaraRunnerConfigMode with prompts based on selected mode
    pub fn fill_values_with_prompt() -> anyhow::Result<MadaraRunnerConfigMode> {
        let name = "Madara".to_string();
        let mode: MadaraMode = PromptSelect::new("Select Madara mode:", MadaraMode::iter()).ask();
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
            _ => panic!("Not supported yet"),
        };

        Ok(MadaraRunnerConfigMode {
            name,
            image: MADARA_DOCKER_IMAGE.to_owned(),
            params,
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
            Some((_, _)) => Err(clap::error::Error::raw(
                ErrorKind::InvalidSubcommand,
                "Valid subcommands are `devnet`, `full-node` and `sequencer`",
            )),
            None => Err(clap::error::Error::raw(
                ErrorKind::MissingSubcommand,
                "Valid subcommands are `devnet`, `full-node` and `sequencer`",
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
                    "Valid subcommands are `devnet`, `full-node` and `sequencer`",
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
        .subcommand_required(true)
    }

    fn has_subcommand(name: &str) -> bool {
        matches!(name, "devnet" | "full-node" | "sequencer")
    }
}

impl Default for MadaraRunnerConfigMode {
    fn default() -> Self {
        Self {
            name: "Madara".to_owned(),
            params: MadaraRunnerParams::Devnet(MadaraRunnerConfigDevnet {
                base_path: "./madara-devnet-db".to_owned(),
            }),
            image: MADARA_DOCKER_IMAGE.to_owned()
        }
    }
}
