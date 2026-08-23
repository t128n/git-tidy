mod config;
mod git;
mod normalize;
mod organize;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use config::Config;
use normalize::normalize_git_url;

#[derive(Parser)]
#[command(
    name = "git-tidy",
    about = "Git workspace organizer and cloning utility",
    version,
    after_help = "Run 'git-tidy <command> --help' for more information on a specific command."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Generate shell completions for the specified shell
    #[arg(long = "completions", value_enum)]
    completions: Option<Shell>,
}

#[derive(Subcommand)]
enum Commands {
    /// Clone a repository into a structured host/org/repo path
    Clone {
        /// The remote URL of the repository to clone
        url: String,

        /// Root directory to clone into (overrides workspace routing)
        root: Option<PathBuf>,
    },

    /// Move flat repositories into structured paths
    Organize {
        /// Directory containing repositories to organize (defaults to config defaultPath)
        path: Option<PathBuf>,

        /// Preview what would be moved without making changes
        #[arg(long)]
        dry_run: bool,
    },

    /// Print usage information
    Help,
}

#[derive(clap::ValueEnum, Clone)]
#[allow(clippy::enum_variant_names)]
enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle shell completion generation
    if let Some(shell) = cli.completions {
        use clap::CommandFactory;
        let mut cmd = Cli::command();
        let shell_name = match shell {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Zsh => clap_complete::Shell::Zsh,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::PowerShell => clap_complete::Shell::PowerShell,
            Shell::Elvish => clap_complete::Shell::Elvish,
        };
        clap_complete::generate(shell_name, &mut cmd, "git-tidy", &mut std::io::stdout());
        return Ok(());
    }

    let config = Config::load();

    match cli.command {
        Some(Commands::Clone { url, root }) => cmd_clone(&config, &url, root),
        Some(Commands::Organize { path, dry_run }) => {
            cmd_organize(&config, path, dry_run)
        }
        Some(Commands::Help) | None => {
            print_usage();
            Ok(())
        }
    }
}

fn cmd_clone(config: &Config, url: &str, root: Option<PathBuf>) -> Result<()> {
    let root = root.unwrap_or_else(|| config.resolve_workspace(url));

    let rel_path: PathBuf = normalize_git_url(url).split('/').collect();
    let target = root.join(rel_path);

    if target.exists() {
        eprintln!("Target already exists: {}", target.display());
        std::process::exit(1);
    }

    git::clone(url, &target)
        .with_context(|| format!("Failed to clone {url}"))?;

    Ok(())
}

fn cmd_organize(config: &Config, path: Option<PathBuf>, dry_run: bool) -> Result<()> {
    let options = organize::OrganizeOptions { path, dry_run };
    organize::organize(config, &options)
}

fn print_usage() {
    println!("git-tidy - Git workspace organizer and cloning utility");
    println!();
    println!("USAGE:");
    println!("    git-tidy clone <url> [root-path]     Clone repository into host/org/repo structure");
    println!("    git-tidy organize [path] [--dry-run]  Organize existing flat git repos");
    println!("    git-tidy help                        Print this help message");
    println!();
    println!("OPTIONS:");
    println!("    --dry-run    Preview moves without making changes");
    println!("    --completions <SHELL>  Generate shell completions (bash, zsh, fish, powershell, elvish)");
}
