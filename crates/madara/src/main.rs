mod commands;

mod constants;
use clap::{Parser, Subcommand};
use madara_cli_common::config::{init_global_config, GlobalConfig};
use xshell::Shell;

use crate::commands::madara::run;

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
        Err(_) => {
            // TODO: add log with the error
            std::process::exit(1);
        }
    }
}

fn run_subcommand(madara_args: Madara) -> anyhow::Result<()> {
    let shell = Shell::new().unwrap();
    init_global_config_inner(&shell, &madara_args.global)?;

    match madara_args.command {
        MadaraSubcommands::Create => run(&shell),
    }?;

    Ok(())
}

fn init_global_config_inner(_shell: &Shell, madara_args: &MadaraGlobalArgs) -> anyhow::Result<()> {
    init_global_config(GlobalConfig {
        verbose: madara_args.verbose,
    });
    Ok(())
}
