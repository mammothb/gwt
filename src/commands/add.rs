use std::path::Path;

use anyhow::Result;

use crate::external::Git;

pub fn add(branch: &str, commit: Option<&str>, path: &Path) -> Result<()> {
    let git = Git::new();
    let root = git.show_toplevel().or_else(|_| git.get_worktree_root())?;
    let path = if path.is_relative() {
        root.join(path)
    } else {
        path.to_path_buf()
    };
    git.add_worktree(branch, &path, commit)?;
    log::info!(
        "Added worktree at {}. Checked out {} at {}",
        path.display(),
        branch,
        commit.unwrap_or("HEAD")
    );
    Ok(())
}
