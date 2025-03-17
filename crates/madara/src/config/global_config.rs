use std::fs;

use figment::{
    providers::{Format, Toml},
    Figment,
};
use madara_cli_common::{logger, validation::validate_filename, Prompt};

use serde::{Deserialize, Serialize};

use crate::constants::DEFAULT_LOCAL_CONFIG_FILE;

use super::{
    eth_wallet::EthWallet, l1_config::L1Configuration, madara::MadaraConfiguration,
    orchestrator::OrchestratorConfiguration,
};

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

    pub fn init(default: bool) -> anyhow::Result<()> {
        logger::new_empty_line();
        logger::intro("CLI Configuration File Initialization");

        let config_file_name: String =
            Prompt::new("Please enter the name for your configuration file")
                .default("my_custom_config.toml")
                .validate_interactively(validate_filename)
                .default_or_ask(default);
        let mut local_template = Config::load(DEFAULT_LOCAL_CONFIG_FILE);

        println!("CONFIG FILE NAME: {}", config_file_name);

        // L1 configuration
        L1Configuration::init(&mut local_template, default)?;

        // ETH Wallet configuration
        EthWallet::init(&mut local_template, default)?;

        // Madara configuration
        MadaraConfiguration::init(&mut local_template, default)?;

        // Orchestrator configuration
        OrchestratorConfiguration::init(&mut local_template, default)?;

        local_template.save(&format!("deps/data/{}", config_file_name));
        Ok(())
    }
}
