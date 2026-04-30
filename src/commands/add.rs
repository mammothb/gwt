use anyhow::Result;

use crate::{cli::AddArgs, external::Git};

pub fn add(args: &AddArgs) -> Result<()> {
    let git = Git::new();
    let root = git.show_toplevel().or_else(|_| git.get_worktree_root())?;
    let path = if args.path.is_relative() {
        root.join(&args.path)
    } else {
        args.path.clone()
    };
    git.add_worktree(&args.branch, &path, args.commit.as_deref())?;
    log::info!(
        "Added worktree at {}. Checked out {} at {}",
        path.display(),
        args.branch,
        args.commit.as_deref().unwrap_or("HEAD")
    );
    Ok(())
}
