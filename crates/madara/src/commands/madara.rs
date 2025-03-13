use std::fs::{self, create_dir_all, File, Permissions};

use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::constants::{
    MADARA_COMPOSE_FILE, MADARA_DOCKER_IMAGE, MADARA_REPO_PATH, MSG_BUILDING_IMAGE_SPINNER,
};
use crate::constants::{MADARA_RPC_API_KEY_FILE, MADARA_RUNNER_SCRIPT, MSG_ARGS_VALIDATOR_ERR};

use anyhow::{anyhow, Context};
use cliclack::log;
use madara_cli_common::Prompt;
use madara_cli_common::{docker, logger, spinner::Spinner};
use madara_cli_config::madara::{
    MadaraRunnerConfigFullNode, MadaraRunnerConfigMode, MadaraRunnerConfigSequencer,
    MadaraRunnerParams,
};
use madara_cli_types::madara::{MadaraMode, MadaraNetwork};
use xshell::Shell;

use super::{orchestrator, workspace_dir};

// For devnet, sequencer and full node, default DBs folder is madara-cli/deps/data
const DBS_PATH: &str = "../data/";
const ENV_FILE_PATH: &str = "deps/madara/.env";

pub(crate) fn run(args: MadaraRunnerConfigMode, shell: &Shell) -> anyhow::Result<()> {
    logger::info("Input Madara parameters...");
    let args = args
        .fill_values_with_prompt()
        .context(MSG_ARGS_VALIDATOR_ERR)?;

    let mode = args.mode.expect("Mode must be already set");
    match mode {
        MadaraMode::AppChain => orchestrator::run(args, shell)?,
        _ => {
            let spinner = Spinner::new(MSG_BUILDING_IMAGE_SPINNER);
            build_image(shell)?;
            spinner.finish();
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
    check_secrets(args.mode.expect("Mode must be already set"))?;

    // TODO: check if we need to run docker::down to remove any remaining previous instance
    let compose_file = format!("{}/{}", MADARA_REPO_PATH, MADARA_COMPOSE_FILE);
    docker::up(shell, &compose_file, false)
}

pub fn process_params(args: &MadaraRunnerConfigMode) -> anyhow::Result<()> {
    let mode = args.mode.expect("Mode must be already set");

    let runner_params = match &args.params {
        MadaraRunnerParams::Devnet(_) => parse_devnet_params(&args.name, &mode),
        MadaraRunnerParams::Sequencer(params) => parse_sequencer_params(&args.name, &mode, params),
        MadaraRunnerParams::FullNode(params) => parse_full_node_params(&args.name, &mode, params),
        MadaraRunnerParams::AppChain(params) => parse_appchain_params(&args.name, params),
    }?;

    write_env_file(args)?;
    let runner_script_path = workspace_dir()
        .join(MADARA_REPO_PATH)
        .join(MADARA_RUNNER_SCRIPT);
    create_runner_script(mode, runner_params, &runner_script_path)?;

    Ok(())
}

/// This will receive the necessary params to launch Madara and it'll overwrite `madara-runner.sh`,
/// so it can be used by docker-compose file to spin up the node
fn create_runner_script(
    mode: MadaraMode,
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

fn check_secrets(mode: MadaraMode) -> anyhow::Result<()> {
    match mode {
        MadaraMode::Sequencer | MadaraMode::FullNode => {
            let rpc_api_secret = PathBuf::new()
                .join(MADARA_REPO_PATH)
                .join(MADARA_RPC_API_KEY_FILE);

            // Create .secrets and missing folders
            if let Some(parent) = rpc_api_secret.parent() {
                if !parent.exists() {
                    log::info(format!(
                        "Creating missing directories: {}",
                        parent.display()
                    ))?;
                    create_dir_all(parent).map_err(|e| {
                        anyhow!("Failed to create directories {}: {}", parent.display(), e)
                    })?;
                }
            }
            if !rpc_api_secret.exists() {
                let rpc_api_url: String = Prompt::new("Input RPC_API url:").ask();
                log::info(format!("Creating file: {}", rpc_api_secret.display()))?;
                fs::write(rpc_api_secret, rpc_api_url)?;
            } else {
                let rpc_api_url = fs::read_to_string(&rpc_api_secret)?;
                let rpc_api_url: String = Prompt::new("Input RPC_API url:")
                    .default(&rpc_api_url)
                    .ask();
                fs::write(rpc_api_secret, rpc_api_url)?;
            }
        }
        MadaraMode::Devnet => {
            let rpc_api_secret = PathBuf::new()
                .join(MADARA_REPO_PATH)
                .join(MADARA_RPC_API_KEY_FILE);

            // Create .secrets and missing folders
            if let Some(parent) = rpc_api_secret.parent() {
                if !parent.exists() {
                    log::info(format!(
                        "Creating missing directories: {}",
                        parent.display()
                    ))?;
                    create_dir_all(parent).map_err(|e| {
                        anyhow!("Failed to create directories {}: {}", parent.display(), e)
                    })?;
                }
            }
            if !rpc_api_secret.exists() {
                log::info(format!("Creating file: {}", rpc_api_secret.display()))?;
                fs::write(rpc_api_secret, "")?;
            }
        }
        MadaraMode::AppChain => {}
    }
    Ok(())
}

fn write_env_file(args: &MadaraRunnerConfigMode) -> anyhow::Result<()> {
    let db_folder = match &args.params {
        MadaraRunnerParams::Devnet(params) => {
            params.base_path.clone().expect("DB name must be set")
        }
        MadaraRunnerParams::FullNode(params) => {
            params.base_path.clone().expect("DB name must be set")
        }
        MadaraRunnerParams::Sequencer(params) => {
            params.base_path.clone().expect("DB name must be set")
        }
        MadaraRunnerParams::AppChain(_) => return Ok(()),
    };

    fs::write(
        ENV_FILE_PATH,
        format!("MADARA_DATA_DIR={}{}", DBS_PATH, db_folder),
    )?;

    Ok(())
}

fn parse_devnet_params(name: &String, mode: &MadaraMode) -> anyhow::Result<Vec<String>> {
    let devnet_params = vec![
        format!("--name {}", name),
        format!("--{}", mode).to_lowercase(),
        "--base-path /tmp/madara".to_string(),
        "--rpc-external".to_string(),
    ];

    Ok(devnet_params)
}

fn parse_sequencer_params(
    name: &String,
    mode: &MadaraMode,
    params: &MadaraRunnerConfigSequencer,
) -> anyhow::Result<Vec<String>> {
    let chain_config_path = params
        .chain_config_path
        .clone()
        .expect("Chain config file must be set");

    // TODO: handle optional params.
    let sequencer_params = vec![
        format!("--name {}", name),
        format!("--{}", mode).to_lowercase(),
        "--base-path /tmp/madara".to_string(),
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
    let network = match params.network {
        Some(MadaraNetwork::Mainnet) => "main",
        Some(MadaraNetwork::Testnet) => "test",
        Some(MadaraNetwork::Integration) => "integration",
        Some(MadaraNetwork::Devnet) => "devnet",
        _ => panic!("A network is required"),
    };
    let full_node_params = vec![
        format!("--name {}", name),
        format!("--network {}", network),
        format!("--full"),
        "--base-path /tmp/madara".to_string(),
        "--rpc-external".to_string(),
        "--l1-endpoint $RPC_API_KEY".to_string(),
    ];

    Ok(full_node_params)
}

fn parse_appchain_params(
    name: &String,
    params: &MadaraRunnerConfigSequencer,
) -> anyhow::Result<Vec<String>> {
    let chain_config_path = params
        .chain_config_path
        .clone()
        .expect("Chain config file must be set");

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
        "--rpc-admin".to_string(),
        "--rpc-admin-port 9943".to_string(),
        "--rpc-admin-external".to_string(),
        "--l1-endpoint http://anvil:8545".to_string(),
    ];

    Ok(appchain_params)
}
