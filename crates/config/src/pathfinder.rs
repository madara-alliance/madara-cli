use clap::{Args, ValueEnum};
use madara_cli_common::{Prompt, PromptSelect};
use strum::{EnumIter, IntoEnumIterator};

const DEFAULT_CHAIN_ID: &str = "MADARA_DEVNET";
const DEFAULT_GATEWAY_URL: &str = "http://madara:8080/gateway";
const DEFAULT_FEEDER_GATEWAY_URL: &str = "http://madara:8080/feeder_gateway";
const DEFAULT_HTTP_RPC: &str = "0.0.0.0:9545";
const DEFAULT_DATA_DIRECTORY: &str = "/usr/share/pathfinder/data";

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, Default, EnumIter, strum::Display)]
pub enum PathfinderNetwork {
    #[default]
    Custom,
    Sepolia,
    Mainnet,
}

#[derive(Debug, Clone, Args)]
pub struct PathfinderRunnerConfigMode {
    pub network: Option<PathfinderNetwork>,
    pub chain_id: Option<String>,
    pub ethereum_url_wss: Option<String>,
    pub gateway_url: Option<String>,
    pub feeder_gateway_url: Option<String>,
    pub http_rpc: Option<String>,
    pub data_directory: Option<String>,
}

impl Default for PathfinderRunnerConfigMode {
    fn default() -> Self {
        Self {
            network: Some(PathfinderNetwork::Custom),
            chain_id: Some(DEFAULT_CHAIN_ID.to_string()),
            ethereum_url_wss: Some("RPC_API_KEY".to_string()),
            gateway_url: Some(DEFAULT_GATEWAY_URL.to_string()),
            feeder_gateway_url: Some(DEFAULT_FEEDER_GATEWAY_URL.to_string()),
            http_rpc: Some(DEFAULT_HTTP_RPC.to_string()),
            data_directory: Some(DEFAULT_DATA_DIRECTORY.to_string()),
        }
    }
}

impl PathfinderRunnerConfigMode {
    pub fn fill_values_with_prompt(self) -> anyhow::Result<PathfinderRunnerConfigMode> {
        let network = self.network.unwrap_or_else(|| {
            PromptSelect::new("Select network", PathfinderNetwork::iter()).ask()
        });

        let chain_id = self.chain_id.unwrap_or_else(|| {
            Prompt::new("Input chain-id")
                .default(DEFAULT_CHAIN_ID)
                .ask()
        });

        // This params should be taken from a file. Keep this as placeholder in case that we want to change it in a future
        // let ethereum_url_wss

        let gateway_url = self.gateway_url.unwrap_or_else(|| {
            Prompt::new("Input gateway url")
                .default(DEFAULT_GATEWAY_URL)
                .ask()
        });

        let feeder_gateway_url = self.feeder_gateway_url.unwrap_or_else(|| {
            Prompt::new("Input feeder gateway url")
                .default(DEFAULT_FEEDER_GATEWAY_URL)
                .ask()
        });

        let http_rpc = self.http_rpc.unwrap_or_else(|| {
            Prompt::new("Input HTTP RPC URL")
                .default(DEFAULT_HTTP_RPC)
                .ask()
        });

        let data_directory = self.data_directory.unwrap_or_else(|| {
            Prompt::new("Input data directory")
                .default(DEFAULT_DATA_DIRECTORY)
                .ask()
        });

        Ok(PathfinderRunnerConfigMode {
            network: Some(network),
            chain_id: Some(chain_id),
            ethereum_url_wss: None,
            gateway_url: Some(gateway_url),
            feeder_gateway_url: Some(feeder_gateway_url),
            http_rpc: Some(http_rpc),
            data_directory: Some(data_directory),
        })
    }

    pub fn unwrap_all(&self) -> (PathfinderNetwork, String, String, String, String, String) {
        (
            self.network.clone().expect("network is None"),
            self.chain_id.clone().expect("chain_id is None"),
            self.gateway_url.clone().expect("gateway_url is None"),
            self.feeder_gateway_url
                .clone()
                .expect("feeder_gateway_url is None"),
            self.http_rpc.clone().expect("http_rpc is None"),
            self.data_directory.clone().expect("data_directory is None"),
        )
    }
}
