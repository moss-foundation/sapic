mod oauth;
mod ssh;

pub use oauth::*;
pub use ssh::*;

use crate::clone_flow;
use anyhow::Result;
use git2::RemoteCallbacks;
use oauth2::TokenResponse;
use std::io::{BufRead, Write};
use std::path::Path;
// TODO: Create a `Sensitive` type for storing passwords securely?
// TODO: Preserving the auth info for repos

pub trait AuthAgent {
    // We have to return the Cred in order to store
    fn authorize<'a>(self, remote_callbacks: &mut RemoteCallbacks<'a>) -> Result<()>;
}

pub trait TestStorage {
    // TODO: We will use more secure method of storing the AuthAgent info
    // For easy testing, we will use environment variables for now
    fn write_to_file(&self) -> Result<()>;
    fn read_from_file() -> Result<Box<Self>>;
}

#[cfg(test)]
mod github_tests {
    use crate::auth::oauth::OAuthAgent;
    use git2::Time;
    use std::path::PathBuf;

    use super::*;

    // Run cargo test cloning_with_https -- --nocapture
    #[test]
    fn cloning_with_https() {
        // From example: https://github.com/ramosbugs/oauth2-rs/blob/main/examples/github.rs
        let repo_url = "https://github.com/**/**.git";
        let repo_path = Path::new("Path to your local repo");

        let auth_url = "https://github.com/login/oauth/authorize";
        let token_url = "https://github.com/login/oauth/access_token";
        let client_id = "***";
        let client_secret = "***";

        let mut auth_agent = match OAuthAgent::read_from_file() {
            Ok(agent) => agent,
            Err(_) => Box::new(OAuthAgent::new(
                auth_url,
                token_url,
                client_id,
                client_secret,
                vec![],
                None,
            )),
        };

        let mut callbacks = git2::RemoteCallbacks::new();

        auth_agent.authorize(&mut callbacks);

        let repo = clone_flow(repo_url, repo_path, callbacks).unwrap();
    }

    #[test]
    fn cloning_with_ssh() {
        let repo_url = "git@github.com:***/***";
        let repo_path = Path::new("Path to your local repo");

        let private = PathBuf::from(".ssh/id_***");
        let public = PathBuf::from(".ssh/id_***.pub");
        let password = "**";

        let mut auth_agent = SSHAgent::new(Some(public), private, Some(password.into()));

        let mut callbacks = git2::RemoteCallbacks::new();
        auth_agent.authorize(&mut callbacks);

        let repo = clone_flow(repo_url, repo_path, callbacks).unwrap();
    }
}

#[cfg(test)]
mod gitlab_tests {
    use super::*;
    #[test]
    fn cloning_with_https() {
        let repo_url = "https://gitlab.com/**/**.git";
        let repo_path = Path::new("Path to your local repo");

        let auth_url = "https://gitlab.com/oauth/authorize";
        let token_url = "https://gitlab.com/oauth/token";
        let client_id = "***";
        let client_secret = "***";

        let mut callbacks = git2::RemoteCallbacks::new();

        let mut auth_agent = match OAuthAgent::read_from_file() {
            Ok(agent) => agent,
            Err(_) => Box::new(OAuthAgent::new(
                auth_url,
                token_url,
                client_id,
                client_secret,
                vec![],
                None,
            )),
        };

        auth_agent.authorize(&mut callbacks);

        let repo = clone_flow(repo_url, repo_path, callbacks).unwrap();
    }

    #[test]
    fn cloning_with_ssh() {
        let repo_url = "git@gitlab.com:**/**.git";
        let repo_path = Path::new("test-repo");

        let private = Path::new(".ssh/id_***");
        let public = Path::new(".ssh/id_***.pub");
        let password = "**";

        let mut auth_agent =
            SSHAgent::new(Some(public.into()), private.into(), Some(password.into()));

        let mut callbacks = git2::RemoteCallbacks::new();
        auth_agent.authorize(&mut callbacks);

        let repo = clone_flow(repo_url, repo_path, callbacks).unwrap();
    }
}
