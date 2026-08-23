# Contributing to git-tidy

Thank you for your interest in contributing to git-tidy! This document provides guidelines and information for contributors.

## Getting Started

1. Fork the repository
2. Clone your fork
3. Create a new branch for your changes
4. Make your changes
5. Submit a pull request

## Development Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [mise](https://mise.jdx.dev/) (for managing tool versions)
- [Bun](https://bun.sh/) (for changesets)

### Setup

```bash
# Clone your fork
git clone https://github.com/<your-username>/git-tidy.git
cd git-tidy

# Install tool versions via mise
mise install

# Install dependencies
bun install

# Install cargo-edit for version sync
cargo install cargo-edit

# Build the project
cargo build
```

## Changesets

This project uses [changesets](https://github.com/changesets/changesets) for version management. When making changes that should trigger a release, you need to add a changeset.

### Adding a Changeset

```bash
bunx changeset
```

Follow the prompts to describe your changes. This will create a markdown file in the `.changeset/` directory.

### Versioning

- **patch**: Bug fixes and minor improvements
- **minor**: New features that are backward-compatible
- **major**: Breaking changes

## Pull Request Process

1. Ensure your code follows the existing style
2. Update documentation if needed
3. Add tests for new functionality
4. Add a changeset if applicable
5. Ensure all CI checks pass
6. Request a review from a maintainer

## Code Style

- Follow standard Rust conventions
- Use `cargo fmt` to format code
- Use `cargo clippy` to lint code
- Keep commits atomic and well-described

## Reporting Issues

- Use the GitHub issue templates when available
- Include reproduction steps for bugs
- Check existing issues before creating new ones

## Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

## Security

For security vulnerabilities, please see our [Security Policy](SECURITY.md).
