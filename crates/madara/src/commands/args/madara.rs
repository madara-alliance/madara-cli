use clap::ValueEnum;
use std::path::PathBuf;

use clap::Parser;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, Default, EnumIter, strum::Display)]
pub enum MadaraMode {
    #[default]
    Devnet,
    Sequencer,
    FullNode,
    AppChain,
}

#[derive(Debug, Parser)]
pub struct MadaraCreateArgs {
    pub name: String,
    pub mode: MadaraMode,
    pub base_path: PathBuf,
}
