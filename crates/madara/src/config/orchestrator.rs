use madara_cli_common::{
    logger,
    validation::{validate_u64, validate_url},
    Prompt,
};
use serde::{Deserialize, Serialize};

use super::global_config::Config;

const DEFAULT_ATLANTIC_URL: &str = "https://atlantic.api.herodotus.cloud";

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl OrchestratorConfiguration {
    pub fn init(template: &mut Config) -> anyhow::Result<()> {
        logger::new_empty_line();
        logger::note(
            "Orchestrator configuration",
            "You'll need to setup all the parameters related to Orchestrator",
        );
        let atlantic_url =
            Prompt::new("Enter the Atlantic prover URL (e.g., http://localhost:8080)")
                .default(&template.orchestrator.atlantic_service_url)
                .validate_interactively(validate_url)
                .ask();

        let maximum_block_to_process: Option<u64> =
            Prompt::new("Enter the maximum block to process (leave empty for no limit)")
                .allow_empty()
                .validate_interactively(validate_u64)
                .ask::<String>()
                .parse()
                .ok();

        template.orchestrator.atlantic_service_url = atlantic_url;
        template.orchestrator.maximum_block_to_process = maximum_block_to_process;

        Ok(())
    }
}
