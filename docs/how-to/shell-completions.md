# Shell Completions

This guide explains how to enable tab completion for git-tidy in your shell.

## Overview

git-tidy can generate shell completions for Bash, Zsh, Fish, PowerShell, and Elvish.

## Generate Completions

### Bash

```bash
git-tidy --completions bash > /etc/bash_completion.d/git-tidy
```

Or for user-local installation:

```bash
git-tidy --completions bash > ~/.local/share/bash-completion/completions/git-tidy
```

### Zsh

```bash
git-tidy --completions zsh > ~/.zfunc/_git-tidy
```

Add to your `~/.zshrc`:

```bash
fpath=(~/.zfunc $fpath)
autoload -Uz compinit
compinit
```

### Fish

```bash
git-tidy --completions fish > ~/.config/fish/completions/git-tidy.fish
```

### PowerShell

```powershell
git-tidy --completions powershell > git-tidy.ps1
```

Then import in your PowerShell profile:

```powershell
. .\git-tidy.ps1
```

### Elvish

```bash
git-tidy --completions elvish > ~/.elvish/lib/git-tidy.elv
```

## Verify

After installing completions, restart your shell and test:

```bash
git-tidy <TAB>
```

You should see available commands and options.

## Troubleshooting

### Completions not working

- Ensure the completions file is in the correct location for your shell
- Restart your shell after installing
- Check that git-tidy is in your PATH

### Permission denied

Use `sudo` for system-wide installation, or install to a user-local directory.
