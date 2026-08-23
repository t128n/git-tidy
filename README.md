# git-tidy

Git workspace organizer and cloning utility.

## Installation

### Binary Releases

Download pre-built binaries from [GitHub Releases](https://github.com/t128n/git-tidy/releases).

Available targets:
- `x86_64-unknown-linux-gnu` (Linux x64)
- `aarch64-unknown-linux-gnu` (Linux ARM64)
- `x86_64-pc-windows-msvc` (Windows x64)

## Configuration

git-tidy reads config from `~/.config/git-tidy/config.json`:

```json
{
    "default_path": "P:\\Dev",
    "local_path": "git.local",
    "workspaces": [
        {
            "pattern": ".*ghe\\.com.*",
            "path": "P:\\Dev-work"
        },
        {
            "pattern": ".*github\\.com.*",
            "path": "P:\\Dev"
        }
    ]
}
```

| Key | Default | Purpose |
|-----|---------|---------|
| `default_path` | current directory | Fallback root when no workspace matches |
| `local_path` | `git.local` | Folder for unrecognized dirs (set to `""` to skip) |
| `workspaces` | `[]` | Array of `{pattern, path}` for routing repos by remote URL regex |

## Usage

### Clone a repository

Clone into a structured path (`host/org/repo`), routed by workspace:

```bash
git tidy clone https://github.com/owner/repo.git
# Clones to: P:\Dev\github.com\owner\repo

git tidy clone https://ghe.company.com/org/repo.git
# Clones to: P:\Dev-work\ghe.company.com\org\repo
```

### Organize existing repositories

Move flat repositories into structured paths, with workspace routing:

```bash
git tidy organize
# Organizes repos in default_path

git tidy organize C:\Repos
# Organizes repos in C:\Repos

# Preview changes without moving anything:
git tidy organize --dry-run

# Unrecognized folders are moved to git.local/
# Set local_path to "" to skip this behavior
```

### Shell Completions

Generate shell completions for your shell:

```bash
# Bash
git tidy --completions bash > /etc/bash_completion.d/git-tidy

# Zsh
git tidy --completions zsh > ~/.zfunc/_git-tidy

# Fish
git tidy --completions fish > ~/.config/fish/completions/git-tidy.fish

# PowerShell
git tidy --completions powershell > git-tidy.ps1
```

## License

MIT
