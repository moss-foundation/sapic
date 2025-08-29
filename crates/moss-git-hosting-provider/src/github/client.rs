use async_trait::async_trait;
use joinerror::OptionExt;
use moss_git::GitAuthAgent;
use oauth2::http::header::ACCEPT;
use reqwest::{Client, header::AUTHORIZATION};
use std::sync::Arc;
use url::Url;

use crate::{
    GitAuthProvider, GitHostingProvider,
    common::{GitUrl, SSHAuthAgent},
    constants::GITHUB_API_URL,
    github::{
        auth::GitHubAuthAgent,
        response::{GetContributorsResponse, GetRepositoryResponse, GetUserResponse},
    },
    models::types::{Contributor, RepositoryMetadata, UserInfo},
};

const CONTENT_TYPE: &'static str = "application/vnd.github+json";

pub struct GitHubClient {
    client: Client,
    #[allow(dead_code)]
    // TODO: Support multiple accounts?
    client_auth_agent: Arc<GitHubAuthAgent>,
    #[allow(dead_code)]
    ssh_auth_agent: Option<Arc<dyn SSHAuthAgent>>,
}

impl GitHubClient {
    pub fn new(
        client: Client,
        client_auth_agent: Arc<GitHubAuthAgent>,
        ssh_auth_agent: Option<impl SSHAuthAgent + 'static>,
    ) -> Self {
        Self {
            client,
            client_auth_agent,
            ssh_auth_agent: ssh_auth_agent.map(|agent| Arc::new(agent) as Arc<dyn SSHAuthAgent>),
        }
    }

    pub fn is_logged_in(&self) -> joinerror::Result<bool> {
        self.client_auth_agent.is_logged_in()
    }

    // Try to fetch/generate credentials and return currently logged-in user info
    // This will trigger an initial OAuth authorization
    // Or will fetch the stored access_token
    pub async fn login(&self) -> joinerror::Result<UserInfo> {
        let _ = self.client_auth_agent.clone().credentials_async().await?;
        self.current_user().await
    }
}

unsafe impl Send for GitHubClient {}
unsafe impl Sync for GitHubClient {}

impl GitAuthProvider for GitHubClient {
    fn git_auth_agent(&self) -> Arc<dyn GitAuthAgent> {
        self.client_auth_agent.clone()
    }
}

// TODO: Handle authentication expiration and reauthentication
// TODO: Better error message when failing
#[async_trait]
impl GitHostingProvider for GitHubClient {
    fn name(&self) -> String {
        "GitHub".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://github.com").unwrap()
    }

    async fn current_user(&self) -> joinerror::Result<UserInfo> {
        let access_token = self
            .client_auth_agent
            .clone()
            .access_token()
            .ok_or_join_err::<()>("github is not logged in yet")?;

        let user_response: GetUserResponse = self
            .client
            .get(format!("{GITHUB_API_URL}/user"))
            .header(ACCEPT, CONTENT_TYPE)
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        // If the user's email is private, we will use their noreply email
        let email = user_response.email.unwrap_or(format!(
            "{}+{}@users.noreply.github.com",
            user_response.id, user_response.login
        ));

        Ok(UserInfo {
            name: user_response.login,
            email,
        })
    }

    async fn contributors(&self, repo_ref: &GitUrl) -> joinerror::Result<Vec<Contributor>> {
        let repo_url = format!("{}/{}", &repo_ref.owner, &repo_ref.name);
        let access_token = self
            .client_auth_agent
            .clone()
            .access_token()
            .ok_or_join_err::<()>("github is not logged in yet")?;

        let contributors_response: GetContributorsResponse = self
            .client
            .get(format!("{GITHUB_API_URL}/repos/{repo_url}/contributors"))
            .header(ACCEPT, CONTENT_TYPE)
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(contributors_response
            .items
            .into_iter()
            .map(|item| Contributor {
                name: item.login,
                avatar_url: item.avatar_url,
            })
            .collect())
    }

    async fn repository_metadata(
        &self,
        repo_ref: &GitUrl,
    ) -> joinerror::Result<RepositoryMetadata> {
        let repo_url = format!("{}/{}", &repo_ref.owner, &repo_ref.name);
        let access_token = self
            .client_auth_agent
            .clone()
            .access_token()
            .ok_or_join_err::<()>("github is not logged in yet")?;

        let repo_response: GetRepositoryResponse = self
            .client
            .get(format!("{GITHUB_API_URL}/repos/{repo_url}"))
            .header(ACCEPT, CONTENT_TYPE)
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(RepositoryMetadata {
            updated_at: repo_response.updated_at,
            owner: repo_response.owner.login,
        })
    }
}

// API related tests are skipped during CI, since it requires setting up refresh token in envvar
// Set `GITHUB_ACCESS_TOKEN = {}` in /.env

#[cfg(test)]
mod tests {
    use std::{ops::Deref, sync::LazyLock};
    // FIXME: Rewrite the tests
    use moss_keyring::KeyringClientImpl;

    use crate::{
        common::ssh_auth_agent::SSHAuthAgentImpl,
        envvar_keys::{GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET},
    };

    use super::*;

    static REPO_REF: LazyLock<GitUrl> = LazyLock::new(|| GitUrl {
        domain: "github.com".to_string(),
        owner: "brutusyhy".to_string(),
        name: "test-public-repo".to_string(),
    });

    async fn create_test_client() -> Arc<GitHubClient> {
        dotenv::dotenv().ok();

        let reqwest_client = reqwest::ClientBuilder::new()
            .user_agent("SAPIC")
            .build()
            .unwrap();

        let keyring_client = Arc::new(KeyringClientImpl::new());
        let auth_agent = Arc::new(GitHubAuthAgent::new(
            oauth2::ureq::builder().redirects(0).build(),
            keyring_client,
            dotenv::var(GITHUB_CLIENT_ID).unwrap(),
            dotenv::var(GITHUB_CLIENT_SECRET).unwrap(),
        ));

        let client = Arc::new(GitHubClient::new(
            reqwest_client,
            auth_agent,
            None as Option<SSHAuthAgentImpl>,
        ));

        client.login().await.unwrap();
        client
    }

    #[ignore]
    #[tokio::test]
    async fn github_client_name() {
        let client = create_test_client().await;

        assert_eq!(client.name(), "GitHub");
    }

    #[ignore]
    #[tokio::test]
    async fn github_client_base_url() {
        let client = create_test_client().await;

        let expected_url = Url::parse("https://github.com").unwrap();

        assert_eq!(client.base_url(), expected_url);
    }

    #[ignore]
    #[tokio::test]
    async fn github_client_current_user() {
        let client = create_test_client().await;
        let user_info = client.current_user().await.unwrap();
        println!("{:?}", user_info);
    }
    #[ignore]
    #[tokio::test]
    async fn github_client_repo_metadata() {
        let client = create_test_client().await;
        let repo_info = client.repository_metadata(REPO_REF.deref()).await.unwrap();
        println!("{:?}", repo_info);
    }

    #[ignore]
    #[tokio::test]
    async fn github_client_contributors() {
        let client = create_test_client().await;
        let contributors = client.contributors(REPO_REF.deref()).await.unwrap();
        for contributor in contributors {
            println!(
                "Contributor {}, avatar_url: {}",
                contributor.name, contributor.avatar_url
            );
        }
    }
}
