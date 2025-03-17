#![allow(unused)]
use std::{
    default,
    path::{Path, PathBuf},
};

use clap::{builder::Str, Parser, ValueEnum};
use madara_cli_common::{validation::validate_url, Prompt, PromptSelect};
use madara_cli_types::madara::{MadaraMode, MadaraNetwork};
use strum::{EnumIter, IntoEnumIterator};

use crate::constants::MADARA_PRESETS_PATH;

#[derive(Debug, Parser)]
pub struct MadaraRunnerConfigDevnet {
    pub base_path: Option<String>,
}

impl Default for MadaraRunnerConfigDevnet {
    fn default() -> Self {
        Self { base_path: None }
    }
}

#[derive(Debug, Default)]
pub struct MadaraRunnerConfigFullNode {
    pub base_path: Option<String>,
    pub network: Option<MadaraNetwork>,
}

impl MadaraRunnerConfigFullNode {
    pub fn fill_values_with_prompt(mut self) -> anyhow::Result<MadaraRunnerConfigFullNode> {
        let base_path = self.base_path.unwrap_or_else(|| {
            Prompt::new("Input DB folder name:")
                .default("fullnode-db")
                .ask()
        });

        let network = self
            .network
            .unwrap_or_else(|| PromptSelect::new("Select Network:", MadaraNetwork::iter()).ask());

        Ok(MadaraRunnerConfigFullNode {
            base_path: Some(base_path),
            network: Some(network),
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

#[derive(Clone)]
pub struct MadaraPreset {
    pub preset_type: MadaraPresetType,
    pub path: Option<String>,
}

pub struct MadaraRunnerConfigSequencer {
    pub base_path: Option<String>,
    pub chain_config_path: Option<String>,
    pub l1_endpoint: Option<String>,
}

pub enum MadaraRunnerParams {
    Devnet(MadaraRunnerConfigDevnet),
    Sequencer(MadaraRunnerConfigSequencer),
    FullNode(MadaraRunnerConfigFullNode),
    AppChain(MadaraRunnerConfigSequencer),
}

impl Default for MadaraRunnerParams {
    fn default() -> Self {
        Self::Devnet(MadaraRunnerConfigDevnet::default())
    }
}

pub struct MadaraRunnerConfigMode {
    pub name: String,
    pub mode: Option<MadaraMode>,
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
    pub fn fill_values_with_prompt(mut self) -> anyhow::Result<MadaraRunnerConfigMode> {
        let name = "Madara".to_string();
        let mode = self
            .mode
            .unwrap_or_else(|| PromptSelect::new("Select Madara mode:", MadaraMode::iter()).ask());

        let params = match mode {
            MadaraMode::Devnet => MadaraRunnerParams::Devnet(
                MadaraRunnerConfigDevnet::default().fill_values_with_prompt()?,
            ),
            MadaraMode::Sequencer => MadaraRunnerParams::Sequencer(
                MadaraRunnerConfigSequencer::default().fill_values_with_prompt()?,
            ),
            MadaraMode::FullNode => MadaraRunnerParams::FullNode(
                MadaraRunnerConfigFullNode::default().fill_values_with_prompt()?,
            ),
            MadaraMode::AppChain => {
                MadaraRunnerParams::AppChain(MadaraRunnerConfigSequencer::default())
            }
        };

        Ok(MadaraRunnerConfigMode {
            name: name,
            mode: Some(mode),
            params,
        })
    }
}

impl MadaraRunnerConfigDevnet {
    pub fn fill_values_with_prompt(mut self) -> anyhow::Result<MadaraRunnerConfigDevnet> {
        let base_path = self.base_path.unwrap_or_else(|| {
            Prompt::new("Input DB folder name:")
                .default("devnet-db")
                .ask()
        });

        Ok(MadaraRunnerConfigDevnet {
            base_path: Some(base_path),
        })
    }
}

impl MadaraRunnerConfigSequencer {
    pub fn fill_values_with_prompt(mut self) -> anyhow::Result<MadaraRunnerConfigSequencer> {
        let base_path = self.base_path.unwrap_or_else(|| {
            Prompt::new("Input DB folder name:")
                .default("sequencer-db")
                .ask()
        });

        let chain_config_path = self.chain_config_path.unwrap_or_else(|| {
            Prompt::new("Input chain config path:")
                .default("configs/presets/devnet.yaml")
                .ask()
        });

        let l1_endpoint = self.l1_endpoint.unwrap_or_else(|| {
            Prompt::new("L1 endpoint (leave empty for no-sync)")
                .allow_empty()
                .validate_interactively(validate_url)
                .ask()
        });
        let l1_endpoint = if l1_endpoint.is_empty() {
            None
        } else {
            Some(l1_endpoint)
        };

        Ok(MadaraRunnerConfigSequencer {
            base_path: Some(base_path),
            chain_config_path: Some(chain_config_path),
            l1_endpoint,
        })
    }
}

impl Default for MadaraRunnerConfigSequencer {
    fn default() -> Self {
        Self {
            base_path: None,
            chain_config_path: Some("configs/presets/devnet.yaml".to_string()),
            l1_endpoint: None,
        }
    }
}
