use std::fs::{self, create_dir_all, File, Permissions};

use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::constants::{
    MADARA_COMPOSE_FILE, MADARA_DOCKER_IMAGE, MADARA_REPO_PATH, MSG_BUILDING_IMAGE_SPINNER,
};
use crate::constants::{MADARA_RPC_API_KEY_FILE, MADARA_RUNNER_SCRIPT, MSG_ARGS_VALIDATOR_ERR};

use anyhow::{anyhow, Context};
use madara_cli_common::Prompt;
use madara_cli_common::{docker, logger, spinner::Spinner};
use madara_cli_config::madara::{
    MadaraPresetType, MadaraRunnerConfigDevnet, MadaraRunnerConfigMode,
    MadaraRunnerConfigSequencer, MadaraRunnerParams,
};
use madara_cli_types::madara::MadaraMode;
use xshell::Shell;

pub(crate) fn run(args: MadaraRunnerConfigMode, shell: &Shell) -> anyhow::Result<()> {
    logger::info("Input Madara parameters...");

    // let params = MadaraRunnerConfigMode::default();
    let args = args
        .fill_values_with_prompt()
        .context(MSG_ARGS_VALIDATOR_ERR)?;

    check_secrets(args.mode.expect("Mode must be already set"))?;
    let spinner = Spinner::new(MSG_BUILDING_IMAGE_SPINNER);
    madara_build_image(shell)?;
    spinner.finish();

    madara_run(shell, args)?;

    Ok(())
}

fn madara_build_image(shell: &Shell) -> anyhow::Result<()> {
    docker::build_image(
        shell,
        MADARA_REPO_PATH.to_string(),
        MADARA_DOCKER_IMAGE.to_string(),
    )
}

fn madara_run(shell: &Shell, args: MadaraRunnerConfigMode) -> anyhow::Result<()> {
    process_params(&args)?;
    let compose_file = format!("{}/{}", MADARA_REPO_PATH, MADARA_COMPOSE_FILE);
    docker::up(shell, &compose_file, false)
}

fn process_params(args: &MadaraRunnerConfigMode) -> anyhow::Result<()> {
    let mode = args.mode.expect("Mode must be already set");

    let runner_params = match &args.params {
        MadaraRunnerParams::Devnet(params) => parse_devnet_params(&args.name, &mode, params),
        MadaraRunnerParams::Sequencer(params) => parse_sequencer_params(&args.name, &mode, params),
        MadaraRunnerParams::FullNode(_) => panic!("Not supported yet"),
    }?;

    let runner_script_path = &format!("{}/{}", MADARA_REPO_PATH, MADARA_RUNNER_SCRIPT);
    create_runner_script(mode, runner_params, runner_script_path)?;

    Ok(())
}

/// This will receive the necessary params to launch Madara and it'll overwrite `madara-runner.sh`,
/// so it can be used by docker-compose file to spin up the node
fn create_runner_script(
    mode: MadaraMode,
    params: Vec<String>,
    output_path: &str,
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
    let path = Path::new(output_path);
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
            let rpc_api_secret = Path::new(MADARA_REPO_PATH).join(MADARA_RPC_API_KEY_FILE);

            // Create .secrets and missing folders
            if let Some(parent) = rpc_api_secret.parent() {
                if !parent.exists() {
                    println!("Creating missing directories: {}", parent.display());
                    create_dir_all(parent).map_err(|e| {
                        anyhow!("Failed to create directories {}: {}", parent.display(), e)
                    })?;
                }
            }
            if !rpc_api_secret.exists() {
                let rpc_api_url: String = Prompt::new("Input RPC_API url:").ask();
                println!("Creating file: {}", rpc_api_secret.display());
                fs::write(rpc_api_secret, rpc_api_url)?;
            }
        }
        _ => {}
    }

    // TODO: handle missing file for other modes

    Ok(())
}

fn parse_devnet_params(
    name: &String,
    mode: &MadaraMode,
    params: &MadaraRunnerConfigDevnet,
) -> anyhow::Result<Vec<String>> {
    // TODO: handle optional params.
    let db_path = params
        .base_path
        .clone()
        .expect("Base path must be already set");

    let devnet_params = vec![
        format!("--name {}", name),
        format!("--{}", mode).to_lowercase(),
        format!("--base-path {}", db_path),
    ];

    Ok(devnet_params)
}

fn parse_sequencer_params(
    name: &String,
    mode: &MadaraMode,
    params: &MadaraRunnerConfigSequencer,
) -> anyhow::Result<Vec<String>> {
    // TODO: handle optional params.
    let db_path = params
        .base_path
        .clone()
        .expect("Base path must be already set");

    let preset = params.preset.clone().expect("Preset must be already set");
    let preset_path = if preset.preset_type == MadaraPresetType::Custom {
        preset.path.expect("Preset path must be set")
    } else {
        preset.preset_type.to_string().to_lowercase()
    };

    let devnet_params = vec![
        format!("--name {}", name),
        format!("--{}", mode).to_lowercase(),
        format!("--base-path {}", db_path),
        format!("--preset {}", preset_path),
        "--l1-endpoint $RPC_API_KEY".to_string(),
    ];

    Ok(devnet_params)
}
