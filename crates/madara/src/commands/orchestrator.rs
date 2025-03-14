use madara_cli_common::{config::global_config, docker, logger, spinner::Spinner};
use madara_cli_config::{
    bootstrapper::BootstrapperConfig,
    madara::MadaraRunnerConfigMode,
    pathfinder::PathfinderRunnerConfigMode,
    prover::{ProverRunnerConfig, ProverType},
};
use xshell::Shell;

use crate::{
    commands,
    config::global_config::Config,
    constants::{DEFAULT_LOCAL_CONFIG_FILE, DEPS_REPO_PATH, DOCKERHUB_ORGANIZATION},
};

use dotenvy::from_filename;
use minijinja::{context, Environment};
use std::env;
use std::fs;

use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

const ORCHESTRATOR_REPO_PATH: &str = "deps/orchestrator";
const ORCHESTRATOR_DOCKER_IMAGE: &str = "orchestrator";
const ORCHESTRATOR_COMPOSE_TEMPLATE_FILE: &str = "compose.template";
const ORCHESTRATOR_COMPOSE_FILE: &str = "compose.yaml";
const ORCHESTRATOR_ENV_TEMPLATE_FILE: &str = ".env.template";
const ORCHESTRATOR_ENV_PATH: &str = "deps/orchestrator/.env";
const ORCHESTRATOR_RUNNER_TEMPLATE_FILE: &str = "run_orchestrator.template";
const ORCHESTRATOR_RUNNER_FILE: &str = "run_orchestrator.sh";

pub(crate) fn init() -> anyhow::Result<()> {
    Config::init()?;
    Ok(())
}

pub(crate) fn run(args_madara: MadaraRunnerConfigMode, shell: &Shell) -> anyhow::Result<()> {
    logger::new_empty_line();
    logger::intro("Madara CLI");

    // TODO: improve error handling and check if file doesn't exist (even default file)
    let config_file = global_config()
        .config_file
        .as_ref()
        .map(|s| s.clone())
        .unwrap_or_else(|| DEFAULT_LOCAL_CONFIG_FILE.to_string());
    let config = Config::load(&config_file);

    // Collect Madara configuration
    commands::madara::process_params(&args_madara, &config)?;

    let args_bootstrapper = BootstrapperConfig::fill_values_with_prompt()?;
    commands::bootstrapper::process_params(&config)?;

    // Collect Pathfinder configuration
    let args_pathfinder = PathfinderRunnerConfigMode::default().fill_values_with_prompt()?;
    commands::pathfinder::parse_params(&args_pathfinder, &config)?;

    // Read and load the env variables from deps/orchestrator/.env if the file was created.
    // On the first run, fallback to `ATLANTIC_API` to give the user a hint about what is needed in that field
    let _ = from_filename(ORCHESTRATOR_ENV_PATH.to_string());
    let prev_atlantic_api = env::var("MADARA_ORCHESTRATOR_ATLANTIC_API_KEY")
        .unwrap_or_else(|_| "ATLANTIC_API_KEY".to_string());

    // Collect Prover configuration
    let args_prover = ProverRunnerConfig::default().fill_values_with_prompt(&prev_atlantic_api)?;
    populate_orchestrator_env(&args_prover, &config)?;
    populate_orchestrator_runner(&args_prover)?;
    populate_orchestrator_compose(&args_prover, &args_bootstrapper, &config)?;

    // Build all images
    if args_prover.build_images {
        build_images(shell)?;
    }

    // Spin up all the necessary services
    run_orchestrator(&shell)?;

    Ok(())
}

fn run_orchestrator(shell: &Shell) -> anyhow::Result<()> {
    let compose_file = format!("{}/{}", DEPS_REPO_PATH, ORCHESTRATOR_COMPOSE_FILE);
    docker::up(shell, &compose_file, false)
}

fn build_images(shell: &Shell) -> anyhow::Result<()> {
    let spinner = Spinner::new("Building Madara image...");
    commands::madara::build_image(shell)?;
    spinner.finish();
    let spinner = Spinner::new("Building Pathfinder image...");
    commands::pathfinder::build_image(shell)?;
    spinner.finish();
    let spinner = Spinner::new("Building Anvil image...");
    commands::anvil::build_image(shell)?;
    spinner.finish();
    let spinner = Spinner::new("Building Bootstrapper image...");
    commands::bootstrapper::build_image(shell)?;
    spinner.finish();
    let spinner = Spinner::new("Building Orchestrator image...");
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

fn populate_orchestrator_env(
    prover_config: &ProverRunnerConfig,
    config: &Config,
) -> anyhow::Result<()> {
    let env_template = format!(
        "{}/{}",
        ORCHESTRATOR_REPO_PATH, ORCHESTRATOR_ENV_TEMPLATE_FILE
    );
    let env_output = ORCHESTRATOR_ENV_PATH.to_string();

    // Read the template file
    let template = fs::read_to_string(env_template).expect("Failed to read .env.template");

    // Set up MiniJinja
    let mut env = Environment::new();
    env.add_template("env_template", &template)
        .expect("Failed to add template");

    let data = context! {
        MADARA_ORCHESTRATOR_ATLANTIC_API_KEY => prover_config.url,
        MADARA_ORCHESTRATOR_ETHEREUM_PRIVATE_KEY => config.eth_wallet.eth_priv_key,
        MADARA_ORCHESTRATOR_ETHEREUM_SETTLEMENT_RPC_URL => config.l1_config.eth_rpc,
        VERIFIER_CONTRACT_ADDRESS => config.l1_config.verifier_address,
        MADARA_ORCHESTRATOR_MAX_BLOCK_NO_TO_PROCESS => config.orchestrator. maximum_block_to_process
    };

    // Render the template
    let tmpl = env.get_template("env_template").unwrap();
    let rendered = tmpl.render(&data).expect("Template rendering failed");
    fs::write(env_output, rendered).expect("Failed to write .env");

    Ok(())
}

fn populate_orchestrator_runner(prover_config: &ProverRunnerConfig) -> anyhow::Result<()> {
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

    let data = context! {
        PROVER_TYPE => prover,
    };

    // Render the template
    let tmpl = env.get_template("runner_template").unwrap();
    let rendered = tmpl.render(&data).expect("Template rendering failed");
    fs::write(&runner_output, rendered).expect("Failed to write run_orchestrator");

    let perms = Permissions::from_mode(0o755);
    fs::set_permissions(&runner_output, perms)?;

    Ok(())
}

fn populate_orchestrator_compose(
    prover_config: &ProverRunnerConfig,
    bootstrapper_config: &BootstrapperConfig,
    config: &Config,
) -> anyhow::Result<()> {
    let compose_template = format!("{}/{}", DEPS_REPO_PATH, ORCHESTRATOR_COMPOSE_TEMPLATE_FILE);
    let compose_output = format!("{}/{}", DEPS_REPO_PATH, ORCHESTRATOR_COMPOSE_FILE);

    // Read the template file
    let template = fs::read_to_string(compose_template).expect("Failed to read compose.template");

    // Set up MiniJinja
    let mut env = Environment::new();
    env.add_template("compose_template", &template)
        .expect("Failed to add template");

    let repo = if prover_config.build_images {
        ""
    } else {
        DOCKERHUB_ORGANIZATION
    };

    let data = context! {
        ENABLE_DUMMY_PROVER => prover_config.prover_type == ProverType::Dummy,
        ENABLE_BOOTSTRAPER_L2_SETUP => bootstrapper_config.deploy_l2_contracts,
        IMAGE_REPOSITORY => repo,
        ETH_PRIV_KEY => config.eth_wallet.eth_priv_key,
    };

    // Render the template
    let tmpl = env.get_template("compose_template").unwrap();
    let rendered = tmpl.render(&data).expect("Template rendering failed");
    fs::write(compose_output, rendered).expect("Failed to write compose.yaml");

    Ok(())
}
