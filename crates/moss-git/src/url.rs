use joinerror::{Error, errors};
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

errors! {
    /// The provided URL string is empty
    EmptyUrlString => "empty_url_string",

    /// No host found in the URL
    NoHostFound => "no_host_found",

    /// No repository path found in the URL
    NoRepositoryPath => "no_repository_path",

    /// Invalid SSH URL format
    InvalidSshFormat => "invalid_ssh_format",

    /// Invalid URL format - missing path separator
    InvalidUrlFormat => "invalid_url_format",

    /// Repository name cannot be empty
    EmptyRepositoryName => "empty_repository_name",

    /// Host cannot be empty when specified
    EmptyHost => "empty_host",

    /// Owner cannot be empty when specified
    EmptyOwner => "empty_owner",

    /// Invalid character sequences in name or owner
    InvalidSequences => "invalid_sequences",

    /// Host not available for normalization
    NoHostForNormalization => "no_host_for_normalization",
}

#[derive(Debug, Clone, PartialEq)]
pub struct GitUrl {
    /// The fully qualified domain name (FQDN) or IP of the repo
    pub host: Option<String>,
    /// The name of the repo
    pub name: String,
    /// The owner/account/project name
    pub owner: String,
    /// The non-conventional port where git service is hosted
    pub port: Option<u16>,
    /// Indicate if url uses the .git suffix
    pub git_suffix: bool,
}

impl GitUrl {
    /// Parse a Git URL from various formats and extract its components
    ///
    /// Supports:
    /// - HTTPS URLs: `https://github.com/user/repo.git`
    /// - SSH URLs: `git@github.com:user/repo.git` or `ssh://git@github.com:user/repo.git`
    /// - Git protocol: `git://github.com/user/repo.git`
    /// - Simplified format: `github.com/user/repo`
    /// - URLs with custom ports: `git.example.com:8080/user/repo`
    pub fn parse(url: &str) -> joinerror::Result<Self> {
        if url.is_empty() {
            return Err(Error::new::<EmptyUrlString>(
                "provided URL string is empty".to_string(),
            ));
        }

        // Try to parse as-is first (for full URLs with protocol)
        if let Ok(parsed_url) = Url::parse(url) {
            return Self::parse_from_url(&parsed_url);
        }

        // Handle SSH format like git@github.com:user/repo.git
        if url.contains('@') && url.contains(':') && !url.starts_with("http") {
            return Self::parse_ssh_format(url);
        }

        // Try adding https:// protocol
        let https_url = if url.starts_with("//") {
            format!("https:{}", url)
        } else {
            format!("https://{}", url)
        };

        if let Ok(parsed_url) = Url::parse(&https_url) {
            return Self::parse_from_url(&parsed_url);
        }

        // Try to parse as simple domain/path format
        Self::parse_simple_format(url)
    }

    /// Parse from a standard URL object
    fn parse_from_url(url: &Url) -> joinerror::Result<Self> {
        let host = url
            .host_str()
            .ok_or_else(|| Error::new::<NoHostFound>("no host found in URL".to_string()))?;

        let mut host_with_port = host.to_string();

        // Include non-default ports
        if let Some(port) = url.port() {
            let is_default_port = match url.scheme() {
                "https" => port == 443,
                "http" => port == 80,
                "ssh" => port == 22,
                "git" => port == 9418,
                _ => false,
            };

            if !is_default_port {
                host_with_port = format!("{}:{}", host, port);
            }
        }

        let path = url.path();
        if path.is_empty() || path == "/" {
            return Err(Error::new::<NoRepositoryPath>(
                "no repository path found".to_string(),
            ));
        }

        let (owner, name, git_suffix) = Self::parse_path_components(path)?;
        let owner = owner.ok_or_else(|| {
            Error::new::<NoRepositoryPath>("no owner found in URL path".to_string())
        })?;

        Ok(GitUrl {
            host: Some(host_with_port),
            name,
            owner,
            port: url.port(),
            git_suffix,
        })
    }

    /// Parse SSH format like git@github.com:user/repo.git
    fn parse_ssh_format(url: &str) -> joinerror::Result<Self> {
        if let Some(at_pos) = url.find('@') {
            if let Some(colon_pos) = url[at_pos..].find(':') {
                let host = &url[at_pos + 1..at_pos + colon_pos];
                let path = &url[at_pos + colon_pos + 1..];

                if host.is_empty() || path.is_empty() {
                    return Err(Error::new::<InvalidSshFormat>(
                        "invalid SSH URL format".to_string(),
                    ));
                }

                let (owner, name, git_suffix) = Self::parse_path_components(&format!("/{}", path))?;
                let owner = owner.ok_or_else(|| {
                    Error::new::<NoRepositoryPath>("no owner found in SSH URL path".to_string())
                })?;

                Ok(GitUrl {
                    host: Some(host.to_string()),
                    name,
                    owner,
                    port: None,
                    git_suffix,
                })
            } else {
                Err(Error::new::<InvalidSshFormat>(
                    "invalid SSH URL format: missing colon".to_string(),
                ))
            }
        } else {
            Err(Error::new::<InvalidSshFormat>(
                "invalid SSH URL format: missing @".to_string(),
            ))
        }
    }

    /// Parse simple format like github.com/user/repo
    fn parse_simple_format(url: &str) -> joinerror::Result<Self> {
        if !url.contains('/') {
            return Err(Error::new::<InvalidUrlFormat>(
                "invalid format: no path separator found".to_string(),
            ));
        }

        let parts: Vec<&str> = url.splitn(2, '/').collect();
        if parts.len() != 2 {
            return Err(Error::new::<InvalidUrlFormat>(
                "invalid format: expected host/path".to_string(),
            ));
        }

        let host = parts[0];
        let path = parts[1];

        if host.is_empty() || path.is_empty() {
            return Err(Error::new::<InvalidUrlFormat>(
                "invalid format: empty host or path".to_string(),
            ));
        }

        let (owner, name, git_suffix) = Self::parse_path_components(&format!("/{}", path))?;
        let owner = owner.ok_or_else(|| {
            Error::new::<NoRepositoryPath>("no owner found in simple URL path".to_string())
        })?;

        Ok(GitUrl {
            host: Some(host.to_string()),
            name,
            owner,
            port: None,
            git_suffix,
        })
    }

    /// Parse path components to extract owner, name, and git suffix
    fn parse_path_components(path: &str) -> joinerror::Result<(Option<String>, String, bool)> {
        let mut path = path.to_string();

        // Remove leading slash
        if path.starts_with('/') {
            path = path[1..].to_string();
        }

        // Check for .git suffix
        let git_suffix = path.ends_with(".git");
        if git_suffix {
            path = path[..path.len() - 4].to_string();
        }

        // Clean up multiple slashes
        let parts: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();

        if parts.is_empty() {
            return Err(Error::new::<NoRepositoryPath>(
                "no repository path found".to_string(),
            ));
        }

        match parts.len() {
            1 => {
                // Single part: just repository name, no owner
                Ok((None, parts[0].to_string(), git_suffix))
            }
            2 => {
                // Two parts: owner/repo
                Ok((Some(parts[0].to_string()), parts[1].to_string(), git_suffix))
            }
            _ => {
                // Multiple parts: treat last as repo name, everything else as owner/group path
                let name = parts.last().unwrap().to_string();
                let owner = parts[..parts.len() - 1].join("/");
                Ok((Some(owner), name, git_suffix))
            }
        }
    }

    /// Normalize the Git URL to a canonical form (domain/owner/repo format)
    pub fn normalize_to_string(&self) -> joinerror::Result<String> {
        let host = self.host.as_ref().ok_or_else(|| {
            Error::new::<NoHostForNormalization>("no host available for normalization".to_string())
        })?;

        let mut path_parts = Vec::new();

        path_parts.push(self.owner.as_str());
        path_parts.push(self.name.as_str());

        let path = path_parts.join("/");
        Ok(format!("{}/{}", host, path))
    }

    /// Get the full URL with .git suffix if originally present
    pub fn to_string_with_suffix(&self) -> joinerror::Result<String> {
        let base_url = self.to_string()?;
        if self.git_suffix {
            Ok(format!("{}.git", base_url))
        } else {
            Ok(base_url)
        }
    }

    /// Get the full URL without .git suffix
    pub fn to_string(&self) -> joinerror::Result<String> {
        let host = self.host.as_ref().ok_or_else(|| {
            Error::new::<NoHostForNormalization>("no host available for normalization".to_string())
        })?;

        let mut path_parts = Vec::new();

        path_parts.push(self.owner.as_str());
        path_parts.push(self.name.as_str());

        let path = path_parts.join("/");

        // Return as HTTPS URL by default
        Ok(format!("https://{}/{}", host, path))
    }

    /// Validate if the URL components are reasonable
    pub fn validate(&self) -> joinerror::Result<()> {
        if self.name.is_empty() {
            return Err(Error::new::<EmptyRepositoryName>(
                "repository name cannot be empty".to_string(),
            ));
        }

        if let Some(ref host) = self.host {
            if host.is_empty() {
                return Err(Error::new::<EmptyHost>("host cannot be empty".to_string()));
            }
        }

        // Check for invalid characters in name
        if self.name.contains("..") || self.name.contains("//") {
            return Err(Error::new::<InvalidSequences>(
                "repository name contains invalid sequences".to_string(),
            ));
        }

        if self.owner.is_empty() {
            return Err(Error::new::<EmptyOwner>(
                "owner cannot be empty".to_string(),
            ));
        }
        if self.owner.contains("..") || self.owner.contains("//") {
            return Err(Error::new::<InvalidSequences>(
                "owner contains invalid sequences".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_https_github_with_git_suffix() {
        let url = GitUrl::parse("https://github.com/moss-foundation/sapic.git").unwrap();
        assert_eq!(url.host, Some("github.com".to_string()));
        assert_eq!(url.owner, "moss-foundation".to_string());
        assert_eq!(url.name, "sapic");
        assert_eq!(url.port, None);
        assert_eq!(url.git_suffix, true);
    }

    #[test]
    fn test_parse_https_github_without_git_suffix() {
        let url = GitUrl::parse("https://github.com/moss-foundation/sapic").unwrap();
        assert_eq!(url.host, Some("github.com".to_string()));
        assert_eq!(url.owner, "moss-foundation".to_string());
        assert_eq!(url.name, "sapic");
        assert_eq!(url.port, None);
        assert_eq!(url.git_suffix, false);
    }

    #[test]
    fn test_parse_ssh_github_with_git_suffix() {
        let url = GitUrl::parse("git@github.com:moss-foundation/sapic.git").unwrap();
        assert_eq!(url.host, Some("github.com".to_string()));
        assert_eq!(url.owner, "moss-foundation".to_string());
        assert_eq!(url.name, "sapic");
        assert_eq!(url.port, None);
        assert_eq!(url.git_suffix, true);
    }

    #[test]
    fn test_parse_ssh_github_without_git_suffix() {
        let url = GitUrl::parse("git@github.com:moss-foundation/sapic").unwrap();
        assert_eq!(url.host, Some("github.com".to_string()));
        assert_eq!(url.owner, "moss-foundation".to_string());
        assert_eq!(url.name, "sapic");
        assert_eq!(url.port, None);
        assert_eq!(url.git_suffix, false);
    }

    #[test]
    fn test_parse_ssh_protocol_github() {
        let url = GitUrl::parse("ssh://git@github.com/moss-foundation/sapic.git").unwrap();
        assert_eq!(url.host, Some("github.com".to_string()));
        assert_eq!(url.owner, "moss-foundation".to_string());
        assert_eq!(url.name, "sapic");
        assert_eq!(url.port, None);
        assert_eq!(url.git_suffix, true);
    }

    #[test]
    fn test_parse_git_protocol() {
        let url = GitUrl::parse("git://github.com/moss-foundation/sapic.git").unwrap();
        assert_eq!(url.host, Some("github.com".to_string()));
        assert_eq!(url.owner, "moss-foundation".to_string());
        assert_eq!(url.name, "sapic");
        assert_eq!(url.port, None);
        assert_eq!(url.git_suffix, true);
    }

    #[test]
    fn test_parse_simple_format() {
        let url = GitUrl::parse("github.com/moss-foundation/sapic").unwrap();
        assert_eq!(url.host, Some("github.com".to_string()));
        assert_eq!(url.owner, "moss-foundation".to_string());
        assert_eq!(url.name, "sapic");
        assert_eq!(url.port, None);
        assert_eq!(url.git_suffix, false);
    }

    #[test]
    fn test_parse_simple_format_with_git_suffix() {
        let url = GitUrl::parse("github.com/moss-foundation/sapic.git").unwrap();
        assert_eq!(url.host, Some("github.com".to_string()));
        assert_eq!(url.owner, "moss-foundation".to_string());
        assert_eq!(url.name, "sapic");
        assert_eq!(url.port, None);
        assert_eq!(url.git_suffix, true);
    }

    #[test]
    fn test_parse_custom_port_https() {
        let url = GitUrl::parse("https://git.example.com:8443/user/project.git").unwrap();
        assert_eq!(url.host, Some("git.example.com:8443".to_string()));
        assert_eq!(url.owner, "user".to_string());
        assert_eq!(url.name, "project");
        assert_eq!(url.port, Some(8443));
        assert_eq!(url.git_suffix, true);
    }

    #[test]
    fn test_parse_custom_port_ssh() {
        let url = GitUrl::parse("ssh://git@git.example.com:2222/user/project.git").unwrap();
        assert_eq!(url.host, Some("git.example.com:2222".to_string()));
        assert_eq!(url.owner, "user".to_string());
        assert_eq!(url.name, "project");
        assert_eq!(url.port, Some(2222));
        assert_eq!(url.git_suffix, true);
    }

    #[test]
    fn test_parse_default_port_stripped() {
        let url = GitUrl::parse("https://github.com:443/user/repo.git").unwrap();
        // Default port should be stripped from host, but URL parser might not preserve it
        assert_eq!(url.host, Some("github.com".to_string()));
        // The URL parser might not preserve default ports, so we don't assert on port
        assert_eq!(url.owner, "user".to_string());
        assert_eq!(url.name, "repo");
    }

    #[test]
    fn test_parse_nested_path() {
        let url = GitUrl::parse("https://gitlab.example.com/group/subgroup/project.git").unwrap();
        assert_eq!(url.host, Some("gitlab.example.com".to_string()));
        assert_eq!(url.owner, "group/subgroup".to_string());
        assert_eq!(url.name, "project");
        assert_eq!(url.git_suffix, true);
    }

    #[test]
    fn test_parse_single_level_path_should_fail() {
        // Single level paths should fail since owner is required
        let result = GitUrl::parse("github.com/repo");
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<NoRepositoryPath>());
    }

    #[test]
    fn test_parse_gitlab_ssh() {
        let url = GitUrl::parse("git@gitlab.com:group/project.git").unwrap();
        assert_eq!(url.host, Some("gitlab.com".to_string()));
        assert_eq!(url.owner, "group".to_string());
        assert_eq!(url.name, "project");
        assert_eq!(url.git_suffix, true);
    }

    #[test]
    fn test_parse_bitbucket() {
        let url = GitUrl::parse("https://bitbucket.org/user/repo.git").unwrap();
        assert_eq!(url.host, Some("bitbucket.org".to_string()));
        assert_eq!(url.owner, "user".to_string());
        assert_eq!(url.name, "repo");
        assert_eq!(url.git_suffix, true);
    }

    #[test]
    fn test_parse_azure_devops() {
        let url =
            GitUrl::parse("https://dev.azure.com/organization/project/_git/repository").unwrap();
        assert_eq!(url.host, Some("dev.azure.com".to_string()));
        assert_eq!(url.owner, "organization/project/_git".to_string());
        assert_eq!(url.name, "repository");
        assert_eq!(url.git_suffix, false);
    }

    #[test]
    fn test_parse_scheme_relative_url() {
        let url = GitUrl::parse("//github.com/user/repo.git").unwrap();
        assert_eq!(url.host, Some("github.com".to_string()));
        assert_eq!(url.owner, "user".to_string());
        assert_eq!(url.name, "repo");
        assert_eq!(url.git_suffix, true);
    }

    #[test]
    fn test_parse_multiple_slashes_cleanup() {
        let url = GitUrl::parse("https://github.com//user//repo//.git").unwrap();
        assert_eq!(url.host, Some("github.com".to_string()));
        assert_eq!(url.owner, "user".to_string());
        assert_eq!(url.name, "repo");
        assert_eq!(url.git_suffix, true);
    }

    // Error cases
    #[test]
    fn test_parse_empty_string() {
        let result = GitUrl::parse("");
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<EmptyUrlString>());
    }

    #[test]
    fn test_parse_no_path() {
        let result = GitUrl::parse("https://github.com");
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<NoRepositoryPath>());
    }

    #[test]
    fn test_parse_only_slash_path() {
        let result = GitUrl::parse("https://github.com/");
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<NoRepositoryPath>());
    }

    #[test]
    fn test_parse_invalid_ssh_format_no_colon() {
        // Test a truly invalid SSH format - with @ but no colon and no path
        let result = GitUrl::parse("git@github.com");
        assert!(result.is_err());
        // This will fail because it tries to parse as https://git@github.com and has no path
        let err = result.unwrap_err();
        assert!(err.is::<NoRepositoryPath>() || err.is::<InvalidUrlFormat>());
    }

    #[test]
    fn test_parse_invalid_ssh_format_no_at() {
        let result = GitUrl::parse("github.com:user/repo");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_simple_format_no_slash() {
        let result = GitUrl::parse("github.com");
        assert!(result.is_err());
        // The actual error type may vary depending on parsing path
        let err = result.unwrap_err();
        assert!(err.is::<InvalidUrlFormat>() || err.is::<NoRepositoryPath>());
    }

    // Normalization tests
    #[test]
    fn test_normalize_basic() {
        let url = GitUrl::parse("https://github.com/moss-foundation/sapic.git").unwrap();
        let normalized = url.normalize_to_string().unwrap();
        assert_eq!(normalized, "github.com/moss-foundation/sapic");
    }

    #[test]
    fn test_normalize_with_custom_port() {
        let url = GitUrl::parse("https://git.example.com:8443/user/project.git").unwrap();
        let normalized = url.normalize_to_string().unwrap();
        assert_eq!(normalized, "git.example.com:8443/user/project");
    }

    #[test]
    fn test_normalize_nested_path() {
        let url = GitUrl::parse("https://gitlab.example.com/group/subgroup/project.git").unwrap();
        let normalized = url.normalize_to_string().unwrap();
        assert_eq!(normalized, "gitlab.example.com/group/subgroup/project");
    }

    #[test]
    fn test_to_string_with_suffix_true() {
        let url = GitUrl::parse("https://github.com/user/repo.git").unwrap();
        let result = url.to_string_with_suffix().unwrap();
        assert_eq!(result, "https://github.com/user/repo.git");
    }

    #[test]
    fn test_to_string_with_suffix_false() {
        let url = GitUrl::parse("https://github.com/user/repo").unwrap();
        let result = url.to_string_with_suffix().unwrap();
        assert_eq!(result, "https://github.com/user/repo");
    }

    #[test]
    fn test_to_string_without_suffix() {
        let url = GitUrl::parse("https://github.com/user/repo.git").unwrap();
        let result = url.to_string().unwrap();
        assert_eq!(result, "https://github.com/user/repo");
    }

    // Validation tests
    #[test]
    fn test_validate_success() {
        let url = GitUrl::parse("https://github.com/user/repo").unwrap();
        assert!(url.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_name() {
        let mut url = GitUrl::parse("https://github.com/user/repo").unwrap();
        url.name = String::new();
        let result = url.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<EmptyRepositoryName>());
    }

    #[test]
    fn test_validate_empty_host() {
        let mut url = GitUrl::parse("https://github.com/user/repo").unwrap();
        url.host = Some(String::new());
        let result = url.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<EmptyHost>());
    }

    #[test]
    fn test_validate_invalid_name_sequences() {
        let mut url = GitUrl::parse("https://github.com/user/repo").unwrap();
        url.name = "repo..name".to_string();
        let result = url.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<InvalidSequences>());
    }

    #[test]
    fn test_validate_invalid_owner_sequences() {
        let mut url = GitUrl::parse("https://github.com/user/repo").unwrap();
        url.owner = "user//name".to_string();
        let result = url.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<InvalidSequences>());
    }

    #[test]
    fn test_validate_empty_owner_when_some() {
        let mut url = GitUrl::parse("https://github.com/user/repo").unwrap();
        url.owner = String::new();
        let result = url.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<EmptyOwner>());
    }

    // Edge cases
    #[test]
    fn test_parse_case_preservation() {
        let url = GitUrl::parse("https://GitHub.com/User/Repo.git").unwrap();
        // Host should be lowercased by URL parser, but path should preserve case
        assert_eq!(url.host, Some("github.com".to_string()));
        assert_eq!(url.owner, "User".to_string());
        assert_eq!(url.name, "Repo");
    }

    #[test]
    fn test_parse_unicode_in_path() {
        let url = GitUrl::parse("https://github.com/user/репозиторий.git").unwrap();
        assert_eq!(url.host, Some("github.com".to_string()));
        assert_eq!(url.owner, "user".to_string());
        // URL parser percent-encodes Unicode
        assert_eq!(
            url.name,
            "%D1%80%D0%B5%D0%BF%D0%BE%D0%B7%D0%B8%D1%82%D0%BE%D1%80%D0%B8%D0%B9"
        );
    }

    #[test]
    fn test_parse_hyphens_and_underscores() {
        let url = GitUrl::parse("https://github.com/my-org/my_project-name.git").unwrap();
        assert_eq!(url.owner, "my-org".to_string());
        assert_eq!(url.name, "my_project-name");
    }

    #[test]
    fn test_parse_single_character_names() {
        let url = GitUrl::parse("https://github.com/a/b.git").unwrap();
        assert_eq!(url.owner, "a".to_string());
        assert_eq!(url.name, "b");
    }

    #[test]
    fn test_parse_very_long_path() {
        let long_name = "a".repeat(100);
        let url_string = format!("https://github.com/user/{}.git", long_name);
        let url = GitUrl::parse(&url_string).unwrap();
        assert_eq!(url.name, long_name);
    }

    #[test]
    fn test_parse_ssh_with_different_user() {
        let url = GitUrl::parse("deploy@git.company.com:repos/project.git").unwrap();
        assert_eq!(url.host, Some("git.company.com".to_string()));
        assert_eq!(url.owner, "repos".to_string());
        assert_eq!(url.name, "project");
    }

    #[test]
    fn test_parse_sourceforge_style() {
        let url = GitUrl::parse("git://git.code.sf.net/p/project/code").unwrap();
        assert_eq!(url.host, Some("git.code.sf.net".to_string()));
        assert_eq!(url.owner, "p/project".to_string());
        assert_eq!(url.name, "code");
    }

    #[test]
    fn test_parse_aws_codecommit() {
        let url = GitUrl::parse("https://git-codecommit.us-east-1.amazonaws.com/v1/repos/my-repo")
            .unwrap();
        assert_eq!(
            url.host,
            Some("git-codecommit.us-east-1.amazonaws.com".to_string())
        );
        assert_eq!(url.owner, "v1/repos".to_string());
        assert_eq!(url.name, "my-repo");
    }

    #[test]
    fn test_parse_google_cloud_source() {
        let url =
            GitUrl::parse("https://source.developers.google.com/p/project/r/repository").unwrap();
        assert_eq!(url.host, Some("source.developers.google.com".to_string()));
        assert_eq!(url.owner, "p/project/r".to_string());
        assert_eq!(url.name, "repository");
    }
}
