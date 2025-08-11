use moss_git::GitAuthAgent;
use moss_keyring::KeyringClientImpl;
use std::sync::Arc;

use crate::{
    common::GitProviderType, github::auth::GitHubAuthAgentImpl, gitlab::auth::GitLabAuthAgentImpl,
};

pub enum AuthAgentType {
    GitHub,
    // TODO: Support custom hosted gitlab server
    GitLab,
    // TODO: Support SSH
}

impl From<GitProviderType> for AuthAgentType {
    fn from(value: GitProviderType) -> Self {
        match value {
            GitProviderType::GitHub => AuthAgentType::GitHub,
            GitProviderType::GitLab => AuthAgentType::GitLab,
        }
    }
}

// TODO: Create auth agent based on user input

/// A utility function to generate a Git Auth Agent for a particular provider
/// Since different git providers require different auth agents
pub fn generate_auth_agent(
    auth_agent_type: AuthAgentType,
) -> joinerror::Result<Arc<dyn GitAuthAgent>> {
    // TODO: Fetch client_id and client_secret from the server

    // FIXME: Not a big deal but maybe we can share the keyring_client
    let keyring_client = Arc::new(KeyringClientImpl::new());

    match auth_agent_type {
        AuthAgentType::GitHub => {
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
        AuthAgentType::GitLab => {
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
    }
}
