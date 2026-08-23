# Configuration Reference

Complete reference for git-tidy configuration.

## Configuration File

git-tidy reads configuration from:

```
~/.config/git-tidy/config.json
```

## Schema

```json
{
    "default_path": "<path>",
    "local_path": "<string>",
    "workspaces": [
        {
            "pattern": "<regex>",
            "path": "<path>"
        }
    ]
}
```

## Options

### default_path

**Type:** `string` (path)
**Default:** Current working directory

The fallback directory when no workspace pattern matches.

```json
{
    "default_path": "/home/you/dev"
}
```

### local_path

**Type:** `string` or `null`
**Default:** `"git.local"`

Directory for unrecognized folders during organize operations. Set to `""` to disable.

```json
{
    "local_path": "git.local"
}
```

When `git-tidy organize` encounters a directory that:

- Is not a Git repository
- Has no valid remote URL

It moves the directory to `<target>/<local_path>/`.

Set to `""` to skip this behavior:

```json
{
    "local_path": ""
}
```

### workspaces

**Type:** `array`
**Default:** `[]`

Array of workspace definitions. Each workspace routes repositories matching a pattern to a specific directory.

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

#### Workspace Object

| Field | Type | Description |
|-------|------|-------------|
| `pattern` | `string` | Regular expression to match against remote URLs |
| `path` | `string` | Directory to clone/move matching repositories into |

## Examples

### Minimal Configuration

```json
{
    "default_path": "/home/you/dev"
}
```

### Multiple Workspaces

```json
{
    "default_path": "/home/you/dev",
    "workspaces": [
        { "pattern": ".*ghe\\.company\\.com.*", "path": "/home/you/work" },
        { "pattern": ".*github\\.com.*", "path": "/home/you/dev" }
    ]
}
```

### Disable Local Path

```json
{
    "default_path": "/home/you/dev",
    "local_path": "",
    "workspaces": [
        { "pattern": ".*github\\.com.*", "path": "/home/you/dev" }
    ]
}
```

### Complete Example

```json
{
    "default_path": "/home/you/dev",
    "local_path": "unsorted",
    "workspaces": [
        { "pattern": ".*ghe\\.company\\.com.*", "path": "/home/you/work" },
        { "pattern": ".*gitlab\\.com.*", "path": "/home/you/dev" },
        { "pattern": ".*github\\.com.*", "path": "/home/you/dev" }
    ]
}
```

## Workspace Resolution

Workspaces are evaluated in order. The first matching pattern wins.

Given this configuration:

```json
{
    "workspaces": [
        { "pattern": ".*ghe\\.company\\.com.*", "path": "/work" },
        { "pattern": ".*github\\.com.*", "path": "/dev" }
    ]
}
```

URLs are resolved as follows:

| URL | Workspace | Path |
|-----|-----------|------|
| `https://ghe.company.com/org/repo` | First (matches) | `/work/ghe.company.com/org/repo` |
| `https://github.com/owner/repo` | Second (matches) | `/dev/github.com/owner/repo` |
| `https://example.com/repo` | None | `default_path/example.com/repo` |

## Regex Patterns

Patterns use Rust regex syntax. Common patterns:

```json
// Match any GitHub URL
".*github\\.com.*"

// Match a specific GitHub Enterprise instance
".*ghe\\.company\\.com.*"

// Match SSH URLs
"git@github\\.com:.*"

// Match HTTPS URLs only
"https://github\\.com/.*"

// Match a specific organization
".*github\\.com/myorg/.*"
```

## Loading Behavior

1. If `~/.config/git-tidy/config.json` does not exist, defaults are used
2. If the file exists but cannot be parsed, a warning is printed and defaults are used
3. Missing fields use their default values

## Verification

Validate your configuration:

```bash
# Check if config is loaded
git-tidy clone https://github.com/owner/repo.git

# Use dry-run to test workspace routing
git-tidy organize --dry-run
```
