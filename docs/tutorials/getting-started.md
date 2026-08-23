# Getting Started

This tutorial walks you through your first steps with git-tidy.

## Prerequisites

- [git-tidy installed](https://github.com/t128n/git-tidy/releases)
- Git installed and available in your PATH

## Your First Clone

Clone a repository using git-tidy:

```bash
git-tidy clone https://github.com/owner/repo.git
```

This creates a structured path:

```
<workspace>/
└── github.com/
    └── owner/
        └── repo/
```

## Understanding Workspace Routing

By default, repositories are cloned to the current directory. Configure workspaces to route different hosts to different locations.

Create a configuration file at `~/.config/git-tidy/config.json`:

```json
{
    "default_path": "/home/you/dev",
    "workspaces": [
        {
            "pattern": ".*github\\.com.*",
            "path": "/home/you/dev"
        },
        {
            "pattern": ".*ghe\\.company\\.com.*",
            "path": "/home/you/work"
        }
    ]
}
```

Now clone repositories and they'll be routed automatically:

```bash
# Goes to /home/you/dev/github.com/owner/repo
git-tidy clone https://github.com/owner/repo.git

# Goes to /home/you/work/ghe.company.com/org/repo
git-tidy clone https://ghe.company.com/org/repo.git
```

## Organizing Existing Repositories

If you have repositories scattered in a flat directory, use the organize command:

```bash
git-tidy organize /path/to/repos
```

This moves each repository into its structured path based on the remote URL.

### Dry Run

Preview changes without moving anything:

```bash
git-tidy organize --dry-run
```

## Next Steps

- Learn more about [workspace configuration](../how-to/configure-workspaces.md)
- See the complete [CLI reference](../reference/cli.md)
- Understand the [architecture](../explanation/architecture.md)
