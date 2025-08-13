use async_trait::async_trait;
use moss_git::GitAuthAgent;
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION},
};
use std::sync::Arc;
use url::Url;

use crate::{
    GitAuthProvider, GitHostingProvider,
    common::SSHAuthAgent,
    constants::GITLAB_API_URL,
    gitlab::response::{AvatarResponse, ContributorsResponse},
    models::types::{Contributor, RepositoryInfo},
};

use crate::{
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
}

unsafe impl Send for GitLabClient {}
unsafe impl Sync for GitLabClient {}

impl GitAuthProvider for GitLabClient {
    fn git_auth_agent(&self) -> Arc<dyn GitAuthAgent> {
        self.client_auth_agent.clone()
    }
}

#[async_trait]
impl GitHostingProvider for GitLabClient {
    fn name(&self) -> String {
        "GitLab".to_string()
    }
    fn base_url(&self) -> Url {
        Url::parse("https://gitlab.com").unwrap()
    }

    async fn current_user(&self) -> joinerror::Result<UserInfo> {
        let access_token = self.client_auth_agent.clone().access_token().await?;

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

    async fn contributors(&self, repo_url: &str) -> joinerror::Result<Vec<Contributor>> {
        let access_token = self.client_auth_agent.clone().access_token().await?;

        let encoded_url = urlencoding::encode(repo_url);

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

    async fn repository_info(&self, repo_url: &str) -> joinerror::Result<RepositoryInfo> {
        let access_token = self.client_auth_agent.clone().access_token().await?;

        let encoded_url = urlencoding::encode(repo_url);

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

#[cfg(test)]
mod tests {
    // FIXME: Rewrite the tests
    // use anyhow::Result;
    // use git2::RemoteCallbacks;
    //
    // use super::*;
    //
    // const REPO_URL: &'static str = "brutusyhy/test-public-repo";
    //
    // struct DummyGitLabAuthAgent;
    //
    // impl GitAuthAgent for DummyGitLabAuthAgent {
    //     fn generate_callback<'a>(&'a self, _cb: &mut RemoteCallbacks<'a>) -> Result<()> {
    //         Ok(())
    //     }
    // }
    // impl GitLabAuthAgent for DummyGitLabAuthAgent {
    //     async fn access_token(&self) -> joinerror::Result<String> {
    //         Ok("".to_string())
    //     }
    // }
    //
    // struct DummySSHAuthAgent;
    //
    // impl GitAuthAgent for DummySSHAuthAgent {
    //     fn generate_callback<'a>(&'a self, _cb: &mut RemoteCallbacks<'a>) -> Result<()> {
    //         Ok(())
    //     }
    // }
    // impl SSHAuthAgent for DummySSHAuthAgent {}
    //
    // #[test]
    // fn gitlab_client_name() {
    //     let client_auth_agent = DummyGitLabAuthAgent;
    //     let ssh_auth_agent: Option<DummySSHAuthAgent> = None;
    //     let reqwest_client = reqwest::ClientBuilder::new()
    //         .user_agent("SAPIC")
    //         .build()
    //         .unwrap();
    //     let client = GitLabClient::new(reqwest_client, client_auth_agent, ssh_auth_agent);
    //
    //     assert_eq!(client.name(), "GitLab");
    // }
    //
    // #[test]
    // fn gitlab_client_base_url() {
    //     let client_auth_agent = DummyGitLabAuthAgent;
    //     let ssh_auth_agent: Option<DummySSHAuthAgent> = None;
    //     let reqwest_client = reqwest::ClientBuilder::new()
    //         .user_agent("SAPIC")
    //         .build()
    //         .unwrap();
    //
    //     let client = GitLabClient::new(reqwest_client, client_auth_agent, ssh_auth_agent);
    //     let expected_url = Url::parse("https://gitlab.com").unwrap();
    //
    //     assert_eq!(client.base_url(), expected_url);
    // }
    //
    // #[tokio::test]
    // async fn gitlab_client_contributors() {
    //     let client_auth_agent = DummyGitLabAuthAgent;
    //     let ssh_auth_agent: Option<DummySSHAuthAgent> = None;
    //     let reqwest_client = reqwest::ClientBuilder::new()
    //         .user_agent("SAPIC")
    //         .build()
    //         .unwrap();
    //     let client = GitLabClient::new(reqwest_client, client_auth_agent, ssh_auth_agent);
    //     let contributors = client.contributors(REPO_URL).await.unwrap();
    //     for contributor in contributors {
    //         println!(
    //             "Contributor {}, avatar_url: {}",
    //             contributor.name, contributor.avatar_url
    //         );
    //     }
    // }
    //
    // #[ignore]
    // #[test]
    // fn manual_gitlab_client_with_ssh_auth_agent() {
    //     let client_auth_agent = DummyGitLabAuthAgent;
    //     let ssh_agent = DummySSHAuthAgent;
    //     let reqwest_client = reqwest::ClientBuilder::new()
    //         .user_agent("SAPIC")
    //         .build()
    //         .unwrap();
    //     let client = GitLabClient::new(reqwest_client, client_auth_agent, Some(ssh_agent));
    //
    //     assert_eq!(client.name(), "GitLab");
    //     let expected_url = Url::parse("https://gitlab.com").unwrap();
    //     assert_eq!(client.base_url(), expected_url);
    // }
}
