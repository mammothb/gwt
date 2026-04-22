use std::{env::current_dir, fs::create_dir_all, path::Path};

use anyhow::{Context, Result, bail};
use git2::{Repository, build::RepoBuilder};
use log::info;

use crate::cli::InitArgs;

pub fn init(args: &InitArgs) -> Result<()> {
    let name = match &args.name {
        Some(name) => name.clone(),
        None => format!("{}-workspace", extract_repo_name(&args.url)?),
    };

    let workspace_dir = current_dir()?.join(&name);
    if workspace_dir.exists() {
        bail!("Directory '{}' already exists", workspace_dir.display());
    }
    create_dir_all(&workspace_dir)?;
    info!("Created '{}'", workspace_dir.display());

    let repo = clone_repository(&args.url, &workspace_dir)?;
    info!("Initialized '{}'", repo.path().display());

    Ok(())
}

fn clone_repository(url: &str, workspace_dir: &Path) -> Result<Repository> {
    let repo_dir = workspace_dir.join(".bare");
    let repo = RepoBuilder::new()
        .bare(true)
        .clone(url, &repo_dir)
        .with_context(|| format!("Failed to clone {} into {}", url, repo_dir.display()))?;

    repo.config()?
        .set_str("remote.origin.fetch", "+refs/heads/*:refs/remotes/origin/*")
        .context("Failed to configure remote.origin.fetch")?;

    Ok(repo)
}

fn extract_repo_name(url: &str) -> Result<String> {
    url.rsplit_once('/')
        .and_then(|(_, part)| part.strip_suffix(".git"))
        .map(|name| name.to_string())
        .with_context(|| "Invalid repo URL")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("https://github.com/user/repo-name.git")]
    #[case("git@github.com:user/repo-name.git")]
    fn extract_repo_name_returns_name_for_valid_url(#[case] url: &str) {
        assert_eq!(extract_repo_name(url).unwrap(), "repo-name");
    }

    #[rstest]
    #[case("https://github.com/user/repo-name")]
    #[case("git@github.com:user/repo-name")]
    #[case("not-a-url")]
    #[should_panic(expected = "Invalid repo URL")]
    fn extract_repo_name_fails_for_invalid_url(#[case] url: &str) {
        extract_repo_name(url).unwrap();
    }
}
