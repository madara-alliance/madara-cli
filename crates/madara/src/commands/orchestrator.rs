use madara_cli_common::logger;
use madara_cli_config::madara::MadaraRunnerConfigMode;
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
    let args_madara = MadaraRunnerConfigMode::default();

    let shell = Shell::new().unwrap();
    commands::madara::run(args_madara, &shell)?;

    // Collect SNOS configuration

    // Collect Pathfinder configuration

    // Collect Prover configuration

    // Prom

    // Spin up all the necessary services

    Ok(())
}
