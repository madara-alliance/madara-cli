use madara_cli_common::{docker, spinner::Spinner};
use xshell::Shell;

use crate::constants::MSG_BUILDING_IMAGE_SPINNER;

const CAIRO_LANG_REPO_PATH: &str = "deps/cairo-lang";
const CAIRO_LANG_DOCKER_IMAGE: &str = "cairo-lang";
const CAIRO_LANG_COMPOSE_FILE: &str = "compose.yaml";

pub fn build_os(shell: &Shell, rebuild: bool) -> anyhow::Result<()> {
    // TODO: Check if OS file is present. If not, build image and copy OS anyways
    if rebuild {
        let spinner = Spinner::new(MSG_BUILDING_IMAGE_SPINNER);
        docker::build_image(
            shell,
            CAIRO_LANG_REPO_PATH.to_string(),
            CAIRO_LANG_DOCKER_IMAGE.to_string(),
        )?;
        spinner.finish();

        let compose_file = format!("{}/{}", CAIRO_LANG_REPO_PATH, CAIRO_LANG_COMPOSE_FILE);
        docker::up(shell, &compose_file, false)?;
    }

    Ok(())
}
