use std::fs::{self, File, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use madara_cli_common::docker;
use madara_cli_config::pathfinder::PathfinderRunnerConfigMode;
use xshell::Shell;

const PATHFINDER_DOCKER_IMAGE: &str = "pathfinder";
const PATHFINDER_REPO_PATH: &str = "deps/pathfinder";
const PATHFINDER_RUNNER_SCRIPT: &str = "pathfinder-runner.sh";

pub fn build_image(shell: &Shell) -> anyhow::Result<()> {
    docker::build_image(
        shell,
        PATHFINDER_REPO_PATH.to_string(),
        PATHFINDER_DOCKER_IMAGE.to_string(),
    )
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
