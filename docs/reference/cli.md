# CLI Reference

Complete reference for the git-tidy command-line interface.

## Synopsis

```bash
git-tidy [COMMAND] [OPTIONS]
git-tidy [--completions <SHELL>]
git-tidy [--help] [--version]
```

## Commands

### clone

Clone a repository into a structured `host/org/repo` path.

```bash
git-tidy clone <URL> [ROOT]
```

**Arguments:**

- `<URL>` - The remote URL of the repository to clone
- `[ROOT]` - Root directory to clone into (overrides workspace routing)

**Examples:**

```bash
git-tidy clone https://github.com/owner/repo.git
git-tidy clone git@github.com:owner/repo.git
git-tidy clone https://ghe.company.com/org/repo.git /custom/path
```

**URL Handling:**

- Strips protocol prefixes (`https://`, `git://`, `ssh://`)
- Converts SSH colons to slashes (`git@host:path` → `host/path`)
- Removes `.git` suffix
- Trims trailing slashes

**Output:**

```
github.com/owner/repo
```

Creates the directory structure:

```
<workspace>/github.com/owner/repo/
```

### organize

Move flat repositories into structured paths.

```bash
git-tidy organize [PATH] [--dry-run]
```

**Arguments:**

- `[PATH]` - Directory containing repositories to organize (defaults to `default_path`)

**Options:**

- `--dry-run` - Preview what would be moved without making changes

**Examples:**

```bash
git-tidy organize
git-tidy organize /path/to/repos
git-tidy organize --dry-run
```

**Behavior:**

1. Scans for Git repositories (directories containing `.git`)
2. Reads `remote.origin.url` for each repository
3. Normalizes the URL to a structured path
4. Moves repositories to the appropriate workspace
5. Moves unrecognized directories to `local_path` (if configured)

### help

Print usage information.

```bash
git-tidy help
```

## Global Options

### --completions

Generate shell completions for the specified shell.

```bash
git-tidy --completions <SHELL>
```

**Supported shells:**

- `bash`
- `zsh`
- `fish`
- `powershell`
- `elvish`

### --help

Print help information.

### --version

Print version information.

## Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | Error (clone failed, target exists, etc.) |

## Environment

- **HOME** - Used to find configuration at `~/.config/git-tidy/config.json`
- **PWD** - Used as default when no path is specified

## Examples

### Clone with workspace routing

```bash
# With default config, clones to <default_path>/github.com/owner/repo
git-tidy clone https://github.com/owner/repo.git
```

### Clone with custom root

```bash
# Overrides workspace routing
git-tidy clone https://github.com/owner/repo.git /custom/path
```

### Organize with dry run

```bash
# Preview changes
git-tidy organize --dry-run
```

### Generate and install completions

```bash
# Generate completions
git-tidy --completions bash > ~/.local/share/bash-completion/completions/git-tidy
```
