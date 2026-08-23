use anyhow::{Context, Result};
use regex::Regex;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub default_path: PathBuf,
    pub local_path: Option<String>,
    pub workspaces: Vec<Workspace>,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub pattern: String,
    pub path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            local_path: Some("git.local".to_string()),
            workspaces: Vec::new(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = match dirs::home_dir() {
            Some(home) => home.join(".config").join("git-tidy").join("config.json"),
            None => return Self::default(),
        };

        if !config_path.exists() {
            return Self::default();
        }

        match Self::load_from(&config_path) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Warning: Failed to parse git-tidy config: {e}");
                Self::default()
            }
        }
    }

    fn load_from(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config from {}", path.display()))?;

        let config: Config = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config from {}", path.display()))?;

        Ok(config)
    }

    /// Resolve which workspace root a URL should be cloned into.
    /// Returns the first workspace whose regex matches, or the default path.
    pub fn resolve_workspace(&self, url: &str) -> PathBuf {
        for ws in &self.workspaces {
            if let Ok(re) = Regex::new(&ws.pattern)
                && re.is_match(url)
            {
                return ws.path.clone();
            }
        }
        self.default_path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = Config::default();
        assert!(config.local_path.is_some());
        assert!(config.workspaces.is_empty());
    }

    #[test]
    fn resolve_workspace_no_match() {
        let config = Config {
            default_path: PathBuf::from("/default"),
            workspaces: vec![Workspace {
                pattern: r".*ghe\.company\.com.*".to_string(),
                path: PathBuf::from("/work"),
            }],
            ..Default::default()
        };

        assert_eq!(
            config.resolve_workspace("https://github.com/owner/repo"),
            PathBuf::from("/default")
        );
    }

    #[test]
    fn resolve_workspace_match() {
        let config = Config {
            default_path: PathBuf::from("/default"),
            workspaces: vec![
                Workspace {
                    pattern: r".*ghe\.company\.com.*".to_string(),
                    path: PathBuf::from("/work"),
                },
                Workspace {
                    pattern: r".*github\.com.*".to_string(),
                    path: PathBuf::from("/dev"),
                },
            ],
            ..Default::default()
        };

        assert_eq!(
            config.resolve_workspace("https://ghe.company.com/org/repo"),
            PathBuf::from("/work")
        );
        assert_eq!(
            config.resolve_workspace("https://github.com/owner/repo"),
            PathBuf::from("/dev")
        );
    }
}
