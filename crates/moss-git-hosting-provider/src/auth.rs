use moss_git::GitAuthAgent;
use moss_keyring::KeyringClientImpl;
use std::sync::Arc;
use url::Url;

use crate::{github::auth::GitHubAuthAgentImpl, gitlab::auth::GitLabAuthAgentImpl};

/// A utility function to generate a Git Auth Agent for a particular repo
/// Since different git providers require different auth agents
pub fn generate_auth_agent(repo_url: &Url) -> joinerror::Result<Arc<dyn GitAuthAgent>> {
    // TODO: Support SSH
    // TODO: Fetch client_id and client_secret from the server
    if repo_url.domain().is_none() {
        return Err(joinerror::Error::new::<()>(
            "repository url does not contain a domain",
        ));
    }

    let domain = repo_url.domain().unwrap();
    // FIXME: Not a big deal but maybe we can share the keyring_client
    let keyring_client = Arc::new(KeyringClientImpl::new());

    match domain {
        "github.com" => {
            let client_id = dotenv::var("GITHUB_CLIENT_ID").map_err(|err| {
                joinerror::Error::new::<()>(format!(
                    "failed to get GITHUB_CLIENT_ID env var: {}",
                    err.to_string()
                ))
            })?;
            let client_secret = dotenv::var("GITHUB_CLIENT_SECRET").map_err(|err| {
                joinerror::Error::new::<()>(format!(
                    "failed to get GITHUB_CLIENT_SECRET env var: {}",
                    err.to_string()
                ))
            })?;
            Ok(Arc::new(GitHubAuthAgentImpl::new(
                keyring_client,
                client_id,
                client_secret,
            )))
        }
        "gitlab.com" => {
            let client_id = dotenv::var("GITLAB_CLIENT_ID").map_err(|err| {
                joinerror::Error::new::<()>(format!(
                    "failed to get GITHUB_CLIENT_ID env var: {}",
                    err.to_string()
                ))
            })?;
            let client_secret = dotenv::var("GITLAB_CLIENT_SECRET").map_err(|err| {
                joinerror::Error::new::<()>(format!(
                    "failed to get GITHUB_CLIENT_SECRET env var: {}",
                    err.to_string()
                ))
            })?;
            Ok(Arc::new(GitLabAuthAgentImpl::new(
                keyring_client,
                client_id,
                client_secret,
            )))
        }
        _ => Err(joinerror::Error::new::<()>(format!(
            "unsupported git provider: {}",
            domain
        ))),
    }
}
