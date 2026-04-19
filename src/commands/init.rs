use anyhow::{Context, Result};

use crate::cli::InitArgs;

pub fn init(args: &InitArgs) -> Result<()> {
    let name = match &args.name {
        Some(name) => name.clone(),
        None => format!("{}-workspace", parse_repo_name(&args.url)?),
    };
    println!("{name}");
    Ok(())
}

fn parse_repo_name(url: &str) -> Result<String> {
    url.rsplit_once('/')
        .and_then(|(_, part)| part.strip_suffix(".git"))
        .map(|name| name.to_string())
        .with_context(|| "Invalid repo URL")
}
