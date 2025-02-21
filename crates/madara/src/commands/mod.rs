use std::path::{Path, PathBuf};

pub mod anvil;
pub mod bootstrapper;
pub mod madara;
pub mod orchestrator;
pub mod os;
pub mod pathfinder;

pub(crate) fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}
