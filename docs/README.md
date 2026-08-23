# git-tidy

Git workspace organizer and cloning utility.

## Overview

git-tidy helps you maintain a clean and organized Git workspace by:

- Cloning repositories into structured `host/org/repo` paths
- Organizing existing flat repositories into structured paths
- Routing repositories to different workspaces based on remote URL patterns

## Quick Start

```bash
# Clone a repository
git-tidy clone https://github.com/owner/repo.git

# Organize existing repositories
git-tidy organize
```

## Documentation

### [Tutorials](tutorials/getting-started.md)

Learn the basics of git-tidy step by step.

- [Getting Started](tutorials/getting-started.md) - Your first steps with git-tidy

### [How-to Guides](how-to/configure-workspaces.md)

Practical guides for common tasks.

- [Configure Workspaces](how-to/configure-workspaces.md) - Set up workspace routing
- [Organize Repositories](how-to/organize-repositories.md) - Organize your existing repos
- [Shell Completions](how-to/shell-completions.md) - Enable tab completion

### [Reference](reference/cli.md)

Complete documentation of all features.

- [CLI Reference](reference/cli.md) - Command-line interface
- [Configuration Reference](reference/configuration.md) - Configuration options

### [Explanation](explanation/architecture.md)

Understanding how git-tidy works.

- [Architecture](explanation/architecture.md) - Design and implementation details

## Installation

Download pre-built binaries from [GitHub Releases](https://github.com/t128n/git-tidy/releases).

## License

MIT
