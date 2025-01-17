#![allow(unused)]
use std::path::PathBuf;

use madara_cli_types::madara::{MadaraMode, MadaraNetwork};

struct MadaraRunnerConfigDevnet {
    name: String,
    mode: MadaraMode,
    base_path: PathBuf,
}

struct MadaraRunnerConfigFullNode {
    name: String,
    mode: MadaraMode,
    network: MadaraNetwork,
    rpc_external: bool,
    l1_endpoint: String,
}

// TODO: replace MadaraCreateArgs with MadaraRunnerConfigMode
enum MadaraRunnerConfigMode {
    Devnet(MadaraRunnerConfigDevnet),
    FullNode(MadaraRunnerConfigFullNode),
}
