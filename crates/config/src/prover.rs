use clap::ValueEnum;
use strum::{EnumIter, IntoEnumIterator};

use madara_cli_common::{Prompt, PromptSelect};

#[derive(Debug, Default, Clone, clap::Parser)]
pub struct ProverRunnerConfig {
    pub prover: ProverType,
    /// API key for Atlantic prover (required if using Atlantic)
    #[arg(long, required_if_eq("prover", "atlantic"))]
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, Default, EnumIter, strum::Display)]
pub enum ProverType {
    #[default]
    Dummy,
    Atlantic,
    Stwo,
}

impl ProverRunnerConfig {
    pub fn fill_values_with_prompt() -> anyhow::Result<ProverRunnerConfig> {
        let prover = PromptSelect::new("Select Prover:", ProverType::iter()).ask();

        let api_key = match prover {
            ProverType::Dummy => None,
            ProverType::Atlantic => Some(Prompt::new("Input Atlantic prover API key:").ask()),
            ProverType::Stwo => panic!("Stwo prover is not supported yet"),
        };

        Ok(ProverRunnerConfig { api_key, prover })
    }
}
