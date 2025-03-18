use clap::{Parser, ValueEnum};
use strum::{EnumIter, IntoEnumIterator};

use madara_cli_common::{Prompt, PromptConfirm, PromptSelect};

#[derive(Debug, Default, Parser, Clone)]
pub struct ProverRunnerConfig {
    pub prover_type: ProverType,
    pub url: String,
    pub build_images: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, Default, EnumIter, strum::Display)]
pub enum ProverType {
    #[default]
    Dummy,
    Atlantic,
    Stwo,
}

impl ProverRunnerConfig {
    pub fn fill_values_with_prompt(prev_atlantic_api: &str) -> anyhow::Result<ProverRunnerConfig> {
        let prover_type = PromptSelect::new("Select Prover:", ProverType::iter()).ask();

        let url = match prover_type {
            ProverType::Dummy => prev_atlantic_api.to_string(),
            ProverType::Atlantic => Prompt::new("Input Atlantic prover API key:")
                .default(prev_atlantic_api)
                .ask(),
            ProverType::Stwo => panic!("Stwo prover is not supported yet"),
        };

        let build_images = PromptConfirm::new("Build and use local images?").ask();

        Ok(ProverRunnerConfig {
            prover_type,
            url,
            build_images,
        })
    }
}
