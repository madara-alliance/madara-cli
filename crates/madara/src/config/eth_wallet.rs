use madara_cli_common::{
    logger,
    validation::{validate_eth_address, validate_private_key},
    Prompt,
};
use serde::{Deserialize, Serialize};

use super::global_config::Config;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

#[allow(dead_code)]
pub struct EthKeys {
    //Private key (32 bytes)
    pub private_key: String,
    //Secp256k1 public key (64 bytes)
    pub public_key: PublicKey,
    //Eth address (20 bytes)
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthWallet {
    pub eth_priv_key: String,
    pub l1_deployer_address: String,
    pub l1_operator_address: String,
    pub l1_multisig_address: String,
}

impl Default for EthWallet {
    fn default() -> Self {
        Self::new(
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
            "0x70997970c51812dc3a010c7d01b50e0d17dc79c8".to_string(),
        )
    }
}

impl EthWallet {
    pub fn new(eth_priv_key: String, l1_multisig_address: String) -> Self {
        let (_, l1_deployer_address) = Self::get_address(&eth_priv_key);
        let l1_operator_address = l1_deployer_address.clone();

        assert_ne!(
            l1_multisig_address, l1_deployer_address,
            "Expected l1_multisig_address ({}) to be different from l1_deployer_address ({})",
            l1_multisig_address, l1_deployer_address
        );
        // By default operator_address == l1_deployer_address (same account)
        Self {
            eth_priv_key,
            l1_deployer_address,
            l1_operator_address,
            l1_multisig_address,
        }
    }

    #[allow(dead_code)]
    pub fn get_keys(index: usize) -> EthKeys {
        // Default Anvil private keys
        let private_key = match index {
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
        };

        let (public_key, address) = Self::get_address(&private_key);

        EthKeys {
            private_key,
            public_key,
            address,
        }
    }

    pub fn get_address(priv_key: &String) -> (PublicKey, String) {
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
        let address = format!("0x{}", hex::encode(&hash[12..]));

        (public_key, address)
    }

    pub fn init(template: &mut Config) -> anyhow::Result<()> {
        logger::new_empty_line();
        logger::note(
            "L1 wallet configuration",
            "You'll need to setup all the parameters related to your L1 or settlement layer wallet",
        );
        let eth_priv_key = Prompt::new("Enter the L1 private key (e.g., 0x...)")
            .default(&template.eth_wallet.eth_priv_key)
            .validate_interactively(validate_private_key)
            .ask();
        let l1_deployer_address = Prompt::new("Enter the L1 deployer address (e.g., 0x...)")
            .default(&&template.eth_wallet.l1_deployer_address)
            .validate_interactively(validate_eth_address)
            .ask();
        let l1_operator_address = Prompt::new("Enter the L1 operator address (e.g., 0x...)")
            .default(&template.eth_wallet.l1_operator_address)
            .validate_interactively(validate_eth_address)
            .ask();
        let l1_multisig_address = Prompt::new("Enter the L1 multisig address (e.g., 0x...)")
            .default(&template.eth_wallet.l1_multisig_address)
            .validate_interactively(validate_eth_address)
            .ask();

        template.eth_wallet.eth_priv_key = eth_priv_key;
        template.eth_wallet.l1_deployer_address = l1_deployer_address;
        template.eth_wallet.l1_operator_address = l1_operator_address;
        template.eth_wallet.l1_multisig_address = l1_multisig_address;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addresses() {
        // Default Anvil accounts
        let expected_addresses = vec![
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

        for (index, expected_address) in expected_addresses.iter().enumerate() {
            let key = EthWallet::get_keys(index);
            let (_, address) = EthWallet::get_address(&key.private_key);
            assert_eq!(address, *expected_address);
        }
    }
}
