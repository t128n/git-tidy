use regex::Regex;

static PROTOCOL_RE: once_cell::sync::Lazy<Regex> =
    once_cell::sync::Lazy::new(|| Regex::new(r"^[a-zA-Z0-9+_.-]+://").unwrap());

static GIT_SUFFIX_RE: once_cell::sync::Lazy<Regex> =
    once_cell::sync::Lazy::new(|| Regex::new(r"\.git$").unwrap());

static TRAILING_SLASH_RE: once_cell::sync::Lazy<Regex> =
    once_cell::sync::Lazy::new(|| Regex::new(r"/+$").unwrap());

/// Normalize a Git remote URL to a relative path like `host/org/repo`.
///
/// Strips protocol prefixes, converts SSH colons to slashes,
/// removes `.git` suffix, and trims trailing slashes.
pub fn normalize_git_url(url: &str) -> String {
    let mut result = url.trim().to_string();

    // Strip protocol prefix (https://, git://, ssh://, etc.)
    result = PROTOCOL_RE.replace(&result, "").to_string();

    // Strip git@ prefix
    if result.starts_with("git@") {
        result = result[4..].to_string();
    }

    // Convert SSH colon to slash (host:path -> host/path)
    if let Some(colon_pos) = result.find(':') {
        result.replace_range(colon_pos..=colon_pos, "/");
    }

    // Remove .git suffix
    result = GIT_SUFFIX_RE.replace(&result, "").to_string();

    // Trim trailing slashes
    result = TRAILING_SLASH_RE.replace(&result, "").to_string();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn https_url() {
        assert_eq!(normalize_git_url("https://github.com/owner/repo.git"), "github.com/owner/repo");
    }

    #[test]
    fn ssh_url() {
        assert_eq!(normalize_git_url("git@github.com:owner/repo.git"), "github.com/owner/repo");
    }

    #[test]
    fn git_protocol() {
        assert_eq!(normalize_git_url("git://github.com/owner/repo.git"), "github.com/owner/repo");
    }

    #[test]
    fn trailing_slash() {
        assert_eq!(normalize_git_url("https://github.com/owner/repo/"), "github.com/owner/repo");
    }

    #[test]
    fn no_git_suffix() {
        assert_eq!(normalize_git_url("https://github.com/owner/repo"), "github.com/owner/repo");
    }

    #[test]
    fn with_whitespace() {
        assert_eq!(normalize_git_url("  https://github.com/owner/repo.git  "), "github.com/owner/repo");
    }
}
