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
    /// Initialize a new workspace
    Init(InitArgs),
}

#[derive(Args)]
pub struct InitArgs {
    // The repository to clone from
    pub repository: String,
    /// Set the resulting workspace name, defaults to the '<repo>-workspace'
    #[arg(short, long)]
    pub name: Option<String>,
}
