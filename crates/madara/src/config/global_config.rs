#![allow(unused)]
use anyhow::{Context, Error};
use clap::builder::Str;
use cliclack::Input;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use hex;
use madara_cli_common::{
    logger,
    validation::{
        validate_eth_address, validate_filename, validate_private_key, validate_time_with_unit,
        validate_u64, validate_url,
    },
    Prompt,
};
use std::{fs, path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::constants::DEFAULT_LOCAL_CONFIG_FILE;

use super::{
    constants::DEFAULT_ATLANTIC_URL,
    eth_wallet::EthWallet,
    l1_config::{self, L1Configuration},
    madara::MadaraConfiguration,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrchestratorConfiguration {
    pub atlantic_service_url: String,
    pub minimum_block_to_process: u64,
    pub maximum_block_to_process: Option<u64>,
}

impl Default for OrchestratorConfiguration {
    fn default() -> Self {
        Self {
            atlantic_service_url: DEFAULT_ATLANTIC_URL.to_string(),
            minimum_block_to_process: 1,
            maximum_block_to_process: Some(100),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub l1_config: L1Configuration,
    pub eth_wallet: EthWallet,
    pub madara: MadaraConfiguration,
    pub orchestrator: OrchestratorConfiguration,
}

impl Config {
    pub fn load(config_file: &str) -> Config {
        Figment::new()
            .merge(Toml::file(config_file))
            .extract::<Config>()
            .expect("Failed to load configuration")
    }

    pub fn save(&self, file_path: &str) {
        let toml = toml::to_string(self).expect("Failed to serialize global config to TOML");
        fs::write(file_path, toml).expect("Failed to write configuration");
    }

    pub fn init() -> anyhow::Result<()> {
        logger::new_empty_line();
        logger::intro("CLI Configuration File Initialization");

        let config_file_name: String =
            Prompt::new("Please enter the name for your configuration file")
                .default("my_custom_config.toml")
                .validate_interactively(validate_filename)
                .ask();
        let mut local_template = Config::load(DEFAULT_LOCAL_CONFIG_FILE);

        // L1 configuration
        L1Configuration::init(&mut local_template)?;

        // ETH Wallet configuration
        EthWallet::init(&mut local_template)?;

        // Madara configuration
        MadaraConfiguration::init(&mut local_template)?;

        // Orchestrator configuration
        logger::new_empty_line();
        logger::note(
            "Orchestrator configuration",
            "You'll need to setup all the parameters related to Orchestrator",
        );
        let atlantic_url =
            Prompt::new("Enter the Atlantic prover URL (e.g., http://localhost:8080)")
                .default(&local_template.orchestrator.atlantic_service_url)
                .validate_interactively(validate_url)
                .ask();

        let maximum_block_to_process: Option<u64> =
            Prompt::new("Enter the maximum block to process (leave empty for no limit)")
                .allow_empty()
                .validate_interactively(validate_u64)
                .ask::<String>()
                .parse()
                .ok();

        local_template.orchestrator.atlantic_service_url = atlantic_url;
        local_template.orchestrator.maximum_block_to_process = maximum_block_to_process;

        local_template.save(&format!("deps/data/{}", config_file_name));
        Ok(())
    }
}
