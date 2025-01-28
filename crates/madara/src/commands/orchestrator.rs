use madara_cli_common::{docker, logger, PromptConfirm};
use madara_cli_config::{
    madara::{MadaraRunnerConfigMode, MadaraRunnerConfigSequencer, MadaraRunnerParams},
    pathfinder::PathfinderRunnerConfigMode,
    prover::ProverRunnerConfig,
};
use madara_cli_types::madara::MadaraMode;
use xshell::Shell;

use crate::{
    commands,
    constants::{DEPS_REPO_PATH, ORCHESTRATOR_COMPOSE_FILE},
};

pub fn run(shell: &Shell) -> anyhow::Result<()> {
    logger::new_empty_line();
    logger::intro();

    // Collect Madara configuration
    let args_madara = MadaraRunnerConfigMode {
        name: "Madara".to_string(),
        mode: Some(MadaraMode::Sequencer),
        params: MadaraRunnerParams::Sequencer(MadaraRunnerConfigSequencer::default()),
    }
    .fill_values_with_prompt()?;
    commands::madara::process_params(&args_madara)?;

    // Collect Pathfinder configuration
    let args_pathfinder = PathfinderRunnerConfigMode::default().fill_values_with_prompt()?;
    commands::pathfinder::parse_params(&args_pathfinder)?;

    // Collect Prover configuration
    let _args_prover = ProverRunnerConfig::default().fill_values_with_prompt()?;

    // Rebuild OS?
    let rebuild = PromptConfirm::new("Rebuild OS?").ask();
    commands::os::build_os(shell, rebuild)?;

    // Spin up all the necessary services
    run_orchestrator(&shell)?;

    Ok(())
}

fn run_orchestrator(shell: &Shell) -> anyhow::Result<()> {
    let compose_file = format!("{}/{}", DEPS_REPO_PATH, ORCHESTRATOR_COMPOSE_FILE);
    docker::up(shell, &compose_file, false)
}
