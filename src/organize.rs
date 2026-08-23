use anyhow::{Context, Result};
use regex::Regex;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::git;
use crate::normalize::normalize_git_url;

pub struct OrganizeOptions {
    pub path: Option<PathBuf>,
    pub dry_run: bool,
}

pub fn organize(config: &Config, options: &OrganizeOptions) -> Result<()> {
    let root = options.path.as_ref().unwrap_or(&config.default_path);

    if !root.exists() {
        anyhow::bail!("Path does not exist: {}", root.display());
    }

    let entries = std::fs::read_dir(root)
        .with_context(|| format!("Failed to read directory: {}", root.display()))?;

    let mut handled: Vec<PathBuf> = Vec::new();

    // First pass: organize git repositories
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let display_name = path.file_name().unwrap_or_default().to_string_lossy();

        // Skip hidden directories (.git, .config, etc.)
        if display_name.starts_with('.') {
            continue;
        }

        // Skip the local_path directory itself
        if let Some(ref local_path) = config.local_path
            && display_name == *local_path
        {
            continue;
        }

        // Check if this is a git repository
        let git_dir = path.join(".git");
        if !git_dir.exists() {
            continue;
        }

        let url = match git::get_remote_url(&path)? {
            Some(url) => url,
            None => continue,
        };

        let clean_path = normalize_git_url(&url);
        let workspace = config.resolve_workspace(&url);

        // Build target: workspace + normalized path (with platform separators)
        let rel_path: PathBuf = clean_path.split('/').collect();
        let target = workspace.join(rel_path);

        // Skip if already in the right place or target exists
        if paths_equal(&path, &target) || target.exists() {
            handled.push(path);
            continue;
        }

        let target_display = normalize_git_url(&url);

        if options.dry_run {
            println!("[dry-run] Would move: {display_name} -> {target_display}");
        } else {
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
            }

            std::fs::rename(&path, &target).with_context(|| {
                format!(
                    "Failed to move {} -> {}",
                    path.display(),
                    target.display()
                )
            })?;

            println!("Moved: {display_name} -> {target_display}");
        }

        handled.push(path);
    }

    // Second pass: move unrecognized directories to localPath (if configured)
    if let Some(ref local_path) = config.local_path
        && !local_path.is_empty()
    {
        let entries = std::fs::read_dir(root)
            .with_context(|| format!("Failed to read directory: {}", root.display()))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if !path.is_dir() || handled.contains(&path) {
                continue;
            }

            let display_name = path.file_name().unwrap_or_default().to_string_lossy();

            // 1. Skip hidden directories (.git, .config, etc.)
            if display_name.starts_with('.') {
                continue;
            }

            // 2. Skip the local_path directory itself
            if display_name == *local_path {
                continue;
            }

            // 3. Skip hostname / domain directories (e.g. "github.com", "mercedes-benz.ghe.com")
            if is_hostname_like(&display_name) {
                continue;
            }

            // 4. Skip directories matching any workspace pattern
            if config.workspaces.iter().any(|ws| {
                Regex::new(&ws.pattern)
                    .map(|r| r.is_match(&display_name))
                    .unwrap_or(false)
            }) {
                continue;
            }

            // 5. Skip directories that contain nested git repositories (structured domain/org folders)
            if contains_nested_git_repos(&path) {
                continue;
            }

            let target = root.join(local_path).join(&*display_name);

            if paths_equal(&path, &target) || target.exists() {
                continue;
            }

            if options.dry_run {
                println!("[dry-run] Would move: {display_name} -> {local_path}/{display_name}");
            } else {
                if let Some(parent) = target.parent() {
                    std::fs::create_dir_all(parent).with_context(|| {
                        format!("Failed to create directory: {}", parent.display())
                    })?;
                }

                std::fs::rename(&path, &target).with_context(|| {
                    format!(
                        "Failed to move {} -> {}",
                        path.display(),
                        target.display()
                    )
                })?;

                println!("Moved: {display_name} -> {local_path}/{display_name}");
            }
        }
    }

    Ok(())
}

/// Check if a directory name looks like a hostname / domain (e.g. "github.com", "mercedes-benz.ghe.com").
fn is_hostname_like(name: &str) -> bool {
    if !name.contains('.') || name.starts_with('.') || name.ends_with('.') {
        return false;
    }

    name.split('.').all(|part| !part.is_empty() && part.chars().all(|c| c.is_alphanumeric() || c == '-'))
}

/// Check if a directory contains any nested git repositories (up to 3 levels deep).
fn contains_nested_git_repos(dir: &Path) -> bool {
    fn check_dir(dir: &Path, depth: usize) -> bool {
        if depth > 3 {
            return false;
        }
        let Ok(entries) = std::fs::read_dir(dir) else {
            return false;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.join(".git").exists() {
                    return true;
                }
                if check_dir(&path, depth + 1) {
                    return true;
                }
            }
        }
        false
    }
    check_dir(dir, 1)
}

/// Compare two paths after canonicalizing, handling missing paths gracefully.
fn paths_equal(a: &Path, b: &Path) -> bool {
    let a_str = a.to_string_lossy().to_lowercase();
    let b_str = b.to_string_lossy().to_lowercase();

    // Normalize path separators for comparison
    let a_normalized = a_str.replace('\\', "/");
    let b_normalized = b_str.replace('\\', "/");

    a_normalized == b_normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hostname_like() {
        assert!(is_hostname_like("github.com"));
        assert!(is_hostname_like("mercedes-benz.ghe.com"));
        assert!(is_hostname_like("gitlab.my-org.co.uk"));
        assert!(!is_hostname_like("my-project"));
        assert!(!is_hostname_like(".git"));
        assert!(!is_hostname_like(".config"));
        assert!(!is_hostname_like("trailing.dot."));
    }
}

