#![allow(unused)]
use std::{
    default,
    path::{Path, PathBuf},
};

use clap::{Parser, ValueEnum};
use cliclack::Confirm;
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
    pub rpc_port: Option<u16>,
    pub rpc_cors: Option<String>,
    pub rpc_external: Option<bool>,
}

impl MadaraRunnerConfigFullNode {
    pub fn fill_values_with_prompt(mut self) -> anyhow::Result<MadaraRunnerConfigFullNode> {
        let base_path = self
            .base_path
            .unwrap_or_else(|| Prompt::new("Input DB path:").default("./madara-db").ask());

        let network = self
            .network
            .unwrap_or_else(|| PromptSelect::new("Select Network:", MadaraNetwork::iter()).ask());

        let rpc_port = self
            .rpc_port
            .unwrap_or_else(|| Prompt::new("Input RPC port:").default("9999").ask());

        let rpc_cors = self.rpc_cors.unwrap_or_else(|| {
            Prompt::new("Input RPC CORS allowed origins:")
                .default("all")
                .ask()
        });

        let rpc_external = self.rpc_external.unwrap_or_else(|| {
            Confirm::new("Run RPC externally (on 0.0.0.0):")
                .interact()
                .unwrap()
        });

        Ok(MadaraRunnerConfigFullNode {
            base_path: Some(base_path),
            network: Some(network),
            rpc_port: Some(rpc_port),
            rpc_cors: Some(rpc_cors),
            rpc_external: Some(rpc_external),
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
    pub preset: Option<MadaraPreset>,
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
        let base_path = self
            .base_path
            .unwrap_or_else(|| Prompt::new("Input DB path:").default("./madara-db").ask());

        Ok(MadaraRunnerConfigDevnet {
            base_path: Some(base_path),
        })
    }
}

impl MadaraRunnerConfigSequencer {
    pub fn fill_values_with_prompt(mut self) -> anyhow::Result<MadaraRunnerConfigSequencer> {
        let base_path = self
            .base_path
            .unwrap_or_else(|| Prompt::new("Input DB path:").default("./madara-db").ask());

        let preset = self.preset.unwrap_or_else(|| {
            let preset_type = PromptSelect::new("Select preset:", MadaraPresetType::iter()).ask();
            let path = if preset_type == MadaraPresetType::Custom {
                Some(
                    Prompt::new("Select preset file path")
                        .default(MADARA_PRESETS_PATH)
                        .ask(),
                )
            } else {
                None
            };

            MadaraPreset { preset_type, path }
        });

        Ok(MadaraRunnerConfigSequencer {
            base_path: Some(base_path),
            preset: Some(preset),
        })
    }
}

impl Default for MadaraRunnerConfigSequencer {
    fn default() -> Self {
        Self {
            base_path: None,
            preset: None,
        }
    }
}
