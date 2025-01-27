use madara_cli_common::{docker, logger};
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

pub fn run(_shell: &Shell) -> anyhow::Result<()> {
    logger::new_empty_line();
    logger::intro();

    let services: String = vec!["Madara", "SNOS", "Prover", "Pathfinder", "Anvil"]
        .iter()
        .map(|arg| format!("  ✅ {}", arg)) // You can replace "✅" with other emojis like "☑️" or custom checkboxes
        .collect::<Vec<_>>()
        .join("\n");
    logger::note("AppChain configuration", services);

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
    let _args_proover = ProverRunnerConfig::default().fill_values_with_prompt()?;

    // Spin up all the necessary services
    let shell = Shell::new().unwrap();
    run_orchestrator(&shell)?;

    Ok(())
}

fn run_orchestrator(shell: &Shell) -> anyhow::Result<()> {
    let compose_file = format!("{}/{}", DEPS_REPO_PATH, ORCHESTRATOR_COMPOSE_FILE);
    docker::up(shell, &compose_file, false)
}
