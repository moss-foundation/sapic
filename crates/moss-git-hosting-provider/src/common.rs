pub mod ssh_auth_agent;
pub mod utils;

use joinerror::Error;
use moss_git::GitAuthAgent;

pub trait SSHAuthAgent: GitAuthAgent {}

pub const GITHUB_DOMAIN: &'static str = "github.com";
pub const GITLAB_DOMAIN: &str = "gitlab.com";

// FIXME: Maybe this type should not be here
pub struct GitUrlForAPI {
    pub domain: String,
    pub owner: String,
    pub name: String,
}

impl GitUrlForAPI {
    /// Parse a Git URL normalized by `moss_git::normalize_git_url`
    /// such as "github.com/user/repo"

    pub fn parse(url: &str) -> joinerror::Result<GitUrlForAPI> {
        // FIXME: Handle more complex URL schemas
        let parts = url.split("/").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(Error::new::<()>(format!("Unsupported Git URL: {}", url)));
        }
        Ok(GitUrlForAPI {
            domain: parts[0].to_string(),
            owner: parts[1].to_string(),
            name: parts[2].to_string(),
        })
    }
}

impl ToString for GitUrlForAPI {
    fn to_string(&self) -> String {
        format!("{}/{}/{}", self.domain, self.owner, self.name)
    }
}
