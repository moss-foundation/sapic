use std::sync::Arc;

use sapic_core::context::AnyAsyncContext;

use crate::ports::server_api::{ExtensionApiOperations, types::ExtensionInfo};

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
}
