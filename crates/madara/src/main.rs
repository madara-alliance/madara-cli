mod commands;
mod config;

mod constants;

use clap::{Parser, Subcommand};
use cliclack::log;
use commands::workspace_dir;
use constants::DEFAULT_TMP_DATA_DIRECTORY;
use madara_cli_common::config::{init_global_config, GlobalConfig};
use madara_cli_config::madara::MadaraRunnerConfigMode;
use xshell::Shell;

use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "Madara CLI")]
#[command(version = "0.0.1")]
#[command(about = "Madara CLI to easily spin up nodes")]
struct Madara {
    #[command(subcommand)]
    command: MadaraSubcommands,
    #[clap(flatten)]
    global: MadaraGlobalArgs,
}

#[derive(Parser, Debug)]
#[clap(next_help_heading = "Global options")]
struct MadaraGlobalArgs {
    /// Verbose mode
    #[clap(short, long, global = true)]
    verbose: bool,
    /// Path to the configuration file
    #[clap(short, long, global = true)]
    config_file: Option<String>,
    /// Default: takes all default values without user interaction
    #[clap(short, long, global = true)]
    default: bool,
}

#[derive(Subcommand, Debug)]
pub enum MadaraSubcommands {
    /// Create configuration file for app-chain
    Init,
    /// Create a Madara node
    Create,
}

fn main() -> anyhow::Result<()> {
    let args = Madara::parse();
    init_data_directory()?;

    match run_subcommand(args) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error(format!("Could not complete request: {e}"))?;
            std::process::exit(1);
        }
    }
}

fn run_subcommand(madara_args: Madara) -> anyhow::Result<()> {
    let shell = Shell::new().unwrap();
    shell.change_dir(workspace_dir());
    init_global_config_inner(&shell, &madara_args.global)?;

    let args = MadaraRunnerConfigMode::default();

    match madara_args.command {
        MadaraSubcommands::Init => commands::orchestrator::init(),
        MadaraSubcommands::Create => commands::madara::run(args, &shell),
    }?;

    Ok(())
}

fn init_global_config_inner(_shell: &Shell, madara_args: &MadaraGlobalArgs) -> anyhow::Result<()> {
    init_global_config(GlobalConfig {
        verbose: madara_args.verbose,
        config_file: madara_args.config_file.clone(),
        default: madara_args.default,
    });
    Ok(())
}

fn init_data_directory() -> anyhow::Result<()> {
    let deps_data_dir = Path::new(DEFAULT_TMP_DATA_DIRECTORY);
    if !deps_data_dir.exists() {
        fs::create_dir_all(deps_data_dir).expect("Unable to create data directory");
    }
    Ok(())
}
