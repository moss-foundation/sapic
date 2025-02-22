use anyhow::Result;
use std::sync::Arc;
// TODO: Create a `Sensitive` type for storing passwords securely?
// TODO: Preserving the auth info for repos

pub trait TestStorage {
    // TODO: We will use more secure method of storing the AuthAgent info
    // For easy testing, we will use environment variables for now
    fn write_to_file(&self) -> Result<()>;
    fn read_from_file() -> Result<Arc<Self>>;
}

#[cfg(test)]
mod github_tests {
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    use super::TestStorage;
    use crate::adapters::auth::{oauth::OAuthAgent, ssh::SSHAgent};
    use crate::repo::RepoHandle;

    // Run cargo test cloning_with_https -- --nocapture
    #[test]
    fn cloning_with_https() {
        // From example: https://github.com/ramosbugs/oauth2-rs/blob/main/examples/github.rs
        dotenv::dotenv().ok();
        let repo_url = &dotenv::var("GITHUB_TEST_REPO_HTTPS").unwrap();
        let repo_path = Path::new("test-repo");

        let auth_agent =
            OAuthAgent::read_from_file().unwrap_or_else(|_| Arc::new(OAuthAgent::github()));

        let repo = RepoHandle::clone(repo_url, repo_path, auth_agent).unwrap();
    }

    #[test]
    fn cloning_with_ssh() {
        dotenv::dotenv().ok();
        let repo_url = &dotenv::var("GITHUB_TEST_REPO_SSH").unwrap();
        let repo_path = Path::new("test-repo");

        let private = PathBuf::from(dotenv::var("GITHUB_SSH_PRIVATE").unwrap());
        let public = PathBuf::from(dotenv::var("GITHUB_SSH_PUBLIC").unwrap());
        let password = dotenv::var("GITHUB_SSH_PASSWORD").unwrap();

        let auth_agent = Arc::new(SSHAgent::new(Some(public), private, Some(password.into())));
        let repo = RepoHandle::clone(repo_url, repo_path, auth_agent).unwrap();
    }
}

#[cfg(test)]
mod gitlab_tests {
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    use super::TestStorage;
    use crate::adapters::auth::{oauth::OAuthAgent, ssh::SSHAgent};
    use crate::repo::RepoHandle;

    #[test]
    fn cloning_with_https() {
        dotenv::dotenv().ok();
        let repo_url = &dotenv::var("GITLAB_TEST_REPO_HTTPS").unwrap();
        let repo_path = Path::new("test-repo-lab");

        let auth_agent =
            OAuthAgent::read_from_file().unwrap_or_else(|_| Arc::new(OAuthAgent::github()));

        let repo = RepoHandle::clone(repo_url, repo_path, auth_agent).unwrap();
    }

    #[test]
    fn cloning_with_ssh() {
        let repo_url = &dotenv::var("GITLAB_TEST_REPO_SSH").unwrap();
        let repo_path = Path::new("test-repo-lab");

        let private = PathBuf::from(dotenv::var("GITLAB_SSH_PRIVATE").unwrap());
        let public = PathBuf::from(dotenv::var("GITLAB_SSH_PUBLIC").unwrap());
        let password = dotenv::var("GITLAB_SSH_PASSWORD").unwrap();

        let auth_agent = Arc::new(SSHAgent::new(Some(public), private, Some(password.into())));
        let repo = RepoHandle::clone(repo_url, repo_path, auth_agent).unwrap();
    }
}
