use madara_cli_common::docker;
use xshell::Shell;

const ANVIL_REPO_PATH: &str = "deps/anvil";
const ANVIL_DOCKER_IMAGE: &str = "anvil";

pub fn build_image(shell: &Shell) -> anyhow::Result<()> {
    docker::build_image(
        shell,
        ANVIL_REPO_PATH.to_string(),
        ANVIL_DOCKER_IMAGE.to_string(),
    )?;

    Ok(())
}
