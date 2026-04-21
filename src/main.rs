mod cli;
mod commands;
mod logger;

use clap::Parser;
use cli::{Cli, Commands};
use commands::init_workspace;
use log::{LevelFilter, error, set_logger, set_max_level};

use crate::logger::Logger;

static LOGGER: Logger = Logger::new(LevelFilter::Info);

fn main() {
    if let Err(err) = set_logger(&LOGGER).map(|()| set_max_level(LevelFilter::Info)) {
        eprintln!("Failed to initialize logger: {err}");
        return;
    }

    let cli = Cli::parse();

    if let Err(err) = match &cli.command {
        Commands::Init(args) => init_workspace(args),
    } {
        error!("Failed: {err}");
    }
}
