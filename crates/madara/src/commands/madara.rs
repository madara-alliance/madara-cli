use std::path::PathBuf;

use crate::{
    commands::args::madara::{MadaraCreateArgs, MadaraMode},
    constants::{MADARA_DOCKER_IMAGE, MSG_BUILDING_IMAGE_SPINNER, MSG_STARTING_CONTAINERS_SPINNER},
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
    Ok(())
}

fn madara_run(shell: &Shell, params: MadaraCreateArgs) -> anyhow::Result<()> {
    Ok(())
}
