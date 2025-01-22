use madara_cli_common::logger;
use madara_cli_config::{madara::MadaraRunnerConfigMode, pathfinder::PathfinderRunnerConfigMode};
use madara_cli_types::madara::MadaraMode;
use xshell::Shell;

use crate::commands;

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
        mode: Some(MadaraMode::Sequencer),
        ..Default::default()
    };

    let shell = Shell::new().unwrap();
    commands::madara::run(args_madara, &shell)?;

    // Collect Pathfinder configuration
    let args_pathfinder = PathfinderRunnerConfigMode::default();
    commands::pathfinder::run(args_pathfinder, &shell)?;

    // Collect SNOS configuration

    // Collect Prover configuration

    // Spin up all the necessary services

    Ok(())
}
