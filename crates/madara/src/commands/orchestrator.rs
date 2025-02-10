use madara_cli_common::{docker, logger, spinner::Spinner};
use madara_cli_config::madara::{MadaraRunnerConfigMode, MadaraRunnerParams};
use xshell::Shell;

use crate::{
    commands,
    constants::{DEPS_REPO_PATH, MSG_BUILDING_IMAGE_SPINNER},
};

const ORCHESTRATOR_REPO_PATH: &str = "deps/orchestrator";
const ORCHESTRATOR_DOCKER_IMAGE: &str = "orchestrator";
const ORCHESTRATOR_COMPOSE_FILE: &str = "compose.yaml";

pub(crate) fn run(args_madara: MadaraRunnerConfigMode, shell: &Shell) -> anyhow::Result<()> {
    logger::new_empty_line();
    // Collect Madara configuration
    commands::madara::process_params(&args_madara)?;

    let args = match args_madara.params {
        MadaraRunnerParams::AppChain(args) => args,
        _ => unreachable!("AppChain config expected"),
    };
    commands::pathfinder::parse_params(&args.pathfinder_config)?;

    // Collect Prover configuration
    let _args_prover = &args.prover_config;

    // Build all images
    build_images(shell)?;

    // Spin up all the necessary services
    run_orchestrator(&shell)?;

    Ok(())
}

fn run_orchestrator(shell: &Shell) -> anyhow::Result<()> {
    let compose_file = format!("{}/{}", DEPS_REPO_PATH, ORCHESTRATOR_COMPOSE_FILE);
    docker::up(shell, &compose_file, false)
}

fn build_images(shell: &Shell) -> anyhow::Result<()> {
    let spinner = Spinner::new(MSG_BUILDING_IMAGE_SPINNER);
    commands::pathfinder::build_image(shell)?;
    commands::anvil::build_image(shell)?;
    build_image(shell)?;
    spinner.finish();

    Ok(())
}

fn build_image(shell: &Shell) -> anyhow::Result<()> {
    docker::build_image(
        shell,
        ORCHESTRATOR_REPO_PATH.to_string(),
        ORCHESTRATOR_DOCKER_IMAGE.to_string(),
    )?;

    Ok(())
}
