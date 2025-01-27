use anyhow::Context;
use madara_cli_common::{cmd::Cmd, logger, Prompt, PromptSelect};
use xshell::{cmd, Shell};

use crate::commands;

#[derive(Debug, Default)]
pub struct OrchestratorRunnerConfig {
    pub aws_access_key_id: Option<String>,
    pub aws_secret_access_key: Option<String>,
    pub aws_region: Option<String>,
    pub rpc_for_snos: Option<String>,
    pub madara_rpc_url: Option<String>,
    pub aws_enabled: bool,
    pub settle_on_starknet: bool,
    pub starknet_rpc_url: Option<String>,
    pub starknet_private_key: Option<String>,
    pub ethereum_private_key: Option<String>,
    pub l1_core_contract_address: Option<String>,
    pub starknet_operator_address: Option<String>,
    pub starknet_account_address: Option<String>,
    pub starknet_cairo_core_contract_address: Option<String>,
    pub starknet_finality_retry_wait_in_secs: Option<u64>,
    pub ethereum_rpc_url: Option<String>,
    pub aws_s3_enabled: bool,
    pub aws_sqs_enabled: bool,
    pub queue_base_url: Option<String>,
    pub aws_sns_enabled: bool,
    pub sharp_enabled: Option<bool>,
    pub sharp_customer_id: Option<u64>,
    pub sharp_url: Option<String>,
    pub sharp_user_crt: Option<String>,
    pub sharp_user_key: Option<String>,
    pub sharp_server_crt: Option<String>,
    pub gps_verifier_contract_address: Option<String>,
    pub sharp_rpc_node_url: Option<String>,
    pub da_on_ethereum: Option<bool>,
    pub ethereum_da_rpc_url: Option<String>,
    pub mongodb_enabled: Option<bool>,
    pub mongodb_connection_url: Option<String>,
}

impl OrchestratorRunnerConfig {
    pub fn fill_values_with_prompt(mut self) -> anyhow::Result<OrchestratorRunnerConfig> {
        self.aws_access_key_id = self
            .aws_access_key_id
            .or_else(|| Some(Prompt::new("AWS Access Key ID").default("test").ask()));

        self.aws_secret_access_key = self
            .aws_secret_access_key
            .or_else(|| Some(Prompt::new("AWS Secret Access Key").default("test").ask()));

        self.aws_region = self
            .aws_region
            .or_else(|| Some(Prompt::new("AWS Region").default("us-east-1").ask()));

        self.rpc_for_snos = self
            .rpc_for_snos
            .or_else(|| Some(Prompt::new("RPC for SNOS").default("http://test.com").ask()));

        self.madara_rpc_url = self.madara_rpc_url.or_else(|| {
            Some(
                Prompt::new("Madara RPC URL")
                    .default("http://test.com")
                    .ask(),
            )
        });

        self.starknet_rpc_url = self.starknet_rpc_url.or_else(|| {
            Some(
                Prompt::new("Starknet RPC URL")
                    .default("https://starknet-sepolia.public.blastapi.io")
                    .ask(),
            )
        });

        self.starknet_private_key = self
            .starknet_private_key
            .or_else(|| Some(Prompt::new("Starknet Private Key").default("1").ask()));

        self.ethereum_private_key = self
            .ethereum_private_key
            .or_else(|| Some(Prompt::new("Ethereum Private Key").default("1").ask()));

        self.l1_core_contract_address = self
            .l1_core_contract_address
            .or_else(|| Some(Prompt::new("L1 Core Contract Address").default("0x1").ask()));

        self.starknet_operator_address = self.starknet_operator_address.or_else(|| {
            Some(
                Prompt::new("Starknet Operator Address")
                    .default("0x1")
                    .ask(),
            )
        });

        self.starknet_account_address = self
            .starknet_account_address
            .or_else(|| Some(Prompt::new("Starknet Account Address").default("0x1").ask()));

        self.starknet_cairo_core_contract_address =
            self.starknet_cairo_core_contract_address.or_else(|| {
                Some(
                    Prompt::new("Starknet Cairo Core Contract Address")
                        .default("0x1")
                        .ask(),
                )
            });

        self.starknet_finality_retry_wait_in_secs =
            self.starknet_finality_retry_wait_in_secs.or_else(|| {
                Some(
                    Prompt::new("Starknet Finality Retry Wait In Seconds")
                        .default("10")
                        .ask::<String>()
                        .parse::<u64>()
                        .unwrap(),
                )
            });

        self.ethereum_rpc_url = self.ethereum_rpc_url.or_else(|| {
            Some(
                Prompt::new("Ethereum RPC URL")
                    .default("http://test.com")
                    .ask(),
            )
        });

        self.queue_base_url = self.queue_base_url.or_else(|| {
            Some(
                Prompt::new("Queue Base URL")
                    .default("http://test.com")
                    .ask(),
            )
        });

        self.sharp_enabled = self.sharp_enabled.or_else(|| {
            let enable_sharp = PromptSelect::new("Enable Sharp?", vec!["Yes", "No"]).ask();
            Some(enable_sharp == "Yes")
        });

        self.sharp_customer_id = self.sharp_customer_id.or_else(|| {
            Some(
                Prompt::new("Sharp Customer ID")
                    .default("1")
                    .ask::<String>()
                    .parse::<u64>()
                    .unwrap(),
            )
        });

        self.sharp_url = self
            .sharp_url
            .or_else(|| Some(Prompt::new("Sharp URL").default("http://test.com").ask()));

        self.sharp_user_crt = self
            .sharp_user_crt
            .or_else(|| Some(Prompt::new("Sharp User CRT").ask()));

        self.sharp_user_key = self
            .sharp_user_key
            .or_else(|| Some(Prompt::new("Sharp User Key").ask()));

        self.sharp_server_crt = self
            .sharp_server_crt
            .or_else(|| Some(Prompt::new("Sharp Server CRT").ask()));

        self.gps_verifier_contract_address = self.gps_verifier_contract_address.or_else(|| {
            Some(
                Prompt::new("GPS Verifier Contract Address")
                    .default("0x07ec0D28e50322Eb0C159B9090ecF3aeA8346DFe")
                    .ask(),
            )
        });

        self.sharp_rpc_node_url = self.sharp_rpc_node_url.or_else(|| {
            Some(
                Prompt::new("Sharp RPC Node URL")
                    .default("http://test.com")
                    .ask(),
            )
        });

        self.da_on_ethereum = self.da_on_ethereum.or_else(|| {
            let enable_da = PromptSelect::new("Enable DA on Ethereum?", vec!["Yes", "No"]).ask();
            Some(enable_da == "Yes")
        });

        self.ethereum_da_rpc_url = self.ethereum_da_rpc_url.or_else(|| {
            Some(
                Prompt::new("Ethereum DA RPC URL")
                    .default("http://test.com")
                    .ask(),
            )
        });

        self.mongodb_enabled = self.mongodb_enabled.or_else(|| {
            let enable_mongodb = PromptSelect::new("Enable MongoDB?", vec!["Yes", "No"]).ask();
            Some(enable_mongodb == "Yes")
        });

        self.mongodb_connection_url = self.mongodb_connection_url.or_else(|| {
            Some(
                Prompt::new("MongoDB Connection URL")
                    .default("mongodb://localhost:27017")
                    .ask(),
            )
        });

        Ok(self)
    }
    pub fn spawn(&self, shell: &Shell) -> anyhow::Result<()> {
        let mut params = String::new();

        // Helper function to add parameters to the `params` string
        fn add_param(key: &str, value: &Option<String>, params: &mut String) {
            if let Some(v) = value {
                params.push_str(&format!("--{} {}\n", key, v));
            }
        }

        // Add each field from the config to the params string
        add_param("aws-access-key-id", &self.aws_access_key_id, &mut params);
        add_param("aws-secret-access-key", &self.aws_secret_access_key, &mut params);
        add_param("aws-region", &self.aws_region, &mut params);
        add_param("rpc-for-snos", &self.rpc_for_snos, &mut params);
        add_param("madara-rpc-url", &self.madara_rpc_url, &mut params);
        add_param("starknet-rpc-url", &self.starknet_rpc_url, &mut params);
        add_param("starknet-private-key", &self.starknet_private_key, &mut params);
        add_param("ethereum-private-key", &self.ethereum_private_key, &mut params);
        add_param("l1-core-contract-address", &self.l1_core_contract_address, &mut params);
        add_param("starknet-operator-address", &self.starknet_operator_address, &mut params);
        add_param("starknet-account-address", &self.starknet_account_address, &mut params);
        add_param(
            "starknet-cairo-core-contract-address",
            &self.starknet_cairo_core_contract_address,
            &mut params,
        );

        if let Some(retry) = self.starknet_finality_retry_wait_in_secs {
            params.push_str(&format!(
                "--starknet-finality-retry-wait-in-secs {}\n",
                retry
            ));
        }

        add_param("ethereum-rpc-url", &self.ethereum_rpc_url, &mut params);
        if self.aws_s3_enabled {
            params.push_str("--aws-s3-enabled true\n");
        }
        if self.aws_sqs_enabled {
            params.push_str("--aws-sqs-enabled true\n");
        }
        add_param("queue-base-url", &self.queue_base_url, &mut params);
        if self.aws_sns_enabled {
            params.push_str("--aws-sns-enabled true\n");
        }
        if self.sharp_enabled.is_some() {
            params.push_str("--sharp-enabled true\n");
        }
        add_param(
            "sharp-customer-id",
            &self.sharp_customer_id.map(|v| v.to_string()),
            &mut params,
        );
        add_param("sharp-url", &self.sharp_url, &mut params);
        add_param("sharp-user-crt", &self.sharp_user_crt, &mut params);
        add_param("sharp-user-key", &self.sharp_user_key, &mut params);
        add_param("sharp-server-crt", &self.sharp_server_crt, &mut params);
        add_param(
            "gps-verifier-contract-address",
            &self.gps_verifier_contract_address,
            &mut params,
        );
        add_param("sharp-rpc-node-url", &self.sharp_rpc_node_url, &mut params);

        if self.da_on_ethereum.is_some() {
            params.push_str("--da-on-ethereum true\n");
        }
        add_param("ethereum-da-rpc-url", &self.ethereum_da_rpc_url, &mut params);
        if self.mongodb_enabled.is_some() {
            params.push_str("--mongodb-enabled true\n");
        }
        add_param("mongodb-connection-url", &self.mongodb_connection_url, &mut params);

        // Spawn the command using `Cmd`
        let command_string = format!("cargo run -p orchestrator run {}", params.trim());

        // Use the `shell` object to execute the command
        Cmd::new(cmd!(shell, "{command_string}"))
            .run()
            .context("Failed to run the orchestrator command")?;

        Ok(())
    }
}

pub fn run(_shell: &Shell) -> anyhow::Result<()> {
    logger::new_empty_line();
    logger::intro();

    let services: String = vec!["Madara", "SNOS", "Prover", "Pathfinder", "Anvil"]
        .iter()
        .map(|arg| format!("  ✅ {}", arg)) // You can replace "✅" with other emojis like "☑️" or custom checkboxes
        .collect::<Vec<_>>()
        .join("\n");
    logger::note("AppChain configuration", services);

    let shell = Shell::new().unwrap();

    // Start Madara with default configuration
    commands::madara::run(Default::default(), &shell)?;

    let args_orcherstrator = OrchestratorRunnerConfig::default().fill_values_with_prompt()?;

    // Spin up all the necessary services
    args_orcherstrator.spawn(&shell)?;

    Ok(())
}
