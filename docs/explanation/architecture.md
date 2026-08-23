# Architecture

This document explains how git-tidy works internally.

## Overview

git-tidy is a Rust CLI tool that provides two main operations:

1. **Clone** - Clones repositories into structured paths
2. **Organize** - Moves existing repositories into structured paths

## Design Principles

- **Simple configuration** - JSON-based config, easy to understand
- **Regex-based routing** - Flexible workspace matching
- **Graceful degradation** - Works without configuration
- **Dry-run support** - Preview changes before applying

## Components

### Main Entry Point (`main.rs`)

Parses CLI arguments and dispatches to the appropriate command handler.

```
CLI Parsing → Command Dispatch → Execution
```

### Configuration (`config.rs`)

Handles loading and parsing the configuration file.

```
Config File → Parse JSON → Config Struct
```

Features:
- Loads from `~/.config/git-tidy/config.json`
- Falls back to defaults if file doesn't exist
- Supports partial configuration (missing fields use defaults)

### Git Operations (`git.rs`)

Wraps Git CLI commands for cloning and reading remote URLs.

```
git clone → Repository
git config --get remote.origin.url → URL
```

### URL Normalization (`normalize.rs`)

Converts Git remote URLs to structured paths.

```
https://github.com/owner/repo.git → github.com/owner/repo
git@github.com:owner/repo.git → github.com/owner/repo
git://github.com/owner/repo.git → github.com/owner/repo
```

**Steps:**
1. Strip protocol prefix
2. Strip `git@` prefix
3. Convert SSH colon to slash
4. Remove `.git` suffix
5. Trim trailing slashes

### Organization (`organize.rs`)

Handles moving repositories into structured paths.

```
Scan Directory → Find Git Repos → Read Remote URL → Move to Target
```

**Algorithm:**
1. Scan target directory for Git repositories
2. For each repository, read remote origin URL
3. Normalize URL to structured path
4. Resolve workspace (first matching pattern)
5. Move repository to target path

## Data Flow

### Clone Operation

```
User Input (URL) → Parse URL → Resolve Workspace → Clone to Target
```

### Organize Operation

```
Directory Scan → Find Git Repos → Read Remote URLs → Normalize URLs → Resolve Workspaces → Move Repositories
```

## URL Normalization Examples

| Input | Output |
|-------|--------|
| `https://github.com/owner/repo.git` | `github.com/owner/repo` |
| `git@github.com:owner/repo.git` | `github.com/owner/repo` |
| `git://github.com/owner/repo.git` | `github.com/owner/repo` |
| `https://github.com/owner/repo/` | `github.com/owner/repo` |
| `  https://github.com/owner/repo.git  ` | `github.com/owner/repo` |

## Workspace Resolution

Workspaces are evaluated in order:

```
URL → Pattern 1 (no match) → Pattern 2 (match) → Return Workspace Path
                                                   ↓
                              If no match → Return default_path
```

## Error Handling

git-tidy uses `anyhow` for error handling:

- **Clone failures** - Git clone command fails
- **Target exists** - Repository already at target path
- **Permission denied** - Cannot read/write directories
- **Invalid config** - Configuration file cannot be parsed

Errors are printed to stderr and the process exits with code 1.

## Performance

- **Clone** - Limited by Git clone speed
- **Organize** - Fast filesystem operations, no Git operations except reading remote URLs

## Platform Support

- **Linux** - Full support
- **macOS** - Full support
- **Windows** - Full support (path separators are normalized)

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `anyhow` | Error handling |
| `regex` | Pattern matching |
| `serde` | JSON deserialization |
| `dirs` | Home directory detection |
| `once_cell` | Lazy static regex compilation |

## Testing

The project includes unit tests for:

- URL normalization
- Configuration parsing
- Workspace resolution

Run tests:

```bash
cargo test
```
