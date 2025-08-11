pub mod ssh_auth_agent;
pub mod utils;

use joinerror::Error;
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

    pub fn parse(url: &str) -> joinerror::Result<GitUrl> {
        // FIXME: Handle more complex URL schemas
        let parts = url.split("/").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(Error::new::<()>(format!("Unsupported Git URL: {}", url)));
        }
        Ok(GitUrl {
            domain: parts[0].to_string(),
            owner: parts[1].to_string(),
            name: parts[2].to_string(),
        })
    }
}

pub enum GitProviderType {
    GitHub,
    // TODO: Support self-hosted gitlab
    GitLab,
}
