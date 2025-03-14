use madara_cli_common::{
    logger,
    validation::{validate_eth_address, validate_u64, validate_url},
    Prompt,
};
use serde::{Deserialize, Serialize};

use super::global_config::Config;

const ANVIL_RPC_URL: &str = "http://anvil:8545";
const ANVIL_CHAIN_ID: u64 = 31337;

// This address will be valid as long as Mock Verifier contract is the first contract that is deploy in Anvil
const DEFAULT_MOCK_VERIFIER_ADDRESS: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl L1Configuration {
    pub fn init(template: &mut Config) -> anyhow::Result<()> {
        logger::new_empty_line();
        logger::note(
            "L1/Settlement layer configuration",
            "You'll need to setup all the parameters related to your L1 or settlement layer",
        );

        let eth_rpc = Prompt::new("Enter the L1 RPC URL (e.g., http://localhost:8545)")
            .default(&template.l1_config.eth_rpc)
            .validate_interactively(validate_url)
            .ask();
        let eth_chain_id = Prompt::new("Enter the L1 chain ID (e.g., 1 for Ethereum Mainnet)")
            .default(&template.l1_config.eth_chain_id.to_string())
            .validate_interactively(validate_u64)
            .ask::<u64>();
        let verifier_address = Prompt::new("Enter the Verifier contract address (e.g., 0x...)")
            .default(&template.l1_config.verifier_address)
            .validate_interactively(validate_eth_address)
            .ask();

        template.l1_config.eth_rpc = eth_rpc;
        template.l1_config.eth_chain_id = eth_chain_id;
        template.l1_config.verifier_address = verifier_address;

        Ok(())
    }
}
