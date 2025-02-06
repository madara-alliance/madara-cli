use std::fs::{self, create_dir_all, File, Permissions};

use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::constants::{
    MADARA_COMPOSE_FILE, MADARA_DOCKER_IMAGE, MADARA_REPO_PATH, MSG_BUILDING_IMAGE_SPINNER,
};
use crate::constants::{MADARA_RPC_API_KEY_FILE, MADARA_RUNNER_SCRIPT};

use anyhow::anyhow;
use madara_cli_common::{docker, logger, spinner::Spinner};
use madara_cli_config::compose::Compose;
use madara_cli_config::madara::{
    MadaraRunnerConfigDevnet, MadaraRunnerConfigFullNode, MadaraRunnerConfigMode,
    MadaraRunnerConfigSequencer, MadaraRunnerParams,
};
use madara_cli_types::madara::{MadaraMode, MadaraNetwork};
use xshell::Shell;

use super::{orchestrator, workspace_dir};

pub(crate) fn run(args: MadaraRunnerConfigMode, shell: &Shell) -> anyhow::Result<()> {
    logger::info("Input Madara parameters...");
    let mode = args.mode();
    match mode {
        MadaraMode::AppChain => orchestrator::run(args, shell)?,
        _ => {
            // TODO: @whichqua resolve if we should build the image or use one from the registry
            if !ci_info::is_ci() {
                let spinner = Spinner::new(MSG_BUILDING_IMAGE_SPINNER);
                build_image(shell)?;
                spinner.finish();
            }

            madara_run(shell, args)?;
        }
    };

    Ok(())
}

pub fn build_image(shell: &Shell) -> anyhow::Result<()> {
    docker::build_image(
        shell,
        MADARA_REPO_PATH.to_string(),
        MADARA_DOCKER_IMAGE.to_string(),
    )
}

fn madara_run(shell: &Shell, args: MadaraRunnerConfigMode) -> anyhow::Result<()> {
    process_params(&args)?;
    let mode = args.mode();
    check_secrets(&mode)?;

    let default_file = format!("{}/{}", MADARA_REPO_PATH, MADARA_COMPOSE_FILE);
    let mut compose: Compose = serde_yml::from_slice(&fs::read(default_file)?)?;
    let container_name = format!("madara-{}", mode.to_string().to_lowercase());
    let compose_file = format!("{}/{}.yaml", MADARA_REPO_PATH, container_name);
    compose.services.get_mut("madara").unwrap().container_name = Some(container_name);

    // TODO: @whichqua resolve if we should build the image or use one from the registry
    if ci_info::is_ci() {
        compose.services.get_mut("madara").unwrap().image = "whichqua/madara:latest".to_string();
    }

    fs::write(&compose_file, serde_yml::to_string(&compose)?)?;

    docker::up(shell, &compose_file, false)
}

pub fn process_params(args: &MadaraRunnerConfigMode) -> anyhow::Result<()> {
    let mode = args.mode();
    let runner_params = match &args.params {
        MadaraRunnerParams::Devnet(params) => parse_devnet_params(&args.name, &mode, params),
        MadaraRunnerParams::Sequencer(params) => parse_sequencer_params(&args.name, &mode, params),
        MadaraRunnerParams::FullNode(params) => parse_full_node_params(&args.name, &mode, params),
        MadaraRunnerParams::AppChain(params) => parse_appchain_params(&args.name, params),
    }?;

    let runner_script_path = workspace_dir()
        .join(MADARA_REPO_PATH)
        .join(MADARA_RUNNER_SCRIPT);
    create_runner_script(&mode, runner_params, &runner_script_path)?;

    Ok(())
}

/// This will receive the necessary params to launch Madara and it'll overwrite `madara-runner.sh`,
/// so it can be used by docker-compose file to spin up the node
fn create_runner_script(
    mode: &MadaraMode,
    params: Vec<String>,
    path: &PathBuf,
) -> anyhow::Result<()> {
    let mut script = String::from("#!/bin/sh\n\n");

    // Add check for RPC_API_KEY_FILE
    match mode {
        MadaraMode::FullNode | MadaraMode::Sequencer => {
            script.push_str("if [ -f \"$RPC_API_KEY_FILE\" ]; then\n");
            script.push_str("  export RPC_API_KEY=$(cat \"$RPC_API_KEY_FILE\")\n");
            script.push_str("else\n");
            script.push_str("  echo \"Error: RPC_API_KEY_FILE not found!\" >&2\n");
            script.push_str("  exit 1\n");
            script.push_str("fi\n\n");
        }
        _ => {}
    }

    script.push_str("exec tini -- ./madara \\\n");

    // Append Madara parameters
    for param in params {
        script.push_str(&format!("  {} \\\n", param));
    }

    // Remove the trailing backslash and newline
    if script.ends_with("\\\n") {
        script.truncate(script.len() - 2);
        script.push('\n');
    }

    // Dump config into madara-runner script
    let mut file = File::create(&path)?;
    file.write_all(script.as_bytes())?;

    // Set execuion permission
    let perms = Permissions::from_mode(0o755);
    fs::set_permissions(path, perms)?;

    Ok(())
}

fn check_secrets(mode: &MadaraMode) -> anyhow::Result<()> {
    let rpc_api_secret = workspace_dir()
        .join(MADARA_REPO_PATH)
        .join(MADARA_RPC_API_KEY_FILE);

    // Create .secrets and missing folders
    if let Some(parent) = rpc_api_secret.parent() {
        if !parent.exists() {
            println!("Creating missing directories: {}", parent.display());
            create_dir_all(parent)
                .map_err(|e| anyhow!("Failed to create directories {}: {}", parent.display(), e))?;
        }
    }

    // TODO: handle missing file for other modes
    if !rpc_api_secret.exists() {
        match mode {
            MadaraMode::Devnet => {
                println!("Creating empty file: {}", rpc_api_secret.display());
                File::create(&rpc_api_secret).map_err(|e| {
                    anyhow!("Failed to create file {}: {}", rpc_api_secret.display(), e)
                })?;
            }
            _ => {
                return Err(anyhow!(
                    "RPC API file must be provided for mode {:?} at {}",
                    mode,
                    rpc_api_secret.display()
                ));
            }
        }
    }

    Ok(())
}

fn parse_devnet_params(
    name: &String,
    mode: &MadaraMode,
    params: &MadaraRunnerConfigDevnet,
) -> anyhow::Result<Vec<String>> {
    let db_path = &params.base_path;

    let devnet_params = vec![
        format!("--name {}", name),
        format!("--{}", mode).to_lowercase(),
        format!("--base-path {}", db_path),
        "--rpc-external".to_string(),
    ];

    Ok(devnet_params)
}

fn parse_sequencer_params(
    name: &String,
    mode: &MadaraMode,
    params: &MadaraRunnerConfigSequencer,
) -> anyhow::Result<Vec<String>> {
    let chain_config_path = &params.chain_config_path;

    // TODO: handle optional params.
    let sequencer_params = vec![
        format!("--name {}", name),
        format!("--{}", mode).to_lowercase(),
        "--base-path /usr/share/madara/data".to_string(),
        format!("--chain-config-path {}", chain_config_path),
        "--feeder-gateway-enable".to_string(),
        "--gateway-enable".to_string(),
        "--gateway-external".to_string(),
        "--rpc-external".to_string(),
        "--rpc-port 9945".to_string(),
        "--rpc-cors \"*\"".to_string(),
        "--gas-price 10".to_string(),
        "--blob-gas-price 20".to_string(),
        "--gateway-port 8080".to_string(),
        "--l1-endpoint http://anvil:8545".to_string(),
    ];

    Ok(sequencer_params)
}

fn parse_full_node_params(
    name: &String,
    _mode: &MadaraMode,
    params: &MadaraRunnerConfigFullNode,
) -> anyhow::Result<Vec<String>> {
    let db_path = &params.base_path;
    let network = match params.network {
        MadaraNetwork::Mainnet => "main",
        MadaraNetwork::Testnet => "test",
        MadaraNetwork::Integration => "integration",
        MadaraNetwork::Devnet => "devnet",
    };
    let full_node_params = vec![
        format!("--name {}", name),
        format!("--network {}", network),
        format!("--full"),
        format!("--base-path {}", db_path),
        "--rpc-external".to_string(),
        "--l1-endpoint $RPC_API_KEY".to_string(),
    ];

    Ok(full_node_params)
}

fn parse_appchain_params(
    name: &String,
    params: &MadaraRunnerConfigSequencer,
) -> anyhow::Result<Vec<String>> {
    let chain_config_path = &params.chain_config_path;

    let appchain_params = vec![
        format!("--name {}", name),
        "--sequencer".to_string(),
        "--base-path /usr/share/madara/data".to_string(),
        format!("--chain-config-path {}", chain_config_path),
        "--feeder-gateway-enable".to_string(),
        "--gateway-enable".to_string(),
        "--gateway-external".to_string(),
        "--rpc-external".to_string(),
        "--rpc-port 9945".to_string(),
        "--rpc-cors \"*\"".to_string(),
        "--gas-price 10".to_string(),
        "--blob-gas-price 20".to_string(),
        "--gateway-port 8080".to_string(),
        "--l1-endpoint http://anvil:8545".to_string(),
    ];

    Ok(appchain_params)
}
