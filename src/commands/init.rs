use std::{env::current_dir, fs::create_dir_all};

use anyhow::{Context, Result, bail};
use log::info;

use crate::cli::InitArgs;

pub fn init(args: &InitArgs) -> Result<()> {
    let name = match &args.name {
        Some(name) => name.clone(),
        None => format!("{}-workspace", parse_repo_name(&args.url)?),
    };

    let workspace_dir = current_dir()?.join(&name);
    if workspace_dir.exists() {
        bail!("Directory '{}' already exists", workspace_dir.display());
    }
    create_dir_all(&workspace_dir)?;
    info!("Created '{}'", workspace_dir.display());

    Ok(())
}

fn parse_repo_name(url: &str) -> Result<String> {
    url.rsplit_once('/')
        .and_then(|(_, part)| part.strip_suffix(".git"))
        .map(|name| name.to_string())
        .with_context(|| "Invalid repo URL")
}
