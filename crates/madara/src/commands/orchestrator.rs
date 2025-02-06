use madara_cli_common::{docker, logger, spinner::Spinner};
use madara_cli_config::{
    madara::MadaraRunnerConfigMode,
    pathfinder::PathfinderRunnerConfigMode,
    prover::{ProverRunnerConfig, ProverType},
};
use xshell::Shell;

use crate::{
    commands,
    constants::{DEPS_REPO_PATH, MSG_BUILDING_IMAGE_SPINNER},
};

use dotenvy::from_filename;
use minijinja::{context, Environment};
use std::env;
use std::fs;

const ORCHESTRATOR_REPO_PATH: &str = "deps/orchestrator";
const ORCHESTRATOR_DOCKER_IMAGE: &str = "orchestrator";
const ORCHESTRATOR_COMPOSE_TEMPLATE_FILE: &str = "compose.template";
const ORCHESTRATOR_COMPOSE_FILE: &str = "compose.yaml";
const ORCHESTRATOR_ENV_TEMPLATE_FILE: &str = ".env.template";
const ORCHESTRATOR_ENV_FILE: &str = ".env";
const ORCHESTRATOR_RUNNER_TEMPLATE_FILE: &str = "run_orchestrator.template";
const ORCHESTRATOR_RUNNER_FILE: &str = "run_orchestrator.sh";

pub(crate) fn run(args_madara: MadaraRunnerConfigMode, shell: &Shell) -> anyhow::Result<()> {
    logger::new_empty_line();
    logger::intro();

    // Collect Madara configuration
    commands::madara::process_params(&args_madara)?;

    // Collect Pathfinder configuration
    let args_pathfinder = PathfinderRunnerConfigMode::default().fill_values_with_prompt()?;
    commands::pathfinder::parse_params(&args_pathfinder)?;

    // Check if the ATLANTIC_API was already set
    let env_output = format!("{}/{}", ORCHESTRATOR_REPO_PATH, ORCHESTRATOR_ENV_FILE);
    let _ = from_filename(env_output);
    let prev_atlantic_api = env::var("MADARA_ORCHESTRATOR_ATLANTIC_API_KEY")
        .unwrap_or_else(|_| "ATLANTIC_API_KEY".to_string());

    // Collect Prover configuration
    let args_prover = ProverRunnerConfig::default().fill_values_with_prompt(&prev_atlantic_api)?;
    populate_orchestrator_env(&args_prover)?;
    pupolate_orchestrator_runner(&args_prover)?;
    pupolate_orchestrator_compose(&args_prover)?;

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

fn pupolate_orchestrator_runner(prover_config: &ProverRunnerConfig) -> anyhow::Result<()> {
    let runner_template = format!(
        "{}/{}",
        ORCHESTRATOR_REPO_PATH, ORCHESTRATOR_RUNNER_TEMPLATE_FILE
    );
    let runner_output = format!("{}/{}", ORCHESTRATOR_REPO_PATH, ORCHESTRATOR_RUNNER_FILE);

    // Read the template file
    let template =
        fs::read_to_string(runner_template).expect("Failed to read run_orchestrator.template");

    // Set up MiniJinja
    let mut env = Environment::new();
    env.add_template("runner_template", &template)
        .expect("Failed to add template");

    let prover = match prover_config.prover_type {
        ProverType::Dummy => "sharp",
        ProverType::Atlantic => "atlantic",
        ProverType::Stwo => panic!("Not supported yet"),
    };
    let data = context! {PROVER_TYPE => prover};

    // Render the template
    let tmpl = env.get_template("runner_template").unwrap();
    let rendered = tmpl.render(&data).expect("Template rendering failed");
    fs::write(runner_output, rendered).expect("Failed to write run_orchestrator");

    Ok(())
}

fn pupolate_orchestrator_compose(prover_config: &ProverRunnerConfig) -> anyhow::Result<()> {
    let compose_template = format!("{}/{}", DEPS_REPO_PATH, ORCHESTRATOR_COMPOSE_TEMPLATE_FILE);
    let compose_output = format!("{}/{}", DEPS_REPO_PATH, ORCHESTRATOR_COMPOSE_FILE);

    // Read the template file
    let template = fs::read_to_string(compose_template).expect("Failed to read compose.template");

    // Set up MiniJinja
    let mut env = Environment::new();
    env.add_template("compose_template", &template)
        .expect("Failed to add template");
    let data = context! {ENABLE_DUMMY_PROVER => prover_config.prover_type == ProverType::Dummy};

    // Render the template
    let tmpl = env.get_template("compose_template").unwrap();
    let rendered = tmpl.render(&data).expect("Template rendering failed");
    fs::write(compose_output, rendered).expect("Failed to write compose.yaml");

    Ok(())
}
