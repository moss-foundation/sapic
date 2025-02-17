pub mod oauth;

use git2::build::RepoBuilder;
use git2::RemoteCallbacks;
use git2::{Config, Cred, Repository};
use oauth2::TokenResponse;
use std::io::{BufRead, Write};
use std::path::Path;

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
    use crate::oauth::OAuth;
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
        let callback_port = "1357";

        let github_oauth = OAuth::new(auth_url, token_url, client_id, client_secret, callback_port);
        let mut callbacks = git2::RemoteCallbacks::new();

        github_oauth.flow(&mut callbacks);

        let repo = clone_flow(repo_url, repo_path, callbacks).unwrap();
    }

    #[test]
    fn cloning_with_ssh() {
        let repo_url = "git@github.com:***/***";
        let repo_path = Path::new("Path to your local repo");

        let private = Path::new(".ssh/id_***");
        let public = Path::new(".ssh/id_***.pub");
        let password = "**";

        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |url, username_from_url, _allowed_types| {
            Cred::ssh_key("git", Some(public), private, Some(password))
        });

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

        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |url, username_from_url, _allowed_types| {
            let default_config = Config::open_default().unwrap();
            Cred::credential_helper(&default_config, url, username_from_url)
        });

        let repo = clone_flow(repo_url, repo_path, callbacks).unwrap();

    }

    #[test]
    fn cloning_with_ssh() {
        let repo_url = "git@gitlab.com:**/**.git";
        let repo_path = Path::new("test-repo");

        let private = Path::new(".ssh/id_***");
        let public = Path::new(".ssh/id_***.pub");
        let password = "**";

        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |url, username_from_url, _allowed_types| {
            Cred::ssh_key("git", Some(public), private, Some(password))
        });

        let repo = clone_flow(repo_url, repo_path, callbacks).unwrap();
    }


}
