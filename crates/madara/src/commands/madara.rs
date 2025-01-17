use std::fs::{self, create_dir_all, File, Permissions};

use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::constants::{MADARA_RPC_API_KEY_FILE, MADARA_RUNNER_SCRIPT};
use crate::{
    commands::args::madara::MadaraCreateArgs,
    constants::{
        MADARA_COMPOSE_FILE, MADARA_DOCKER_IMAGE, MADARA_REPO_PATH, MSG_BUILDING_IMAGE_SPINNER,
    },
};

use anyhow::anyhow;
use madara_cli_common::{docker, logger, spinner::Spinner, Prompt, PromptSelect};
use madara_cli_types::madara::MadaraMode;
use strum::IntoEnumIterator;
use xshell::Shell;

pub fn run(shell: &Shell) -> anyhow::Result<()> {
    logger::info("Input Madara parameters...");

    let name = "Madara".to_string();
    let mode = PromptSelect::new("Select Madara mode:", MadaraMode::iter()).ask();
    let base_path = Prompt::new("Input DB path:").default("./madara-db").ask();

    let params = MadaraCreateArgs {
        name,
        mode,
        base_path,
    };

    let spinner = Spinner::new(MSG_BUILDING_IMAGE_SPINNER);
    madara_build_image(shell)?;
    spinner.finish();

    madara_run(shell, params)?;

    Ok(())
}

fn madara_build_image(shell: &Shell) -> anyhow::Result<()> {
    docker::build_image(
        shell,
        MADARA_REPO_PATH.to_string(),
        MADARA_DOCKER_IMAGE.to_string(),
    )
}

fn madara_run(shell: &Shell, params: MadaraCreateArgs) -> anyhow::Result<()> {
    process_params(&params)?;
    check_secrets(params.mode)?;

    let compose_file = format!("{}/{}", MADARA_REPO_PATH, MADARA_COMPOSE_FILE);
    docker::up(shell, &compose_file, false)
}

fn process_params(params: &MadaraCreateArgs) -> anyhow::Result<()> {
    let runner_params = match params.mode {
        MadaraMode::Devnet => parse_devnet_params(&params)?,
        MadaraMode::FullNode => panic!("Not implemented yet"),
        MadaraMode::Sequencer => panic!("Not implemented yet"),
        MadaraMode::AppChain => panic!("Not implemented yet"),
    };

    let runner_script_path = &format!("{}/{}", MADARA_REPO_PATH, MADARA_RUNNER_SCRIPT);
    create_runner_script(params.mode, runner_params, runner_script_path)?;

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
    if mode == MadaraMode::FullNode {
        script.push_str("if [ -f \"$RPC_API_KEY_FILE\" ]; then\n");
        script.push_str("  export RPC_API_KEY=$(cat \"$RPC_API_KEY_FILE\")\n");
        script.push_str("else\n");
        script.push_str("  echo \"Error: RPC_API_KEY_FILE not found!\" >&2\n");
        script.push_str("  exit 1\n");
        script.push_str("fi\n\n");
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
    let rpc_api_secret = Path::new(MADARA_REPO_PATH).join(MADARA_RPC_API_KEY_FILE);

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

fn parse_devnet_params(params: &MadaraCreateArgs) -> anyhow::Result<Vec<String>> {
    // TODO: handle optional params.
    let devnet_params = vec![
        "--name madara".to_string(),
        format!("--{}", params.mode).to_lowercase(),
        format!("--base-path {}", params.base_path),
    ];

    Ok(devnet_params)
}
