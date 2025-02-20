use madara_cli_common::docker;
use xshell::Shell;

const BOOTSTRAPPER_REPO_PATH: &str = "deps/bootstrapper";
const BOOTSTRAPPER_DOCKER_IMAGE: &str = "bootstrapper";

pub fn build_image(shell: &Shell) -> anyhow::Result<()> {
    docker::build_image(
        shell,
        BOOTSTRAPPER_REPO_PATH.to_string(),
        BOOTSTRAPPER_DOCKER_IMAGE.to_string(),
    )?;

    Ok(())
}
