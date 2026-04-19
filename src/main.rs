mod cli;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => println!("{:?}, {}", args.name, args.repository),
    }
}
