mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use commands::cmd_init;

fn main() {
    let cli = Cli::parse();

    if let Err(err) = match &cli.command {
        Commands::Init(args) => cmd_init(args),
    } {
        eprintln!("Failed: {err}");
    }
}
