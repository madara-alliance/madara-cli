use std::fs::{self, create_dir_all, File, Permissions};

use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::constants::{
    MADARA_COMPOSE_FILE, MADARA_DOCKER_IMAGE, MADARA_REPO_PATH, MSG_BUILDING_IMAGE_SPINNER,
};
use crate::constants::{MADARA_RPC_API_KEY_FILE, MADARA_RUNNER_SCRIPT, MSG_ARGS_VALIDATOR_ERR};

use anyhow::{anyhow, Context};
use madara_cli_common::{docker, logger, spinner::Spinner};
use madara_cli_config::madara::{
    MadaraPresetType, MadaraRunnerConfigDevnet, MadaraRunnerConfigFullNode, MadaraRunnerConfigMode,
    MadaraRunnerConfigSequencer, MadaraRunnerParams,
};
use madara_cli_types::madara::{MadaraMode, MadaraNetwork};
use xshell::Shell;

use super::workspace_dir;

pub(crate) fn run(args: MadaraRunnerConfigMode, shell: &Shell) -> anyhow::Result<()> {
    logger::info("Input Madara parameters...");

    // let params = MadaraRunnerConfigMode::default();
    let args = args
        .fill_values_with_prompt()
        .context(MSG_ARGS_VALIDATOR_ERR)?;

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
    check_secrets(args.mode.expect("Mode must be already set"))?;

    let compose_file = format!("{}/{}", MADARA_REPO_PATH, MADARA_COMPOSE_FILE);
    docker::up(shell, &compose_file, false)
}

fn process_params(args: &MadaraRunnerConfigMode) -> anyhow::Result<()> {
    let mode = args.mode.expect("Mode must be already set");

    let runner_params = match &args.params {
        MadaraRunnerParams::Devnet(params) => parse_devnet_params(&args.name, &mode, params),
        MadaraRunnerParams::Sequencer(params) => parse_sequencer_params(&args.name, &mode, params),
        MadaraRunnerParams::FullNode(params) => parse_full_node_params(&args.name, &mode, params),
    }?;

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

fn parse_full_node_params(
    name: &String,
    _mode: &MadaraMode,
    params: &MadaraRunnerConfigFullNode,
) -> anyhow::Result<Vec<String>> {
    let db_path = params
        .base_path
        .clone()
        .expect("Base path must be already set");
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
        format!("--base-path {}", db_path),
        "--l1-endpoint $RPC_API_KEY".to_string(),
    ];

    Ok(full_node_params)
}

#[cfg(test)]
mod tests {
    use std::{
        ffi::OsStr,
        process::{Child, Command},
        time::Duration,
    };

    use enigo::{Direction, Enigo, Key, Keyboard, Settings};
    use rstest::rstest;

    use crate::workspace_dir;

    pub struct CommandTest {
        child: Child,
        enigo: Enigo,
    }

    impl CommandTest {
        pub fn new<S: AsRef<OsStr>>(command: &str, args: Vec<S>) -> anyhow::Result<Self> {
            let _ = std::process::Command::new(env!("CARGO"))
                .arg("build")
                .arg(command)
                .output()
                .unwrap();
            let bin = workspace_dir().join("target/debug").join(command);
            let child = Command::new(bin)
                .args(args)
                .spawn()
                .expect("Failed to start process");
            let _ = std::process::Command::new("docker")
                .arg("stop")
                .arg("madara_runner")
                .output()
                .unwrap();
            let _ = std::process::Command::new("docker")
                .arg("rm")
                .arg("madara_runner")
                .output()
                .unwrap();
            let enigo = Enigo::new(&Settings::default()).unwrap();

            Ok(Self { child, enigo })
        }

        // Simulate keypress
        pub fn press_key(&mut self, key: Key) {
            std::thread::sleep(Duration::from_millis(200));
            self.enigo.key(key, Direction::Click).unwrap();
        }

        pub fn type_text(&mut self, text: &str) {
            std::thread::sleep(Duration::from_millis(200));
            for c in text.chars() {
                std::thread::sleep(Duration::from_millis(200));
                self.press_key(Key::Unicode(c));
            }
        }

        pub fn wait_for_status(&mut self, status: &str) -> anyhow::Result<()> {
            fn get_container_status() -> String {
                let output = std::process::Command::new("docker")
                    .arg("inspect")
                    .arg("--format='{{.State.Status}}'")
                    .arg("madara_runner")
                    .output()
                    .unwrap();

                let err = String::from_utf8_lossy(&output.stderr);
                let status = &output.status;
                println!("Docker command results {err}, {status:?}");

                let binding = String::from_utf8_lossy(&output.stdout);
                let string = binding.trim();
                if string.is_empty() {
                    return "pending".to_owned();
                }
                let first_last_off: &str = &string[1..string.len() - 1];
                first_last_off.to_owned()
            }
            loop {
                std::thread::sleep(Duration::from_millis(1000));
                let c_status = get_container_status();
                let is_valid = c_status == status;
                if is_valid {
                    self.child.kill().unwrap();
                    break;
                }
                if c_status == "exited" {
                    self.child.kill().unwrap();
                    panic!("Unexpected container exit")
                }
            }
            Ok(())
        }
    }

    impl Drop for CommandTest {
        fn drop(&mut self) {
            self.child.kill().unwrap();
            let _ = std::process::Command::new("docker")
                .arg("stop")
                .arg("madara_runner")
                .output()
                .unwrap();
        }
    }

    #[rstest]
    #[timeout(Duration::from_secs(60))]
    fn test_madara_devnet_create() {
        let mut command = CommandTest::new("madara", vec!["create"]).unwrap();
        command.press_key(enigo::Key::Return); // Select the first option (DEVNET)
        command.type_text("tmp_devnet_db"); // Select the custom DB
        command.press_key(enigo::Key::Return); // Start the process
                                               // command.wait_for_status("running").unwrap();
    }

    #[rstest]
    #[timeout(Duration::from_secs(120))]
    #[should_panic]
    fn test_madara_full_node_create() {
        let mut command = CommandTest::new("madara", vec!["create"]).unwrap();
        command.press_key(enigo::Key::DownArrow); // Scroll to FullNode
        command.press_key(enigo::Key::DownArrow); // Scroll to FullNode
        command.press_key(enigo::Key::Return); // Start the process
        command.type_text("tmp_full"); // Select the custom DB

        command.press_key(enigo::Key::Return); // Start the process
        command.press_key(enigo::Key::Return); // Start the process

        let _output = command.wait_for_status("running").unwrap();
    }
}
