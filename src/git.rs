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

/// Get the remote URL for a repository at `repo_dir`.
/// Tries `remote.origin.url` first, then falls back to any configured remote URL.
pub fn get_remote_url(repo_dir: &Path) -> Result<Option<String>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
        .with_context(|| format!("Failed to read remote URL for {}", repo_dir.display()))?;

    if output.status.success() {
        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !url.is_empty() {
            return Ok(Some(url));
        }
    }

    // Fallback: get the first configured remote URL
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("config")
        .arg("--get-regexp")
        .arg(r"^remote\..*\.url$")
        .output()
        .with_context(|| format!("Failed to read remote URLs for {}", repo_dir.display()))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().next() {
            if let Some((_, url)) = line.split_once(' ') {
                let url = url.trim().to_string();
                if !url.is_empty() {
                    return Ok(Some(url));
                }
            }
        }
    }

    Ok(None)
}

