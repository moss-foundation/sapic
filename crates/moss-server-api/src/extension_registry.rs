use async_trait::async_trait;
use joinerror::ResultExt;
use moss_applib::{AppRuntime, context, context::ContextResultExt};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionInfo {
    id: String,
    external_id: String,
    name: String,
    authors: Vec<String>,
    description: String,
    repository: String,
    downloads: u64,
    // TODO: If necessary we can parse it with chrono
    created_at: String,
    updated_at: String,
    latest_version: String,
}

#[derive(Debug, Deserialize)]
pub struct ListExtensionsResponse {
    extensions: Vec<ExtensionInfo>,
}

#[async_trait]
pub trait ListExtensionsApiReq<R: AppRuntime>: Send + Sync {
    async fn list_extensions(&self, ctx: &R::Context) -> joinerror::Result<ListExtensionsResponse>;
}

#[derive(Clone)]
pub struct ExtensionsRegistryApiClient {
    base_url: Arc<String>,
    client: HttpClient,
}

impl ExtensionsRegistryApiClient {
    pub fn new(client: HttpClient, base_url: String) -> Self {
        Self {
            base_url: base_url.into(),
            client,
        }
    }

    pub fn base_url(&self) -> Arc<String> {
        self.base_url.clone()
    }
}

#[async_trait]
impl<R: AppRuntime> ListExtensionsApiReq<R> for ExtensionsRegistryApiClient {
    async fn list_extensions(&self, ctx: &R::Context) -> joinerror::Result<ListExtensionsResponse> {
        context::abortable(ctx, async {
            let resp = self
                .client
                .get(format!("{}/extensions", self.base_url))
                .send()
                .await
                .join_err::<()>("failed to list extensions")?;

            if !resp.status().is_success() {
                let error_text = resp.text().await?;
                return Err(joinerror::Error::new::<()>(error_text));
            }

            resp.json()
                .await
                .join_err::<()>("failed to parse list extensions response")
        })
        .await
        .join_err_bare()
    }
}
