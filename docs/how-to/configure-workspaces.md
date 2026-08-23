# Configure Workspaces

This guide explains how to set up workspace routing for different Git hosts.

## Overview

Workspaces allow you to route repositories to different directories based on their remote URL. This is useful when you work with multiple Git hosts (e.g., GitHub, GitLab, Bitbucket) or need to separate personal and work repositories.

## Configuration File

Create or edit `~/.config/git-tidy/config.json`:

```json
{
    "default_path": "/home/you/dev",
    "local_path": "git.local",
    "workspaces": [
        {
            "pattern": ".*github\\.com.*",
            "path": "/home/you/dev"
        },
        {
            "pattern": ".*gitlab\\.com.*",
            "path": "/home/you/dev"
        },
        {
            "pattern": ".*ghe\\.company\\.com.*",
            "path": "/home/you/work"
        }
    ]
}
```

## Configuration Options

### default_path

The fallback directory when no workspace pattern matches.

```json
{
    "default_path": "/home/you/dev"
}
```

### local_path

Directory for unrecognized folders during organize operations. Set to `""` to disable.

```json
{
    "local_path": "git.local"
}
```

### workspaces

Array of workspace definitions with regex patterns.

```json
{
    "workspaces": [
        {
            "pattern": ".*github\\.com.*",
            "path": "/home/you/dev"
        }
    ]
}
```

Each workspace has:

- **pattern**: Regular expression to match against the remote URL
- **path**: Directory to clone/move the repository into

## Pattern Examples

```json
// Match all GitHub URLs
{ "pattern": ".*github\\.com.*", "path": "/home/you/dev" }

// Match a specific GitHub Enterprise instance
{ "pattern": ".*ghe\\.company\\.com.*", "path": "/home/you/work" }

// Match SSH URLs
{ "pattern": "git@github\\.com:.*", "path": "/home/you/dev" }

// Match HTTPS URLs only
{ "pattern": "https://github\\.com/.*", "path": "/home/you/dev" }
```

## Workspace Resolution

Workspaces are evaluated in order. The first matching pattern wins:

```json
{
    "workspaces": [
        { "pattern": ".*ghe\\.company\\.com.*", "path": "/home/you/work" },
        { "pattern": ".*github\\.com.*", "path": "/home/you/dev" }
    ]
}
```

A URL like `https://ghe.company.com/org/repo` matches the first pattern and goes to `/home/you/work`.

## Verification

Test your configuration by cloning a repository:

```bash
git-tidy clone https://github.com/owner/repo.git
```

The repository should appear in the correct workspace directory.

## Troubleshooting

### Repository goes to default path

- Check that your regex pattern matches the full URL
- Use a regex tester to verify your patterns
- Ensure patterns are in the correct order

### Configuration not loaded

- Verify the file is at `~/.config/git-tidy/config.json`
- Check for JSON syntax errors
- Ensure the file is readable
