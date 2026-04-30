use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, crate_version};

#[derive(Parser)]
#[command(about = "Git worktree helper.")]
#[command(arg_required_else_help = true)]
#[command(version = crate_version!())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new worktree
    Add(AddArgs),
    /// Initialize a new workspace
    Init(InitArgs),
    /// Purge stale branch configurations
    Purge,
}

#[derive(Args)]
pub struct AddArgs {
    /// Create a new branch named <BRANCH> and check out <BRANCH> into the new
    /// worktree. Refuses to create a new branch if it already exists.
    #[arg(short)]
    pub branch: String,
    /// Create a worktree at <PATH>.
    pub path: PathBuf,
    /// Start at <COMMIT-ISH>.
    #[arg(value_name = "COMMIT-ISH")]
    pub commit: Option<String>,
}

#[derive(Args)]
pub struct InitArgs {
    // The repository to clone from
    pub url: String,
    /// Set the resulting workspace name, defaults to the '<repo>-workspace'
    #[arg(short, long)]
    pub name: Option<String>,
}
