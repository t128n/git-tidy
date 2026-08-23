use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub default_path: PathBuf,
    pub local_path: Option<String>,
    pub workspaces: Vec<Workspace>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub pattern: String,
    pub path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_path: dirs::home_dir().map(|h| h.join("Dev")).unwrap_or_else(|| PathBuf::from(".")),
            local_path: Some("git.local".to_string()),
            workspaces: Vec::new(),
        }
    }
}

impl Config {
    pub fn config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".config").join("git-tidy").join("config.json"))
    }

    pub fn load() -> Self {
        let config_path = match Self::config_path() {
            Some(path) => path,
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

    pub fn init(force: bool) -> Result<PathBuf> {
        let path = Self::config_path().context("Could not determine user home directory")?;
        if path.exists() && !force {
            anyhow::bail!("Config file already exists at {}. Use --force to overwrite.", path.display());
        }

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory {}", parent.display()))?;
        }

        let default_dev = dirs::home_dir().map(|h| h.join("Dev")).unwrap_or_else(|| PathBuf::from("."));
        let default_work = dirs::home_dir().map(|h| h.join("Dev-work")).unwrap_or_else(|| PathBuf::from("."));

        let template_config = Config {
            default_path: default_dev.clone(),
            local_path: Some("git.local".to_string()),
            workspaces: vec![
                Workspace {
                    pattern: r".*ghe\.company\.com.*".to_string(),
                    path: default_work,
                },
                Workspace {
                    pattern: r".*github\.com.*".to_string(),
                    path: default_dev,
                },
            ],
        };

        let json = serde_json::to_string_pretty(&template_config)
            .context("Failed to serialize config")?;
        std::fs::write(&path, json)
            .with_context(|| format!("Failed to write config file {}", path.display()))?;

        Ok(path)
    }

    pub fn reset() -> Result<PathBuf> {
        let path = Self::config_path().context("Could not determine user home directory")?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory {}", parent.display()))?;
        }

        let default_config = Config::default();
        let json = serde_json::to_string_pretty(&default_config)
            .context("Failed to serialize config")?;
        std::fs::write(&path, json)
            .with_context(|| format!("Failed to write config file {}", path.display()))?;

        Ok(path)
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
