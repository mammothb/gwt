use std::{
    collections::{HashMap, HashSet},
    fmt,
    path::{Path, PathBuf},
    process::{Command, Output},
    str::FromStr,
};

use anyhow::{Context, Result, anyhow, bail};

pub struct Git {
    executable_path: String,
}

impl Default for Git {
    fn default() -> Self {
        Self::new()
    }
}

impl Git {
    pub fn new() -> Self {
        Self {
            executable_path: std::env::var("GWT_GIT").unwrap_or_else(|_| "git".into()),
        }
    }

    pub fn add_worktree(&self, branch: &str, path: &Path, commit: Option<&str>) -> Result<()> {
        let path = path
            .to_str()
            .ok_or_else(|| anyhow!("Invalid UTF-8: '{}'", path.display()))?;
        let mut args = vec!["worktree", "add", "-b", branch, path];
        if let Some(c) = commit {
            args.push(c);
        }
        self.run(&args)?;
        Ok(())
    }

    pub fn clone(&self, args: GitCloneArgs) -> Result<()> {
        self.run(&args.to_args()?)?;
        Ok(())
    }

    pub fn get_tracked_branches(&self) -> Result<HashSet<String>> {
        let output = match self.run_optional(&[
            "config",
            "get",
            "--all",
            "--show-names",
            "--regexp",
            "^branch.*merge$",
        ])? {
            Some(output) => output,
            None => return Ok(HashSet::new()),
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
        let branches = stdout
            .lines()
            .filter_map(|line| {
                line.split_whitespace()
                    .next()
                    .and_then(|s| s.strip_prefix("branch."))
                    .and_then(|s| s.strip_suffix(".merge"))
                    .map(String::from)
            })
            .collect();
        Ok(branches)
    }

    pub fn get_worktree_root(&self) -> Result<PathBuf> {
        let output = self.run(&["rev-parse", "--absolute-git-dir"])?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let parent = PathBuf::from(stdout.trim())
            .parent()
            .context("cannot get worktree root")?
            .to_owned();
        Ok(parent)
    }

    pub fn list_worktrees(&self) -> Result<Worktrees> {
        let output = self.run(&["worktree", "list", "--porcelain"])?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(Worktrees::from_str(&stdout)?)
    }

    pub fn remove_section(&self, name: &str) -> Result<()> {
        self.run(&["config", "remove-section", name])?;
        Ok(())
    }

    pub fn show_toplevel(&self) -> Result<PathBuf> {
        let output = self.run(&["rev-parse", "--show-toplevel"])?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(PathBuf::from(stdout.trim()))
    }

    fn run(&self, args: &[impl AsRef<str>]) -> Result<Output> {
        let output = Command::new(&self.executable_path)
            .args(args.iter().map(|s| s.as_ref()))
            .output()
            .map_err(|err| anyhow!("Git error: {err}"))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::debug!("Git error: {stderr}");
            bail!("Git operation failed");
        }
        Ok(output)
    }

    fn run_optional(&self, args: &[impl AsRef<str>]) -> Result<Option<Output>> {
        let output = Command::new(&self.executable_path)
            .args(args.iter().map(|s| s.as_ref()))
            .output()
            .map_err(|err| anyhow!("Git error: {err}"))?;
        if output.status.success() {
            return Ok(Some(output));
        }
        if output.status.code() == Some(1) {
            return Ok(None);
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::debug!("Git error: {stderr}");
        bail!("Git operation failed");
    }
}

pub struct GitCloneArgs {
    pub url: String,
    pub dir: PathBuf,
    pub bare: bool,
    pub config: HashMap<String, String>,
}

impl GitCloneArgs {
    pub fn to_args(&self) -> Result<Vec<String>> {
        let dir = self
            .dir
            .to_str()
            .ok_or_else(|| anyhow!("Invalid UTF-8: '{}'", self.dir.display()))?;

        let mut args = vec!["clone".to_string()];
        if self.bare {
            args.push("--bare".to_string());
        }
        for (key, value) in &self.config {
            args.push("--config".to_string());
            args.push(format!("{key}={value}"));
        }
        args.push("--".to_string());
        args.push(self.url.clone());
        args.push(dir.to_string());
        Ok(args)
    }
}

pub struct ParseWorktreeError;

impl fmt::Display for ParseWorktreeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse worktree output")
    }
}

impl fmt::Debug for ParseWorktreeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseWorktreeError")
    }
}

impl std::error::Error for ParseWorktreeError {}

#[derive(Debug)]
pub struct Worktree {
    #[allow(dead_code)]
    path: PathBuf,
    #[allow(dead_code)]
    bare: bool,
    #[allow(dead_code)]
    head: Option<String>,
    branch: Option<String>,
}

impl FromStr for Worktree {
    type Err = ParseWorktreeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = s.lines();
        let path = lines
            .next()
            .ok_or(ParseWorktreeError)?
            .strip_prefix("worktree ")
            .ok_or(ParseWorktreeError)?;

        let mut bare = false;
        let mut head = None;
        let mut branch = None;
        for line in lines {
            match line {
                "bare" => bare = true,
                line if line.starts_with("HEAD ") => head = Some(line[5..].to_string()),
                line if line.starts_with("branch ") => {
                    let raw = &line[7..];
                    branch = Some(raw.strip_prefix("refs/heads/").unwrap_or(raw).to_string());
                }
                _ => {}
            }
        }
        Ok(Worktree {
            path: PathBuf::from(path),
            bare,
            head,
            branch,
        })
    }
}

impl Worktree {
    pub fn branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }
}

#[derive(Debug)]
pub struct Worktrees(Vec<Worktree>);

impl FromStr for Worktrees {
    type Err = ParseWorktreeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(
            s.split("\n\n")
                .map(str::trim)
                .filter(|block| !block.is_empty())
                .map(Worktree::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl std::ops::Deref for Worktrees {
    type Target = Vec<Worktree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for Worktrees {
    type Item = Worktree;
    type IntoIter = std::vec::IntoIter<Worktree>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Worktrees {
    type Item = &'a Worktree;
    type IntoIter = std::slice::Iter<'a, Worktree>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Worktrees {
    pub fn branches(&self) -> HashSet<String> {
        self.iter()
            .filter_map(|tree| tree.branch().map(String::from))
            .collect()
    }
}
