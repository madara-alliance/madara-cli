use madara_cli_common::docker;
use xshell::Shell;

use crate::constants::{ANVIL_DOCKER_IMAGE, ANVIL_REPO_PATH};

pub fn build_image(shell: &Shell) -> anyhow::Result<()> {
    docker::build_image(
        shell,
        ANVIL_REPO_PATH.to_string(),
        ANVIL_DOCKER_IMAGE.to_string(),
    )?;

    Ok(())
}
