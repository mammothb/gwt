use clap::{Parser, Subcommand, crate_version};

#[derive(Parser)]
#[command(about = "Git worktree helper.")]
#[command(version = crate_version!())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
}
