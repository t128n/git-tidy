# Organize Repositories

This guide explains how to organize existing flat repositories into structured paths.

## Overview

If you have repositories scattered in a flat directory, `git-tidy organize` moves them into structured `host/org/repo` paths based on their remote URLs.

## Basic Usage

Organize repositories in the default path:

```bash
git-tidy organize
```

Organize repositories in a specific directory:

```bash
git-tidy organize /path/to/repos
```

## Dry Run

Preview changes without moving anything:

```bash
git-tidy organize --dry-run
```

This shows what would be moved:

```
[dry-run] Would move: my-project -> github.com/owner/my-project
[dry-run] Would move: another-repo -> gitlab.com/group/another-repo
```

## How It Works

1. Scans the target directory for Git repositories
2. Reads each repository's remote origin URL
3. Normalizes the URL to a `host/org/repo` path
4. Moves the repository to the appropriate workspace

## Unrecognized Directories

Directories without a valid Git remote are moved to the `local_path` (default: `git.local`).

To disable this behavior, set `local_path` to `""` in your configuration:

```json
{
    "local_path": ""
}
```

## Examples

### Basic Organization

Before:

```
/repos/
├── my-project/       (remote: https://github.com/owner/my-project.git)
├── work-api/         (remote: https://ghe.company.com/team/api.git)
└── random-folder/    (not a git repository)
```

After running `git-tidy organize /repos`:

```
/dev/
├── github.com/
│   └── owner/
│       └── my-project/
/work/
└── ghe.company.com/
    └── team/
        └── api/
/repos/
└── git.local/
    └── random-folder/
```

### With Workspace Configuration

Given this configuration:

```json
{
    "default_path": "/home/you/dev",
    "workspaces": [
        { "pattern": ".*ghe\\.company\\.com.*", "path": "/home/you/work" }
    ]
}
```

Running `git-tidy organize` routes repositories accordingly:

- GitHub repositories go to `/home/you/dev/github.com/...`
- Company repositories go to `/home/you/work/ghe.company.com/...`

## Error Handling

- **Path does not exist**: Verify the directory path is correct
- **Target already exists**: The repository won't be moved if the target path exists
- **Permission denied**: Ensure you have write permissions to both source and target directories

## Next Steps

- Learn about [workspace configuration](configure-workspaces.md)
- See the [CLI reference](../reference/cli.md)
