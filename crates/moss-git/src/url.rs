use anyhow::{Result, anyhow};
use url::Url;

// Strip the protocol and ".git" suffix from a git repo url
pub fn normalize_git_url(repo_url: &Url) -> Result<String> {
    let domain = repo_url
        .domain()
        .ok_or_else(|| anyhow!("no domain found"))?;
    let path = {
        let path = repo_url.path();
        path.strip_suffix(".git").unwrap_or(path)
    };

    Ok(format!("{domain}{path}"))
}

mod tests {
    use super::*;

    #[test]
    fn test_clean_git_url() {
        let https_url = Url::parse("https://github.com/moss-foundation/sapic.git").unwrap();
        let cleaned_url = normalize_git_url(&https_url).unwrap();

        assert_eq!(cleaned_url, "github.com/moss-foundation/sapic");
    }
}
