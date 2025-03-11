// ... existing code ...

use figment::{
    providers::{Format, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;

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
