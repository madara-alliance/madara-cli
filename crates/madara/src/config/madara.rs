// ... existing code ...

use figment::{
    providers::{Format, Yaml},
    Figment,
};
use madara_cli_common::{
    logger,
    validation::{validate_time_with_unit, validate_u64},
    Prompt,
};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;

use super::global_config::Config;

const MADARA_CHAIN_NAME: &str = "Madara";
const MADARA_APP_CHAIN_ID: &str = "MADARA_DEVNET";
const MADARA_NATIVE_FEE_TOKEN_ADDRESS: &str =
    "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d";
const MADARA_PARENT_FEE_TOKEN_ADDRESS: &str =
    "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
const MADARA_LATEST_PROTOCOL_VERSION: &str = "0.13.2";
const MADARA_BLOCK_TIME: &str = "10s";
const MADARA_PENDING_BLOCK_UPDATE_TIME: &str = "2s";
#[derive(Debug, Serialize, Deserialize)]
pub struct MadaraPresetConfiguration {
    pub chain_name: String,
    pub chain_id: String,
    pub feeder_gateway_url: String,
    pub gateway_url: String,
    pub native_fee_token_address: String,
    pub parent_fee_token_address: String,
    pub latest_protocol_version: String,
    pub block_time: String,
    pub pending_block_update_time: String,
    pub execution_batch_size: u32,
    pub bouncer_config: BouncerConfig,
    pub sequencer_address: String,
    pub eth_core_contract_address: String,
    pub eth_gps_statement_verifier: String,
    pub mempool_tx_limit: u32,
    pub mempool_declare_tx_limit: u32,
    pub mempool_tx_max_age: Option<u32>, // Assuming this can be null
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BouncerConfig {
    pub block_max_capacity: BlockMaxCapacity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockMaxCapacity {
    pub builtin_count: BuiltinCount,
    pub gas: u64,
    pub n_steps: u64,
    pub message_segment_length: u64,
    pub n_events: u64,
    pub state_diff_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuiltinCount {
    pub add_mod: u64,
    pub bitwise: u64,
    pub ecdsa: u64,
    pub ec_op: u64,
    pub keccak: u64,
    pub mul_mod: u64,
    pub pedersen: u64,
    pub poseidon: u64,
    pub range_check: u64,
    pub range_check96: u64,
}

impl MadaraPresetConfiguration {
    pub fn load(file_path: &str) -> MadaraPresetConfiguration {
        Figment::new()
            .merge(Yaml::file(file_path))
            .extract::<MadaraPresetConfiguration>()
            .expect("Failed to load configuration")
    }

    pub fn save(&self, file_path: &str) {
        let yaml = serde_yaml::to_string(self).expect("Failed to serialize Madara configuration");
        fs::write(file_path, yaml).expect("Failed to write configuration");
    }
}

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

impl MadaraConfiguration {
    pub fn init(template: &mut Config) -> anyhow::Result<()> {
        logger::new_empty_line();
        logger::note(
            "Madara configuration",
            "You'll need to setup all the parameters related to Madara",
        );
        let chain_name = Prompt::new("Enter the name of the Madara chain (e.g., MyChain)")
            .default(&template.madara.chain_name)
            .ask();
        let app_chain_id = Prompt::new("Enter the Madara chain ID (e.g., MADARA_DEVNET)")
            .default(&&template.madara.app_chain_id)
            .ask();
        let block_time = Prompt::new("Enter the block time for Madara (in seconds, e.g., 15s)")
            .default(&template.madara.block_time)
            .validate_interactively(validate_time_with_unit)
            .ask();
        let gas_price = Prompt::new("Enter the gas price for Madara (in Gwei, e.g., 20)")
            .default(&template.madara.gas_price.to_string())
            .validate_interactively(validate_u64)
            .ask::<u64>();

        let blob_gas_price = Prompt::new("Enter the blob gas price for Madara (in Gwei, e.g., 5)")
            .default(&template.madara.blob_gas_price.to_string())
            .validate_interactively(validate_u64)
            .ask::<u64>();

        template.madara.chain_name = chain_name;
        template.madara.app_chain_id = app_chain_id;
        template.madara.block_time = block_time;
        template.madara.gas_price = gas_price;
        template.madara.blob_gas_price = blob_gas_price;
        Ok(())
    }
}
