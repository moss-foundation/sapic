pub mod ssh_auth_agent;
pub mod utils;

use moss_git::GitAuthAgent;

pub trait SSHAuthAgent: GitAuthAgent {}

pub struct GitUrl {
    pub domain: String,
    pub owner: String,
    pub name: String,
}

impl GitUrl {
    /// Parse a Git URL normalized by `moss_git::normalize_git_url`
    /// such as "github.com/user/repo"

    pub fn parse(url: &str) -> anyhow::Result<GitUrl> {
        // FIXME: Handle more complex URL schemas
        let parts = url.split("/").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Unsupported Git Url: {}", url));
        }
        Ok(GitUrl {
            domain: parts[0].to_string(),
            owner: parts[1].to_string(),
            name: parts[2].to_string(),
        })
    }
}
