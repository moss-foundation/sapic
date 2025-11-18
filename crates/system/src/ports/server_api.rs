pub mod types;

use async_trait::async_trait;
use sapic_core::context::AnyAsyncContext;

use super::server_api::types::*;

#[async_trait]
pub trait ExtensionApiOperations: Send + Sync {
    async fn list_extensions(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<ExtensionInfo>>;
}

pub trait ServerApiClient: Send + Sync + ExtensionApiOperations {}
