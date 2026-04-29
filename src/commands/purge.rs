use anyhow::Result;

use crate::external::Git;

pub fn purge() -> Result<()> {
    let git = Git::new();
    git.get_tracked_branches()?
        .difference(&git.list_worktrees()?.branches())
        .map(|branch| format!("branch.{branch}"))
        .for_each(|name| match git.remove_section(&name) {
            Ok(_) => log::info!("Removed config section: '{name}'"),
            Err(err) => log::warn!("Failed to remove config section '{name}': {err}"),
        });
    Ok(())
}
