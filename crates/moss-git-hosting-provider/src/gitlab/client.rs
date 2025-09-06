use moss_applib::{
    AppRuntime,
    context::{self, ContextResultExt},
};
use moss_git::url::GitUrl;
use moss_user::AccountSession;
use oauth2::http::header::{ACCEPT, AUTHORIZATION};
use reqwest::Client as HttpClient;

use crate::gitlab::response::{GetContributorsResponse, GetRepositoryResponse, GetUserResponse};

fn api_url(host: &str) -> String {
    format!("https://{host}/api/v4") // TODO: make version configurable?
}

const CONTENT_TYPE: &'static str = "application/json";

#[derive(Clone)]
pub struct GitLabApiClient {
    client: HttpClient,
}

impl GitLabApiClient {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn get_user<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        account_handle: &AccountSession,
    ) -> joinerror::Result<GetUserResponse> {
        let access_token = account_handle.access_token().await?;
        let resp = context::abortable(
            ctx,
            self.client
                .get(format!("{}/user", api_url(&account_handle.host())))
                .header(ACCEPT, CONTENT_TYPE)
                .header(AUTHORIZATION, format!("Bearer {}", access_token))
                .send(),
        )
        .await
        .join_err()?;

        let status = resp.status();
        if status.is_success() {
            Ok(resp.json().await?)
        } else {
            let error_text = resp.text().await?;
            eprintln!("GitLab API Error: Status {}, Body: {}", status, error_text);
            Err(joinerror::Error::new::<()>(error_text))
        }
    }

    pub async fn get_contributors<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        account_handle: &AccountSession,
        url: &GitUrl,
    ) -> joinerror::Result<GetContributorsResponse> {
        let access_token = account_handle.access_token().await?;
        let repo_url = format!("{}/{}", &url.owner, &url.name);
        let encoded_url = urlencoding::encode(&repo_url);

        let resp = context::abortable(
            ctx,
            self.client
                .get(format!(
                    "{}/projects/{}/repository/contributors",
                    api_url(&account_handle.host()),
                    encoded_url
                ))
                .header(ACCEPT, CONTENT_TYPE)
                .header(AUTHORIZATION, format!("Bearer {}", access_token))
                .send(),
        )
        .await
        .join_err()?;

        let status = resp.status();
        if status.is_success() {
            Ok(resp.json().await?)
        } else {
            let error_text = resp.text().await?;
            eprintln!("GitLab API Error: Status {}, Body: {}", status, error_text);
            Err(joinerror::Error::new::<()>(error_text))
        }
    }

    pub async fn get_repository<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        account_handle: &AccountSession,
        url: &GitUrl,
    ) -> joinerror::Result<GetRepositoryResponse> {
        let access_token = account_handle.access_token().await?;
        let repo_url = format!("{}/{}", &url.owner, &url.name);
        let encoded_url = urlencoding::encode(&repo_url);

        let resp = context::abortable(
            ctx,
            self.client
                .get(format!(
                    "{}/projects/{}/repository/contributors",
                    api_url(&account_handle.host()),
                    encoded_url
                ))
                .header(ACCEPT, CONTENT_TYPE)
                .header(AUTHORIZATION, format!("Bearer {}", access_token))
                .send(),
        )
        .await
        .join_err()?;

        let status = resp.status();
        if status.is_success() {
            Ok(resp.json().await?)
        } else {
            let error_text = resp.text().await?;
            eprintln!("GitLab API Error: Status {}, Body: {}", status, error_text);
            Err(joinerror::Error::new::<()>(error_text))
        }
    }
}
