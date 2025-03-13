#![allow(unused)]
use anyhow::{Context, Error};
use hex;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

use figment::{
    providers::{Format, Toml},
    Figment,
};

use serde::Deserialize;

use super::constants::{
    ANVIL_CHAIN_ID, ANVIL_RPC_URL, DEFAULT_ATLANTIC_URL, DEFAULT_MOCK_VERIFIER_ADDRESS,
    MADARA_APP_CHAIN_ID, MADARA_BLOCK_TIME, MADARA_CHAIN_NAME, MADARA_LATEST_PROTOCOL_VERSION,
    MADARA_NATIVE_FEE_TOKEN_ADDRESS, MADARA_PARENT_FEE_TOKEN_ADDRESS,
    MADARA_PENDING_BLOCK_UPDATE_TIME,
};

#[derive(Debug, Deserialize, Clone)]
pub struct EthWallet {
    pub eth_priv_key: String,
    pub l1_deployer_address: String,
    pub operator_address: String,
    pub l1_multisig_address: String,
}

impl Default for EthWallet {
    fn default() -> Self {
        Self::new(
            Self::get_priv_key(0),
            "0x70997970c51812dc3a010c7d01b50e0d17dc79c8".to_string(),
        )
    }
}

impl EthWallet {
    pub fn new(eth_priv_key: String, l1_multisig_address: String) -> Self {
        let l1_deployer_address = Self::get_address(&eth_priv_key);
        let operator_address = l1_deployer_address.clone();

        assert_ne!(
            l1_multisig_address, l1_deployer_address,
            "Expected l1_multisig_address ({}) to be different from l1_deployer_address ({})",
            l1_multisig_address, l1_deployer_address
        );
        // By default operator_address == l1_deployer_address (same account)
        Self {
            eth_priv_key,
            l1_deployer_address,
            operator_address,
            l1_multisig_address,
        }
    }

    pub fn get_priv_key(index: usize) -> String {
        // Default Anvil private keys
        match index {
            0 => String::from("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"),
            1 => String::from("0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d"),
            2 => String::from("0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a"),
            3 => String::from("0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6"),
            4 => String::from("0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a"),
            5 => String::from("0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba"),
            6 => String::from("0x92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e"),
            7 => String::from("0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356"),
            8 => String::from("0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97"),
            9 => String::from("0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6"),
            _ => String::from("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"), // Default case
        }
    }

    pub fn get_address(priv_key: &String) -> String {
        let secp = Secp256k1::signing_only();
        let priv_key = priv_key.trim_start_matches("0x");

        // Convert the private key from a hex string to a SecretKey object
        let priv_key_bytes = hex::decode(priv_key).expect("Invalid hex string");
        let secret_key = SecretKey::from_slice(&priv_key_bytes).expect("Invalid private key");

        // Generate the public key from the private key
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let uncompressed_pubkey_serialized = public_key.serialize_uncompressed();

        // Hash the public key using Keccak-256 (Ethereum address derivation)
        let mut hasher = Keccak256::new();
        // Skip the first byte (0x04) since it's the uncompressed pubkey identifier
        hasher.update(&uncompressed_pubkey_serialized[1..]);
        let hash = hasher.finalize();

        // Take the last 20 bytes of the hash to form the Ethereum address
        format!("0x{}", hex::encode(&hash[12..]))
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct L1Configuration {
    pub eth_rpc: String,
    pub eth_chain_id: u64,
    pub verifier_address: String,
}

impl Default for L1Configuration {
    fn default() -> Self {
        Self {
            eth_rpc: ANVIL_RPC_URL.to_string(),
            eth_chain_id: ANVIL_CHAIN_ID,
            verifier_address: DEFAULT_MOCK_VERIFIER_ADDRESS.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Config {
    pub l1_config: L1Configuration,
    pub eth_wallet: EthWallet,
    pub madara: MadaraConfiguration,
    pub orchestrator: OrchestratorConfiguration,
}

pub fn load_config(config_file: &String) -> Config {
    Figment::new()
        .merge(Toml::file(config_file))
        .extract::<Config>()
        .expect("Failed to load configuration")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pub_keys() {
        // Default Anvil accounts
        let expected_pub_keys = vec![
            "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
            "0x70997970c51812dc3a010c7d01b50e0d17dc79c8",
            "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc",
            "0x90f79bf6eb2c4f870365e785982e1f101e93b906",
            "0x15d34aaf54267db7d7c367839aaf71a00a2c6a65",
            "0x9965507d1a55bcc2695c58ba16fb37d819b0a4dc",
            "0x976ea74026e726554db657fa54763abd0c3a0aa9",
            "0x14dc79964da2c08b23698b3d3cc7ca32193d9955",
            "0x23618e81e3f5cdf7f54c3d65f7fbc0abf5b21e8f",
            "0xa0ee7a142d267c1f36714e4a8f75612f20a79720",
        ];

        for (index, expected_pub_key) in expected_pub_keys.iter().enumerate() {
            let priv_key = EthWallet::get_priv_key(index);
            let pub_key = EthWallet::get_address(&priv_key);
            assert_eq!(pub_key, *expected_pub_key);
        }
    }
}
