use anyhow::{Context, Result};
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

        let display_name = path.file_name().unwrap_or_default().to_string_lossy();
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

/// Compare two paths after canonicalizing, handling missing paths gracefully.
fn paths_equal(a: &Path, b: &Path) -> bool {
    let a_str = a.to_string_lossy().to_lowercase();
    let b_str = b.to_string_lossy().to_lowercase();

    // Normalize path separators for comparison
    let a_normalized = a_str.replace('\\', "/");
    let b_normalized = b_str.replace('\\', "/");

    a_normalized == b_normalized
}
