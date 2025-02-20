use clap::Parser;
use madara_cli_common::PromptConfirm;

#[derive(Debug, Parser, Clone)]
pub struct BootstrapperConfig {
    #[arg(short, long, default_value = "true")]
    pub deploy_l2_contracts: bool,
}

impl BootstrapperConfig {
    /// Fill AppChain configuration using interactive prompts
    pub fn fill_values_with_prompt() -> anyhow::Result<BootstrapperConfig> {
        let deploy_l2_contracts = PromptConfirm::new("Deploy L2 contracts?").ask();
        Ok(BootstrapperConfig {
            deploy_l2_contracts,
        })
    }
}
