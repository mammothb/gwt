use anyhow::Result;

use crate::external::Git;

pub fn purge() -> Result<()> {
    let git = Git::new();
    git.get_tracked_branches()?
        .difference(&git.list_worktrees()?.branches())
        .map(|branch| format!("branch.{branch}"))
        .for_each(|name| {
            if let Err(err) = git.remove_section(&name) {
                log::warn!("Failed to remove config section '{name}': {err}");
            }
        });
    Ok(())
}
