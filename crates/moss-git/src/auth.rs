mod oauth;
mod ssh;
mod cred_helper;

pub use oauth::*;
pub use ssh::*;
pub use cred_helper::*;

use git2::build::RepoBuilder;
use git2::RemoteCallbacks;
use git2::{Repository};
use oauth2::TokenResponse;
use std::io::{BufRead, Write};
use std::path::Path;
use crate::auth::cred_helper::CredHelper;

// TODO: Create a `Sensitive` type for storing passwords securely?


pub trait AuthAgent {
    fn authorize(self, remote_callbacks: &mut RemoteCallbacks);
}


fn clone_flow(url: &str, path: &Path, callback: RemoteCallbacks) -> Result<Repository, String> {
    // remove_dir_all(path);

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callback);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);

    match builder.clone(url, path) {
        Ok(repo) => Ok(repo),
        Err(e) => Err(format!("failed to clone: {}", e)),
    }
}


#[cfg(test)]
mod github_tests {
    use crate::auth::oauth::OAuthAgent;
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

        let auth_agent = OAuthAgent::new(auth_url, token_url, client_id, client_secret);

        let mut callbacks = git2::RemoteCallbacks::new();

        auth_agent.authorize(&mut callbacks);

        let repo = clone_flow(repo_url, repo_path, callbacks).unwrap();
    }

    #[test]
    fn cloning_with_ssh() {
        let repo_url = "git@github.com:***/***";
        let repo_path = Path::new("Path to your local repo");

        let private = Path::new(".ssh/id_***");
        let public = Path::new(".ssh/id_***.pub");
        let password = "**";

        let auth_agent = SSHAgent::new(Some(public), private, Some(password.into()));

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

        let auth_agent = CredHelper::new();

        let mut callbacks = git2::RemoteCallbacks::new();
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

        let auth_agent = SSHAgent::new(Some(public), private, Some(password.into()));

        let mut callbacks = git2::RemoteCallbacks::new();
        auth_agent.authorize(&mut callbacks);

        let repo = clone_flow(repo_url, repo_path, callbacks).unwrap();
    }


}
