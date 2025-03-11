use figment::{
    providers::{Format, Json},
    Figment,
};

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct BootstrapperConfiguration {
    pub eth_rpc: String,
    pub eth_priv_key: String,
    pub rollup_seq_url: String,
    pub rollup_declare_v0_seq_url: String,
    pub rollup_priv_key: String,
    pub eth_chain_id: u32,
    pub l1_deployer_address: String,
    pub l1_wait_time: String,
    pub sn_os_program_hash: String,
    pub config_hash_version: String,
    pub app_chain_id: String,
    pub fee_token_address: String,
    pub native_fee_token_address: String,
    pub cross_chain_wait_time: u32,
    pub l1_multisig_address: String,
    pub l2_multisig_address: String,
    pub verifier_address: String,
    pub operator_address: String,
    pub dev: bool,
    pub core_contract_address: String,
    pub core_contract_implementation_address: String,
    pub core_contract_mode: String,
    pub l1_eth_bridge_address: String,
}

impl BootstrapperConfiguration {
    pub fn load(file_path: &str) -> BootstrapperConfiguration {
        Figment::new()
            .merge(Json::file(file_path))
            .extract::<BootstrapperConfiguration>()
            .expect("Failed to load configuration")
    }

    pub fn save(&self, file_path: &str) {
        let json = serde_json::to_string_pretty(self)
            .expect("Failed to serialize bootstrapper configuration");
        fs::write(file_path, json).expect("Failed to write configuration");
    }
}
