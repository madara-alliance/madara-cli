use madara_cli_common::{docker, logger, spinner::Spinner};
use madara_cli_config::{
    madara::MadaraRunnerConfigMode, pathfinder::PathfinderRunnerConfigMode,
    prover::ProverRunnerConfig,
};
use xshell::Shell;

use crate::{
    commands,
    constants::{DEPS_REPO_PATH, MSG_BUILDING_IMAGE_SPINNER},
};

use minijinja::{context, Environment};
use std::fs;

const ORCHESTRATOR_REPO_PATH: &str = "deps/orchestrator";
const ORCHESTRATOR_DOCKER_IMAGE: &str = "orchestrator";
const ORCHESTRATOR_COMPOSE_FILE: &str = "compose.yaml";
const ORCHESTRATOR_ENV_TEMPLATE_FILE: &str = ".env.template";
const ORCHESTRATOR_ENV_FILE: &str = ".env";

pub(crate) fn run(args_madara: MadaraRunnerConfigMode, shell: &Shell) -> anyhow::Result<()> {
    logger::new_empty_line();
    logger::intro();

    // Collect Madara configuration
    commands::madara::process_params(&args_madara)?;

    // Collect Pathfinder configuration
    let args_pathfinder = PathfinderRunnerConfigMode::default().fill_values_with_prompt()?;
    commands::pathfinder::parse_params(&args_pathfinder)?;

    // Collect Prover configuration
    let args_prover = ProverRunnerConfig::default().fill_values_with_prompt()?;
    populate_orchestrator_env(&args_prover)?;

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
    commands::madara::build_image(shell)?;
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

fn populate_orchestrator_env(prover_config: &ProverRunnerConfig) -> anyhow::Result<()> {
    let env_template = format!(
        "{}/{}",
        ORCHESTRATOR_REPO_PATH, ORCHESTRATOR_ENV_TEMPLATE_FILE
    );
    let env_output = format!("{}/{}", ORCHESTRATOR_REPO_PATH, ORCHESTRATOR_ENV_FILE);

    // Read the template file
    let template = fs::read_to_string(env_template).expect("Failed to read .env.template");

    // Set up MiniJinja
    let mut env = Environment::new();
    env.add_template("env_template", &template)
        .expect("Failed to add template");

    let data = context! {MADARA_ORCHESTRATOR_ATLANTIC_API_KEY => prover_config.url};

    // Render the template
    let tmpl = env.get_template("env_template").unwrap();
    let rendered = tmpl.render(&data).expect("Template rendering failed");
    fs::write(env_output, rendered).expect("Failed to write .env");

    Ok(())
}
