use std::fs::{self, File, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use anyhow::Context;
use madara_cli_common::{docker, logger, spinner::Spinner};
use madara_cli_config::pathfinder::PathfinderRunnerConfigMode;
use xshell::Shell;

use crate::constants::{
    MSG_ARGS_VALIDATOR_ERR, MSG_BUILDING_IMAGE_SPINNER, PATHFINDER_COMPOSE_FILE,
    PATHFINDER_DOCKER_IMAGE, PATHFINDER_REPO_PATH, PATHFINDER_RUNNER_SCRIPT,
};

pub fn run(args: PathfinderRunnerConfigMode, shell: &Shell) -> anyhow::Result<()> {
    logger::info("Input Pathfinder parameters...");

    let args = args
        .fill_values_with_prompt()
        .context(MSG_ARGS_VALIDATOR_ERR)?;

    let spinner = Spinner::new(MSG_BUILDING_IMAGE_SPINNER);
    build_image(shell)?;
    spinner.finish();

    pathfinder_run(args, shell)?;

    Ok(())
}

pub fn build_image(shell: &Shell) -> anyhow::Result<()> {
    docker::build_image(
        shell,
        PATHFINDER_REPO_PATH.to_string(),
        PATHFINDER_DOCKER_IMAGE.to_string(),
    )
}

fn pathfinder_run(args: PathfinderRunnerConfigMode, shell: &Shell) -> anyhow::Result<()> {
    parse_params(&args)?;

    let compose_file = format!("{}/{}", PATHFINDER_REPO_PATH, PATHFINDER_COMPOSE_FILE);
    docker::up(shell, &compose_file, false)
}

fn create_runner_script(params: Vec<String>, output_path: &str) -> anyhow::Result<()> {
    let mut script = String::from("#!/bin/sh\n\n");

    script.push_str("exec tini -- ./pathfinder \\\n");

    // Append Pathfinder parameters
    for param in params {
        script.push_str(&format!("  {} \\\n", param));
    }

    // Remove the trailing backslash and newline
    if script.ends_with("\\\n") {
        script.truncate(script.len() - 2);
        script.push('\n');
    }

    // Dump config into pathfinder-runner script
    let path = Path::new(output_path);
    let mut file = File::create(&path)?;
    file.write_all(script.as_bytes())?;

    // Set execuion permission
    let perms = Permissions::from_mode(0o755);
    fs::set_permissions(path, perms)?;

    Ok(())
}

pub fn parse_params(params: &PathfinderRunnerConfigMode) -> anyhow::Result<()> {
    // TODO: handle optional params.
    let (network, chain_id, gateway_url, feeder_gateway_url, http_rpc, data_directory) =
        params.unwrap_all();

    let pathfinder_params = vec![
        format!("--network {}", network).to_lowercase(),
        format!("--chain-id {}", chain_id),
        "--ethereum.url http://anvil:8545".to_string(),
        format!("--gateway-url {}", gateway_url),
        format!("--feeder-gateway-url {}", feeder_gateway_url),
        "--storage.state-tries archive".to_string(),
        format!("--data-directory {}", data_directory),
        format!("--http-rpc {}", http_rpc),
    ];

    let runner_script_path = &format!("{}/{}", PATHFINDER_REPO_PATH, PATHFINDER_RUNNER_SCRIPT);
    create_runner_script(pathfinder_params, runner_script_path)?;

    Ok(())
}
