use async_trait::async_trait;
use joinerror::ResultExt;
use sapic_base::extension::types::ExtensionInfo;
use sapic_core::context::{self, AnyAsyncContext, ContextResultExt};
use sapic_system::ports::server_api::ExtensionApiOperations;

use super::{HttpServerApiClient, types::ListExtensionsResponse};

const LIST_EXTENSIONS_BASE_SEGMENT: &str = "extension-registry";

#[async_trait]
impl ExtensionApiOperations for HttpServerApiClient {
    async fn list_extensions(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<ExtensionInfo>> {
        let response: ListExtensionsResponse = context::abortable(ctx, async {
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
