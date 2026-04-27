use anyhow::Result;

use crate::external::Git;

pub fn purge() -> Result<()> {
    let git = Git::new();
    let worktrees = git.list_worktrees()?;
    println!("{:?}", worktrees);
    Ok(())
}
