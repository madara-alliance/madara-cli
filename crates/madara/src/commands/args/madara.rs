use clap::Parser;
use madara_cli_types::madara::MadaraMode;

#[derive(Debug, Parser)]
pub struct MadaraCreateArgs {
    pub name: String,
    pub mode: MadaraMode,
    pub base_path: String,
}
