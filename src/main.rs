mod cli;
mod commands;
mod external;
mod logger;

use clap::Parser;
use cli::{Cli, Commands};
use commands::init_workspace;
use lazy_static::lazy_static;
use log::{LevelFilter, error, set_logger, set_max_level};

use crate::logger::Logger;

lazy_static! {
    static ref LOGGER: Logger = {
        let level = std::env::var("GWT_LOG")
            .ok()
            .and_then(|v| v.parse::<LevelFilter>().ok())
            .unwrap_or(LevelFilter::Info);
        set_max_level(level);
        Logger::new(level)
    };
}

fn main() {
    if let Err(err) = set_logger(&*LOGGER) {
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
