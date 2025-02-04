mod commands;

mod constants;

use clap::{Parser, Subcommand};
use cliclack::log;
use commands::workspace_dir;
use madara_cli_common::config::{init_global_config, GlobalConfig};
use madara_cli_config::madara::MadaraRunnerConfigMode;
use xshell::Shell;

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
}

#[derive(Subcommand, Debug)]
pub enum MadaraSubcommands {
    /// Create a Madara node
    Create,
}

fn main() -> anyhow::Result<()> {
    let args = Madara::parse();

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
        MadaraSubcommands::Create => commands::madara::run(args, &shell),
    }?;

    Ok(())
}

fn init_global_config_inner(_shell: &Shell, madara_args: &MadaraGlobalArgs) -> anyhow::Result<()> {
    init_global_config(GlobalConfig {
        verbose: madara_args.verbose,
    });
    Ok(())
}
