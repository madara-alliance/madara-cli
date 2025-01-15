use std::path::PathBuf;

use crate::{
    commands::args::madara::{MadaraCreateArgs, MadaraMode},
    constants::{
        MADARA_DOCKER_IMAGE, MADARA_REPO_PATH, MSG_BUILDING_IMAGE_SPINNER,
        MSG_STARTING_CONTAINERS_SPINNER,
    },
};

use anyhow::Context;
use madara_cli_common::{docker, logger, spinner::Spinner, Prompt, PromptSelect};
use strum::IntoEnumIterator;
use xshell::Shell;

pub fn run(shell: &Shell) -> anyhow::Result<()> {
    logger::info("Input Madara parameters...");

    let name = "Madara".to_string();
    let mode = PromptSelect::new("Select Madara mode:", MadaraMode::iter()).ask();
    let base_path: PathBuf = Prompt::new("Input DB path:").default("./madara-db").ask();

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
    let name = "madara_node";
    let docker_args: Vec<String> = vec!["--name".to_string(), name.to_string(), "--rm".to_string()];

    // TODO: implement a better to_string as params must be lowercase
    let mode = match params.mode {
        MadaraMode::Devnet => "devnet",
        _ => panic!("Mode not supported"),
    };

    let command: Vec<String> = vec![
        format!("--{}", mode),
        "--name".to_string(),
        params.name,
        "--base-path".to_string(),
        params.base_path.to_string_lossy().to_string(),
    ];

    docker::run_command(shell, MADARA_DOCKER_IMAGE, docker_args, command)
}
