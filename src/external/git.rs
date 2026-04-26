use std::{
    collections::HashMap,
    path::PathBuf,
    process::{Command, Output},
};

use anyhow::{Result, anyhow, bail};

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

    pub fn clone(&self, args: GitCloneArgs) -> Result<()> {
        self.run(&args.to_args()?)?;
        Ok(())
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
        args.push(self.url.clone());
        args.push(dir.to_string());
        Ok(args)
    }
}
