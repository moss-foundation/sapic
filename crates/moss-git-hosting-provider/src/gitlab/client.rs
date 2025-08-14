use crate::{
    GitAuthProvider, GitHostingProvider,
    common::SSHAuthAgent,
    constants::GITLAB_API_URL,
    gitlab::response::{AvatarResponse, ContributorsResponse},
    models::types::{Contributor, RepositoryInfo},
};
use async_trait::async_trait;
use joinerror::OptionExt;
use moss_git::GitAuthAgent;
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION},
};
use std::sync::Arc;
use url::Url;

use crate::{
    common::GitUrl,
    gitlab::{
        auth::GitLabAuthAgent,
        response::{RepositoryResponse, UserResponse},
    },
    models::types::UserInfo,
};

const CONTENT_TYPE: &'static str = "application/json";

// FIXME: Support self-hosted GitLab domains
pub struct GitLabClient {
    client: Client,
    #[allow(dead_code)]
    client_auth_agent: Arc<GitLabAuthAgent>,
    #[allow(dead_code)]
    ssh_auth_agent: Option<Arc<dyn SSHAuthAgent>>,
}

impl GitLabClient {
    pub fn new(
        client: Client,
        client_auth_agent: Arc<GitLabAuthAgent>,
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

unsafe impl Send for GitLabClient {}
unsafe impl Sync for GitLabClient {}

impl GitAuthProvider for GitLabClient {
    fn git_auth_agent(&self) -> Arc<dyn GitAuthAgent> {
        self.client_auth_agent.clone()
    }
}

// TODO: Handle authentication expiration and reauthentication
#[async_trait]
impl GitHostingProvider for GitLabClient {
    fn name(&self) -> String {
        "GitLab".to_string()
    }
    fn base_url(&self) -> Url {
        Url::parse("https://gitlab.com").unwrap()
    }

    async fn current_user(&self) -> joinerror::Result<UserInfo> {
        let access_token = self
            .client_auth_agent
            .clone()
            .access_token()
            .ok_or_join_err::<()>("gitlab is not logged in yet")?;
        let user_response: UserResponse = self
            .client
            .get(format!("{GITLAB_API_URL}/user"))
            .header(ACCEPT, CONTENT_TYPE)
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(UserInfo {
            name: user_response.username,
            email: user_response.commit_email,
        })
    }

    async fn contributors(&self, repo_ref: &GitUrl) -> joinerror::Result<Vec<Contributor>> {
        let repo_url = format!("{}/{}", &repo_ref.owner, &repo_ref.name);
        let encoded_url = urlencoding::encode(&repo_url);

        let access_token = self
            .client_auth_agent
            .clone()
            .access_token()
            .ok_or_join_err::<()>("gitlab is not logged in yet")?;

        let contributors_response: ContributorsResponse = self
            .client
            .get(format!(
                "{GITLAB_API_URL}/projects/{encoded_url}/repository/contributors"
            ))
            .header(ACCEPT, CONTENT_TYPE)
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        // FIXME: Is there a better strategy?
        // Gitlab contributor endpoint only provides the contributor email
        // We will need to fetch their avatar separately

        let mut list = Vec::new();
        for item in contributors_response.items {
            let name = item.name;

            let email = item.email;

            let avatar_response: AvatarResponse = self
                .client
                .get(format!("{GITLAB_API_URL}/avatar"))
                .query(&[("email", &email)])
                .send()
                .await?
                .json()
                .await?;

            let avatar_url = avatar_response.avatar_url;

            list.push(Contributor { name, avatar_url });
        }
        Ok(list)
    }

    async fn repository_info(&self, repo_ref: &GitUrl) -> joinerror::Result<RepositoryInfo> {
        let repo_url = format!("{}/{}", &repo_ref.owner, &repo_ref.name);
        let encoded_url = urlencoding::encode(&repo_url);

        let access_token = self
            .client_auth_agent
            .clone()
            .access_token()
            .ok_or_join_err::<()>("gitlab is not logged in yet")?;

        let repository_response: RepositoryResponse = self
            .client
            .get(format!("{GITLAB_API_URL}/projects/{encoded_url}"))
            .header(ACCEPT, CONTENT_TYPE)
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(RepositoryInfo {
            created_at: repository_response.created_at,
            updated_at: repository_response.updated_at,
            owner: repository_response.owner.username,
        })
    }
}

// API related tests are skipped during CI, since it requires setting up refresh token in envvar
// Set `GITLAB_REFRESH_TOKEN = {}` in /.env

#[cfg(test)]
mod tests {
    use std::{ops::Deref, sync::LazyLock};
    // FIXME: Rewrite the tests
    use moss_keyring::KeyringClientImpl;

    use crate::{
        common::ssh_auth_agent::SSHAuthAgentImpl,
        envvar_keys::{GITLAB_CLIENT_ID, GITLAB_CLIENT_SECRET},
    };

    use super::*;

    static REPO_REF: LazyLock<GitUrl> = LazyLock::new(|| GitUrl {
        domain: "gitlab.com".to_string(),
        owner: "brutusyhy".to_string(),
        name: "test-public-repo".to_string(),
    });

    async fn create_test_client() -> Arc<GitLabClient> {
        dotenv::dotenv().ok();

        let reqwest_client = reqwest::ClientBuilder::new()
            .user_agent("SAPIC")
            .build()
            .unwrap();

        let keyring_client = Arc::new(KeyringClientImpl::new());
        let auth_agent = Arc::new(GitLabAuthAgent::new(
            keyring_client,
            dotenv::var(GITLAB_CLIENT_ID).unwrap(),
            dotenv::var(GITLAB_CLIENT_SECRET).unwrap(),
        ));

        let client = Arc::new(GitLabClient::new(
            reqwest_client,
            auth_agent,
            None as Option<SSHAuthAgentImpl>,
        ));

        client.login().await.unwrap();
        client
    }

    #[tokio::test]
    async fn gitlab_client_name() {
        let client = create_test_client().await;

        assert_eq!(client.name(), "GitLab");
    }

    #[tokio::test]
    async fn gitlab_client_base_url() {
        let client = create_test_client().await;

        let expected_url = Url::parse("https://gitlab.com").unwrap();

        assert_eq!(client.base_url(), expected_url);
    }

    #[ignore]
    #[tokio::test]
    async fn gitlab_client_current_user() {
        let client = create_test_client().await;
        let user_info = client.current_user().await.unwrap();
        println!("{:?}", user_info);
    }
    #[ignore]
    #[tokio::test]
    async fn gitlab_client_repo_info() {
        let client = create_test_client().await;
        let repo_info = client.repository_info(REPO_REF.deref()).await.unwrap();
        println!("{:?}", repo_info);
    }

    #[ignore]
    #[tokio::test]
    async fn gitlab_client_contributors() {
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
