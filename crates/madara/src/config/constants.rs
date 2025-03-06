pub const ANVIL_RPC_URL: &str = "http://anvil:8545";
pub const ANVIL_CHAIN_ID: u64 = 31337;

#[allow(unused)]
pub const SEPOLIA_RPC_URL: &str = "https://ethereum-sepolia-rpc.publicnode.com";
#[allow(unused)]
pub const SEPOLIA_CHAIN_ID: u64 = 11155111;

// This address will be valid as long as Mock Verifier contract is the first contract that is deploy in Anvil
pub const DEFAULT_MOCK_VERIFIER_ADDRESS: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

pub const MADARA_CHAIN_NAME: &str = "Madara";
pub const MADARA_APP_CHAIN_ID: &str = "MADARA_DEVNET";
pub const MADARA_NATIVE_FEE_TOKEN_ADDRESS: &str =
    "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d";
pub const MADARA_PARENT_FEE_TOKEN_ADDRESS: &str =
    "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
pub const MADARA_LATEST_PROTOCOL_VERSION: &str = "0.13.2";
pub const MADARA_BLOCK_TIME: &str = "10s";
pub const MADARA_PENDING_BLOCK_UPDATE_TIME: &str = "2s";

pub const DEFAULT_ATLANTIC_URL: &str = "https://atlantic.api.herodotus.cloud";
