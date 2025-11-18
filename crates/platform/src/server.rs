mod types;

use async_trait::async_trait;
use joinerror::ResultExt;
use reqwest::Client as HttpClient;
use sapic_core::context::{AnyAsyncContext, ContextResultExt, abortable};
use sapic_system::ports::server_api::{
    ExtensionApiOperations, ServerApiClient, types::ExtensionInfo,
};

use crate::server::types::ListExtensionsResponse;

pub struct HttpServerApiClient {
    base_url: String,
    client: HttpClient,
}

impl HttpServerApiClient {
    pub fn new(base_url: String, client: HttpClient) -> Self {
        Self { base_url, client }
    }
}

impl ServerApiClient for HttpServerApiClient {}

const LIST_EXTENSIONS_BASE_SEGMENT: &str = "extension-registry";

#[async_trait]
impl ExtensionApiOperations for HttpServerApiClient {
    async fn list_extensions(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<ExtensionInfo>> {
        let response: ListExtensionsResponse = abortable(ctx, async {
            let resp = self
                .client
                .get(format!(
                    "{}/{LIST_EXTENSIONS_BASE_SEGMENT}/extensions",
                    self.base_url
                ))
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
        .join_err_bare()?;

        Ok(response
            .extensions
            .into_iter()
            .map(|response| response.into())
            .collect())
    }
}
