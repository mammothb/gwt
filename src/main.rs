mod commands;
mod external;
mod logger;

use std::{path::Path, process};

use clap::Parser;
use lazy_static::lazy_static;
use log::{LevelFilter, error, set_logger, set_max_level};

use crate::{
    commands::{Cli, Commands, add_worktree, init_workspace, purge_workspace},
    logger::Logger,
};

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
        Commands::Add { path, args } => add_worktree(&args.branch, args.commit.as_deref(), path),
        Commands::AddFeat(args) => {
            add_worktree(&args.branch, args.commit.as_deref(), Path::new("feat"))
        }
        Commands::AddFix(args) => {
            add_worktree(&args.branch, args.commit.as_deref(), Path::new("fix"))
        }
        Commands::AddPr(args) => {
            add_worktree(&args.branch, args.commit.as_deref(), Path::new("pr"))
        }
        Commands::Init(args) => init_workspace(&args.url, args.name.as_deref()),
        Commands::Purge => purge_workspace(),
    } {
        error!("Failed: {err}");
        process::exit(1);
    }
}
