use anyhow::{Result, anyhow};
use regex::Regex;
use std::sync::LazyLock;
use url::Url;

/// Regex for validating Git URLs.
///
/// This regex matches the following URL formats:
/// - Full URLs with protocol: `https://github.com/user/repo.git`
/// - SSH URLs: `git@github.com:user/repo.git`
/// - URLs without protocol: `github.com/user/repo.git`
/// - URLs without .git suffix: `github.com/user/repo`
pub static GIT_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:(?:https?|git|ssh)://)?(?:[a-zA-Z0-9._-]+@)?[a-zA-Z0-9.-]+(?::[0-9]+)?[:/][a-zA-Z0-9._/-]+(?:\.git)?/?$").unwrap()
});

/// Normalizes a Git URL from a string to a canonical form suitable for storing in manifest files.
///
/// This function can handle various Git URL formats including:
/// - Full URLs with protocol: `https://github.com/user/repo.git`
/// - SSH URLs: `git@github.com:user/repo.git`
/// - URLs without protocol: `github.com/user/repo.git`
/// - URLs without .git suffix: `github.com/user/repo`
///
/// # Examples
///
/// ```
/// use moss_git::url::normalize_git_url;
///
/// assert_eq!(normalize_git_url("https://github.com/user/repo.git").unwrap(), "github.com/user/repo");
/// assert_eq!(normalize_git_url("git@github.com:user/repo.git").unwrap(), "github.com/user/repo");
/// assert_eq!(normalize_git_url("github.com/user/repo").unwrap(), "github.com/user/repo");
/// ```
pub fn normalize_git_url(url_str: &str) -> Result<String> {
    // Try to parse as-is first (for full URLs with protocol)
    if let Ok(url) = Url::parse(url_str) {
        return normalize_git_url_internal(&url);
    }

    // Handle SSH format like git@github.com:user/repo.git
    if url_str.contains('@') && url_str.contains(':') && !url_str.starts_with("http") {
        // Convert SSH format to a parseable URL
        if let Some(at_pos) = url_str.find('@') {
            if let Some(colon_pos) = url_str[at_pos..].find(':') {
                let host = &url_str[at_pos + 1..at_pos + colon_pos];
                let path = &url_str[at_pos + colon_pos + 1..];

                // Create a fake SSH URL that can be parsed
                let ssh_url = format!("ssh://{}@{}/{}", &url_str[..at_pos], host, path);
                if let Ok(url) = Url::parse(&ssh_url) {
                    return normalize_git_url_internal(&url);
                }
            }
        }
    }

    // Try adding https:// protocol
    let https_url = if url_str.starts_with("//") {
        format!("https:{}", url_str)
    } else {
        format!("https://{}", url_str)
    };

    if let Ok(url) = Url::parse(&https_url) {
        return normalize_git_url_internal(&url);
    }

    // If all else fails, try to parse it as a simple domain/path format
    if url_str.contains('/') {
        let parts: Vec<&str> = url_str.splitn(2, '/').collect();
        if parts.len() == 2 {
            let domain = parts[0];
            let mut path = parts[1];

            // Remove .git suffix if present
            if path.ends_with(".git") {
                path = &path[..path.len() - 4];
            }

            // Clean up multiple slashes
            let path_parts: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
            if path_parts.is_empty() {
                return Err(anyhow!("Invalid path: no repository path found"));
            }
            let cleaned_path = path_parts.join("/");

            return Ok(format!("{}/{}", domain, cleaned_path));
        }
    }

    Err(anyhow!("Unable to parse Git URL: {}", url_str))
}

/// Internal function that normalizes a parsed Url object.
/// This function strips the protocol, port (if default), and ".git" suffix from Git URLs.
/// It supports various Git URL formats including HTTPS, SSH, and git protocol.
fn normalize_git_url_internal(repo_url: &Url) -> Result<String> {
    let host = get_normalized_host(repo_url)?;
    let path = get_normalized_path(repo_url.path())?;

    Ok(format!("{}{}", host, path))
}

/// Extracts and normalizes the host part of a Git URL.
/// For SSH URLs, this handles the special case where the domain might be embedded in the path.
fn get_normalized_host(repo_url: &Url) -> Result<String> {
    // Handle SSH URLs like git@github.com:user/repo.git
    if repo_url.scheme() == "ssh" && repo_url.host_str().is_none() {
        // For URLs like ssh://git@github.com/user/repo.git, the host is properly parsed
        // For URLs like git@github.com:user/repo.git, we need special handling
        let path = repo_url.path();
        if let Some(at_pos) = path.find('@') {
            if let Some(colon_pos) = path[at_pos..].find(':') {
                let host_part = &path[at_pos + 1..at_pos + colon_pos];
                if !host_part.is_empty() {
                    return Ok(host_part.to_string());
                }
            }
        }
    }

    let host = repo_url
        .host_str()
        .ok_or_else(|| anyhow!("No host found in URL: {}", repo_url))?;

    // Only include port if it's not a default port for the scheme
    match (repo_url.scheme(), repo_url.port()) {
        ("https", Some(443)) | ("http", Some(80)) | ("ssh", Some(22)) | ("git", Some(9418)) => {
            Ok(host.to_string())
        }
        (_, Some(port)) => Ok(format!("{}:{}", host, port)),
        (_, None) => Ok(host.to_string()),
    }
}

/// Normalizes the path component of a Git URL.
/// Removes ".git" suffix and cleans up multiple slashes.
fn get_normalized_path(path: &str) -> Result<String> {
    if path.is_empty() {
        return Err(anyhow!("Empty path in Git URL"));
    }

    let mut normalized_path = path.to_string();

    // Handle SSH URLs where path might contain the host part
    if let Some(colon_pos) = path.find(':') {
        // For SSH URLs like git@github.com:user/repo.git, extract path after colon
        if colon_pos > 0 && path[..colon_pos].contains('@') {
            normalized_path = path[colon_pos + 1..].to_string();
        }
    }

    // Remove .git suffix
    if normalized_path.ends_with(".git") {
        normalized_path = normalized_path[..normalized_path.len() - 4].to_string();
    }

    // Ensure path starts with /
    if !normalized_path.starts_with('/') {
        normalized_path = format!("/{}", normalized_path);
    }

    // Clean up multiple slashes and remove trailing slash
    let parts: Vec<&str> = normalized_path
        .split('/')
        .filter(|part| !part.is_empty())
        .collect();

    if parts.is_empty() {
        return Err(anyhow!("Invalid path: no repository path found"));
    }

    Ok(format!("/{}", parts.join("/")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_https_github_with_git_suffix() {
        let result = normalize_git_url("https://github.com/moss-foundation/sapic.git").unwrap();
        assert_eq!(result, "github.com/moss-foundation/sapic");
    }

    #[test]
    fn test_https_github_without_git_suffix() {
        let result = normalize_git_url("https://github.com/moss-foundation/sapic").unwrap();
        assert_eq!(result, "github.com/moss-foundation/sapic");
    }

    #[test]
    fn test_ssh_github_with_git_suffix() {
        let result = normalize_git_url("ssh://git@github.com/moss-foundation/sapic.git").unwrap();
        assert_eq!(result, "github.com/moss-foundation/sapic");
    }

    #[test]
    fn test_ssh_github_without_git_suffix() {
        let result = normalize_git_url("ssh://git@github.com/moss-foundation/sapic").unwrap();
        assert_eq!(result, "github.com/moss-foundation/sapic");
    }

    #[test]
    fn test_git_protocol() {
        let result = normalize_git_url("git://github.com/moss-foundation/sapic.git").unwrap();
        assert_eq!(result, "github.com/moss-foundation/sapic");
    }

    #[test]
    fn test_gitlab_https() {
        let result = normalize_git_url("https://gitlab.com/user/project.git").unwrap();
        assert_eq!(result, "gitlab.com/user/project");
    }

    #[test]
    fn test_self_hosted_gitlab_with_custom_port() {
        let result = normalize_git_url("https://git.example.com:8443/user/project.git").unwrap();
        assert_eq!(result, "git.example.com:8443/user/project");
    }

    #[test]
    fn test_ssh_with_custom_port() {
        let result = normalize_git_url("ssh://git@git.example.com:2222/user/project.git").unwrap();
        assert_eq!(result, "git.example.com:2222/user/project");
    }

    #[test]
    fn test_https_default_port_stripped() {
        let result = normalize_git_url("https://github.com:443/user/repo.git").unwrap();
        assert_eq!(result, "github.com/user/repo");
    }

    #[test]
    fn test_ssh_default_port_stripped() {
        let result = normalize_git_url("ssh://git@github.com:22/user/repo.git").unwrap();
        assert_eq!(result, "github.com/user/repo");
    }

    #[test]
    fn test_nested_repository_path() {
        let result =
            normalize_git_url("https://git.example.com/group/subgroup/project.git").unwrap();
        assert_eq!(result, "git.example.com/group/subgroup/project");
    }

    #[test]
    fn test_path_with_multiple_slashes() {
        let result = normalize_git_url("https://github.com//user//repo//.git").unwrap();
        assert_eq!(result, "github.com/user/repo");
    }

    #[test]
    fn test_bitbucket_ssh() {
        let result = normalize_git_url("ssh://git@bitbucket.org/user/repo.git").unwrap();
        assert_eq!(result, "bitbucket.org/user/repo");
    }

    #[test]
    fn test_codeberg_https() {
        let result = normalize_git_url("https://codeberg.org/user/repo.git").unwrap();
        assert_eq!(result, "codeberg.org/user/repo");
    }

    #[test]
    fn test_sourceforge_git() {
        let result = normalize_git_url("git://git.code.sf.net/p/project/code").unwrap();
        assert_eq!(result, "git.code.sf.net/p/project/code");
    }

    #[test]
    fn test_error_no_host() {
        let result = normalize_git_url("file:///local/repo.git");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No host found"));
    }

    #[test]
    fn test_error_empty_path() {
        let result = normalize_git_url("https://github.com");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("no repository path found")
        );
    }

    #[test]
    fn test_error_only_slash_path() {
        let result = normalize_git_url("https://github.com/");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("no repository path found")
        );
    }

    #[test]
    fn test_azure_devops() {
        let result =
            normalize_git_url("https://dev.azure.com/organization/project/_git/repository")
                .unwrap();
        assert_eq!(result, "dev.azure.com/organization/project/_git/repository");
    }

    #[test]
    fn test_aws_codecommit() {
        let result =
            normalize_git_url("https://git-codecommit.us-east-1.amazonaws.com/v1/repos/my-repo")
                .unwrap();
        assert_eq!(
            result,
            "git-codecommit.us-east-1.amazonaws.com/v1/repos/my-repo"
        );
    }

    #[test]
    fn test_google_cloud_source() {
        let result =
            normalize_git_url("https://source.developers.google.com/p/project/r/repository")
                .unwrap();
        assert_eq!(
            result,
            "source.developers.google.com/p/project/r/repository"
        );
    }

    // Edge cases and regression tests

    #[test]
    fn test_case_sensitive_preservation() {
        let result = normalize_git_url("https://GitHub.com/User/Repo.git").unwrap();
        // URL parsing automatically lowercases the domain
        assert_eq!(result, "github.com/User/Repo");
    }

    #[test]
    fn test_unicode_in_path() {
        let result = normalize_git_url("https://github.com/user/репозиторий.git").unwrap();
        // URL parser automatically percent-encodes Unicode characters
        assert_eq!(
            result,
            "github.com/user/%D1%80%D0%B5%D0%BF%D0%BE%D0%B7%D0%B8%D1%82%D0%BE%D1%80%D0%B8%D0%B9"
        );
    }

    #[test]
    fn test_hyphen_and_underscore_in_names() {
        let result = normalize_git_url("https://github.com/my-org/my_project-name.git").unwrap();
        assert_eq!(result, "github.com/my-org/my_project-name");
    }

    #[test]
    fn test_single_character_names() {
        let result = normalize_git_url("https://github.com/a/b.git").unwrap();
        assert_eq!(result, "github.com/a/b");
    }

    #[test]
    fn test_very_long_path() {
        let long_path = "a".repeat(100);
        let url_string = format!("https://github.com/user/{}.git", long_path);
        let result = normalize_git_url(&url_string).unwrap();
        assert_eq!(result, format!("github.com/user/{}", long_path));
    }

    // Tests for normalize_git_url_from_str function

    #[test]
    fn test_from_str_full_https_url() {
        let result = normalize_git_url("https://github.com/user/repo.git").unwrap();
        assert_eq!(result, "github.com/user/repo");
    }

    #[test]
    fn test_from_str_ssh_format() {
        let result = normalize_git_url("git@github.com:user/repo.git").unwrap();
        assert_eq!(result, "github.com/user/repo");
    }

    #[test]
    fn test_from_str_ssh_format_without_git_suffix() {
        let result = normalize_git_url("git@github.com:user/repo").unwrap();
        assert_eq!(result, "github.com/user/repo");
    }

    #[test]
    fn test_from_str_url_without_protocol() {
        let result = normalize_git_url("github.com/user/repo.git").unwrap();
        assert_eq!(result, "github.com/user/repo");
    }

    #[test]
    fn test_from_str_url_without_protocol_and_git_suffix() {
        let result = normalize_git_url("github.com/user/repo").unwrap();
        assert_eq!(result, "github.com/user/repo");
    }

    #[test]
    fn test_from_str_gitlab_ssh() {
        let result = normalize_git_url("git@gitlab.com:group/project.git").unwrap();
        assert_eq!(result, "gitlab.com/group/project");
    }

    #[test]
    fn test_from_str_bitbucket_without_protocol() {
        let result = normalize_git_url("bitbucket.org/user/repo.git").unwrap();
        assert_eq!(result, "bitbucket.org/user/repo");
    }

    #[test]
    fn test_from_str_nested_path() {
        let result = normalize_git_url("gitlab.example.com/group/subgroup/project").unwrap();
        assert_eq!(result, "gitlab.example.com/group/subgroup/project");
    }

    #[test]
    fn test_from_str_azure_devops_ssh() {
        let result = normalize_git_url("git@ssh.dev.azure.com:v3/org/project/repo").unwrap();
        assert_eq!(result, "ssh.dev.azure.com/v3/org/project/repo");
    }

    #[test]
    fn test_from_str_scheme_relative_url() {
        let result = normalize_git_url("//github.com/user/repo.git").unwrap();
        assert_eq!(result, "github.com/user/repo");
    }

    #[test]
    fn test_from_str_path_with_dots() {
        let result = normalize_git_url("github.com/my.user/my.repo.name.git").unwrap();
        assert_eq!(result, "github.com/my.user/my.repo.name");
    }

    #[test]
    fn test_from_str_single_level_path() {
        let result = normalize_git_url("github.com/repo").unwrap();
        assert_eq!(result, "github.com/repo");
    }

    #[test]
    fn test_from_str_error_invalid_format() {
        let result = normalize_git_url("not-a-valid-url");
        assert!(result.is_err());
        // Could be either "Unable to parse Git URL" or "Invalid path: no repository path found"
        // Both are valid error messages for invalid formats
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_error_empty_string() {
        let result = normalize_git_url("");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_codeberg_ssh() {
        let result = normalize_git_url("git@codeberg.org:user/project.git").unwrap();
        assert_eq!(result, "codeberg.org/user/project");
    }

    #[test]
    fn test_from_str_sourceforge_format() {
        let result = normalize_git_url("git.code.sf.net/p/project/code").unwrap();
        assert_eq!(result, "git.code.sf.net/p/project/code");
    }

    #[test]
    fn test_from_str_ssh_with_different_user() {
        let result = normalize_git_url("deploy@git.company.com:repos/project.git").unwrap();
        assert_eq!(result, "git.company.com/repos/project");
    }

    #[test]
    fn test_from_str_preserves_case_in_path() {
        let result = normalize_git_url("GitHub.com/User/Repo.git").unwrap();
        assert_eq!(result, "github.com/User/Repo");
    }

    #[test]
    fn test_from_str_multiple_slashes_cleanup() {
        let result = normalize_git_url("github.com//user//repo//.git").unwrap();
        assert_eq!(result, "github.com/user/repo");
    }
}
