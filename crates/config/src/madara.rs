#![allow(unused)]
use std::{
    default,
    path::{Path, PathBuf},
};

use clap::{builder::Str, Parser, ValueEnum};
use madara_cli_common::{Prompt, PromptSelect};
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
            Prompt::new("Input DB path:")
                .default("./madara-fullnode-db")
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
    // l1_endpoint has to be set as environmental variable
}

pub enum MadaraRunnerParams {
    Devnet(MadaraRunnerConfigDevnet),
    Sequencer(MadaraRunnerConfigSequencer),
    FullNode(MadaraRunnerConfigFullNode),
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
            _ => panic!("Not supported yet"),
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
            Prompt::new("Input DB path:")
                .default("./madara-devnet-db")
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
            Prompt::new("Input DB path:")
                .default("./madara-sequencer-db")
                .ask()
        });

        let chain_config_path = self.chain_config_path.unwrap_or_else(|| {
            Prompt::new("Input chain config path:")
                .default("configs/presets/devnet.yaml")
                .ask()
        });

        Ok(MadaraRunnerConfigSequencer {
            base_path: Some(base_path),
            chain_config_path: Some(chain_config_path),
        })
    }
}

impl Default for MadaraRunnerConfigSequencer {
    fn default() -> Self {
        Self {
            base_path: Some("madara/data".to_string()),
            chain_config_path: Some("configs/presets/devnet.yaml".to_string()),
        }
    }
}
