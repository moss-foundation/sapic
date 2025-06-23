use anyhow::{Result, anyhow};
use url::Url;

// Strip the protocol and ".git" suffix from a git repo url
pub fn clean_git_url(repo_url: &Url) -> Result<String> {
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
    use url::Url;

    use crate::url::clean_git_url;
    #[test]
    fn test_clean_git_url() {
        let https_url = Url::parse("https://github.com/moss-foundation/sapic.git").unwrap();
        let cleaned_url = clean_git_url(&https_url).unwrap();

        assert_eq!(cleaned_url, "github.com/moss-foundation/sapic");
    }
}
