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
    constants::{
        DEFAULT_ATLANTIC_URL, MADARA_APP_CHAIN_ID, MADARA_BLOCK_TIME, MADARA_CHAIN_NAME,
        MADARA_LATEST_PROTOCOL_VERSION, MADARA_NATIVE_FEE_TOKEN_ADDRESS,
        MADARA_PARENT_FEE_TOKEN_ADDRESS, MADARA_PENDING_BLOCK_UPDATE_TIME,
    },
    eth_wallet::EthWallet,
    l1_config::{self, L1Configuration},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MadaraConfiguration {
    pub chain_name: String,
    pub app_chain_id: String,
    pub native_fee_token_address: String,
    pub parent_fee_token_address: String,
    pub latest_protocol_version: String,
    pub block_time: String,
    pub pending_block_update_time: String,
    pub gas_price: u64,
    pub blob_gas_price: u64,
}

impl Default for MadaraConfiguration {
    fn default() -> Self {
        Self {
            chain_name: MADARA_CHAIN_NAME.to_string(),
            app_chain_id: MADARA_APP_CHAIN_ID.to_string(),
            native_fee_token_address: MADARA_NATIVE_FEE_TOKEN_ADDRESS.to_string(),
            parent_fee_token_address: MADARA_PARENT_FEE_TOKEN_ADDRESS.to_string(),
            latest_protocol_version: MADARA_LATEST_PROTOCOL_VERSION.to_string(),
            block_time: MADARA_BLOCK_TIME.to_string(),
            pending_block_update_time: MADARA_PENDING_BLOCK_UPDATE_TIME.to_string(),
            gas_price: 0,
            blob_gas_price: 0,
        }
    }
}

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
        logger::new_empty_line();
        logger::note(
            "Madara configuration",
            "You'll need to setup all the parameters related to Madara",
        );
        let chain_name = Prompt::new("Enter the name of the Madara chain (e.g., MyChain)")
            .default(&local_template.madara.chain_name)
            .ask();
        let app_chain_id = Prompt::new("Enter the Madara chain ID (e.g., MADARA_DEVNET)")
            .default(&&local_template.madara.app_chain_id)
            .ask();
        let block_time = Prompt::new("Enter the block time for Madara (in seconds, e.g., 15s)")
            .default(&local_template.madara.block_time)
            .validate_interactively(validate_time_with_unit)
            .ask();
        let gas_price = Prompt::new("Enter the gas price for Madara (in Gwei, e.g., 20)")
            .default(&local_template.madara.gas_price.to_string())
            .validate_interactively(validate_u64)
            .ask::<u64>();

        let blob_gas_price = Prompt::new("Enter the blob gas price for Madara (in Gwei, e.g., 5)")
            .default(&local_template.madara.blob_gas_price.to_string())
            .validate_interactively(validate_u64)
            .ask::<u64>();

        local_template.madara.chain_name = chain_name;
        local_template.madara.app_chain_id = app_chain_id;
        local_template.madara.block_time = block_time;
        local_template.madara.gas_price = gas_price;
        local_template.madara.blob_gas_price = blob_gas_price;

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
