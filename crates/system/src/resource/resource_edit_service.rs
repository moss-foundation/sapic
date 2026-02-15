use sapic_core::context::AnyAsyncContext;
use std::sync::Arc;

use crate::resource::{ResourceEditBackend, ResourceEditParams};

pub struct ResourceEditService {
    backend: Arc<dyn ResourceEditBackend>,
}

impl ResourceEditService {
    pub fn new(backend: Arc<dyn ResourceEditBackend>) -> Self {
        Self { backend }
    }

    pub async fn edit<'a>(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ResourceEditParams<'a>,
    ) -> joinerror::Result<()> {
        self.backend.edit(ctx, params).await?;

        Ok(())
    }
}
