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
    #[arg(long, default_value = PathfinderNetwork::Custom.to_string().to_lowercase())]
    pub network: PathfinderNetwork,
    #[arg(long, default_value = DEFAULT_CHAIN_ID)]
    pub chain_id: String,
    pub ethereum_url_wss: Option<String>,
    #[arg(long, default_value = DEFAULT_GATEWAY_URL)]
    pub gateway_url: String,
    #[arg(long, default_value = DEFAULT_FEEDER_GATEWAY_URL)]
    pub feeder_gateway_url: String,
    #[arg(long, default_value = DEFAULT_HTTP_RPC)]
    pub http_rpc: String,
    #[arg(long, default_value = DEFAULT_DATA_DIRECTORY)]
    pub data_directory: String,
}

impl Default for PathfinderRunnerConfigMode {
    fn default() -> Self {
        Self {
            network: PathfinderNetwork::Custom,
            chain_id: DEFAULT_CHAIN_ID.to_string(),
            ethereum_url_wss: Some("RPC_API_KEY".to_string()),
            gateway_url: DEFAULT_GATEWAY_URL.to_string(),
            feeder_gateway_url: DEFAULT_FEEDER_GATEWAY_URL.to_string(),
            http_rpc: DEFAULT_HTTP_RPC.to_string(),
            data_directory: DEFAULT_DATA_DIRECTORY.to_string(),
        }
    }
}

impl PathfinderRunnerConfigMode {
    pub fn fill_values_with_prompt() -> anyhow::Result<PathfinderRunnerConfigMode> {
        let network = PromptSelect::new("Select network", PathfinderNetwork::iter()).ask();

        let chain_id = Prompt::new("Input chain-id")
            .default(DEFAULT_CHAIN_ID)
            .ask();

        // This params should be taken from a file. Keep this as placeholder in case that we want to change it in a future
        // let ethereum_url_wss

        let gateway_url = Prompt::new("Input gateway url")
            .default(DEFAULT_GATEWAY_URL)
            .ask();

        let feeder_gateway_url = Prompt::new("Input feeder gateway url")
            .default(DEFAULT_FEEDER_GATEWAY_URL)
            .ask();

        let http_rpc = Prompt::new("Input HTTP RPC URL")
            .default(DEFAULT_HTTP_RPC)
            .ask();

        let data_directory = Prompt::new("Input data directory")
            .default(DEFAULT_DATA_DIRECTORY)
            .ask();

        Ok(PathfinderRunnerConfigMode {
            network,
            chain_id,
            ethereum_url_wss: None,
            gateway_url,
            feeder_gateway_url,
            http_rpc,
            data_directory,
        })
    }
}
