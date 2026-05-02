use std::{
    collections::HashMap,
    env::current_dir,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use log::info;

use crate::external::{Git, GitCloneArgs};

pub fn init(url: &str, name: Option<&str>) -> Result<()> {
    let name = match name {
        Some(name) => name.to_owned(),
        None => format!("{}-workspace", extract_repo_name(url)?),
    };

    let workspace_dir = current_dir()?.join(&name);
    if workspace_dir.exists() {
        bail!("Directory '{}' already exists", workspace_dir.display());
    }
    create_dir_all(&workspace_dir)?;
    info!("Created '{}'", workspace_dir.display());

    let repo_dir = clone_repository(url, &workspace_dir)?;
    info!("Initialized '{}'", repo_dir.display());

    Ok(())
}

fn clone_repository(url: &str, workspace_dir: &Path) -> Result<PathBuf> {
    let repo_dir = workspace_dir.join(".bare");
    let git = Git::new();
    let args = GitCloneArgs {
        url: url.into(),
        dir: repo_dir.clone(),
        bare: true,
        config: HashMap::from([(
            "remote.origin.fetch".into(),
            "+refs/heads/*:refs/remotes/origin/*".into(),
        )]),
    };
    git.clone(args)?;

    Ok(repo_dir)
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
