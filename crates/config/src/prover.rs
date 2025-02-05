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
    pub fn fill_values_with_prompt(self) -> anyhow::Result<ProverRunnerConfig> {
        let prover = PromptSelect::new("Select Prover:", ProverType::iter()).ask();

        let url = match prover {
            ProverType::Dummy => "".to_string(),
            ProverType::Atlantic => Prompt::new("Input Atlantic prover API key:").ask(),
            ProverType::Stwo => panic!("Stwo prover is not supported yet"),
        };

        Ok(ProverRunnerConfig { url })
    }
}
