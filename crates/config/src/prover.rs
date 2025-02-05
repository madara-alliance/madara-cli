use clap::ValueEnum;
use strum::{EnumIter, IntoEnumIterator};

use madara_cli_common::{Prompt, PromptSelect};

#[derive(Default)]
pub struct ProverRunnerConfig {
    pub url: String,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, Default, EnumIter, strum::Display)]
pub enum ProverType {
    #[default]
    Dummy,
    Atlantic,
    Stwo,
}

impl ProverRunnerConfig {
    pub fn fill_values_with_prompt(
        self,
        prev_atlantic_api: &str,
    ) -> anyhow::Result<ProverRunnerConfig> {
        let prover = PromptSelect::new("Select Prover:", ProverType::iter()).ask();

        let url = match prover {
            ProverType::Dummy => prev_atlantic_api.to_string(),
            ProverType::Atlantic => Prompt::new("Input Atlantic prover API key:")
                .default(prev_atlantic_api)
                .ask(),
            ProverType::Stwo => panic!("Stwo prover is not supported yet"),
        };

        Ok(ProverRunnerConfig { url })
    }
}
