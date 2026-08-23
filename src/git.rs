use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Clone a repository from `url` into `target_dir`.
pub fn clone(url: &str, target_dir: &Path) -> Result<()> {
    let status = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(target_dir)
        .status()
        .with_context(|| format!("Failed to execute `git clone {url}`"))?;

    if !status.success() {
        anyhow::bail!("`git clone {url}` exited with status {status}");
    }

    Ok(())
}

/// Get the `remote.origin.url` for a repository at `repo_dir`.
pub fn get_remote_url(repo_dir: &Path) -> Result<Option<String>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
        .with_context(|| format!("Failed to read remote URL for {}", repo_dir.display()))?;

    if !output.status.success() {
        return Ok(None);
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if url.is_empty() {
        Ok(None)
    } else {
        Ok(Some(url))
    }
}
