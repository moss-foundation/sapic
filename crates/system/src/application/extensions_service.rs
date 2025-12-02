use std::sync::Arc;

use sapic_base::extension::types::ExtensionInfo;
use sapic_core::context::AnyAsyncContext;

use crate::ports::server_api::ExtensionApiOperations;

pub struct ExtensionsApiService {
    client: Arc<dyn ExtensionApiOperations>,
}

impl ExtensionsApiService {
    pub fn new(client: Arc<dyn ExtensionApiOperations>) -> Self {
        Self { client }
    }

    pub async fn list_extensions(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<ExtensionInfo>> {
        self.client.list_extensions(ctx).await
    }

    pub async fn download_extension(
        &self,
        ctx: &dyn AnyAsyncContext,
        extension_id: &str,
        version: &str,
    ) -> joinerror::Result<(Vec<u8>, String)> {
        self.client
            .download_extension(ctx, extension_id, version)
            .await
    }
}
