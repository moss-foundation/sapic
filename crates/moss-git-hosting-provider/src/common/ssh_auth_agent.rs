use anyhow::Result;
use git2::{Cred, RemoteCallbacks};
use moss_git::GitAuthAgent;
use std::path::PathBuf;

use super::SSHAuthAgent;

#[derive(Clone)]
pub struct SSHAuthAgentImpl {
    public_key: Option<PathBuf>,
    private_key: PathBuf,
    passphrase: Option<String>,
}

impl SSHAuthAgent for SSHAuthAgentImpl {}

impl SSHAuthAgentImpl {
    pub fn new(
        public_key: Option<PathBuf>,
        private_key: PathBuf,
        passphrase: Option<String>,
    ) -> Self {
        SSHAuthAgentImpl {
            public_key,
            private_key,
            passphrase,
        }
    }
}

impl GitAuthAgent for SSHAuthAgentImpl {
    fn generate_callback<'a>(&'a self, cb: &mut RemoteCallbacks<'a>) -> Result<()> {
        cb.credentials(|_url, _username_from_url, _allowed_types| {
            Cred::ssh_key(
                "git",
                self.public_key.as_deref(),
                &self.private_key,
                self.passphrase.as_deref(),
            )
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use moss_git::repo::RepoHandle;
    use std::{
        path::{Path, PathBuf},
        sync::Arc,
    };
    #[ignore]
    #[test]
    fn manual_github_cloning_with_ssh() {
        dotenv::dotenv().ok();
        let repo_url = &dotenv::var("GITHUB_TEST_REPO_SSH").unwrap();
        let repo_path = Path::new("test-repo");

        let private = PathBuf::from(dotenv::var("GITHUB_SSH_PRIVATE").unwrap());
        let public = PathBuf::from(dotenv::var("GITHUB_SSH_PUBLIC").unwrap());
        let password = dotenv::var("GITHUB_SSH_PASSWORD").unwrap();

        let auth_agent = Arc::new(SSHAuthAgentImpl::new(
            Some(public),
            private,
            Some(password.into()),
        ));
        let _repo = RepoHandle::clone(repo_url, repo_path, auth_agent).unwrap();
    }

    #[ignore]
    #[test]
    fn manual_gitlab_cloning_with_ssh() {
        dotenv::dotenv().ok();
        let repo_url = &dotenv::var("GITLAB_TEST_REPO_SSH").unwrap();
        let repo_path = Path::new("test-repo-lab");

        let private = PathBuf::from(dotenv::var("GITLAB_SSH_PRIVATE").unwrap());
        let public = PathBuf::from(dotenv::var("GITLAB_SSH_PUBLIC").unwrap());
        let password = dotenv::var("GITLAB_SSH_PASSWORD").unwrap();

        let auth_agent = Arc::new(SSHAuthAgentImpl::new(
            Some(public),
            private,
            Some(password.into()),
        ));
        let _repo = RepoHandle::clone(repo_url, repo_path, auth_agent).unwrap();
    }
}
