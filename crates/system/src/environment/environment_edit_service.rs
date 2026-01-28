use crate::environment::{EnvironmentEditBackend, EnvironmentEditParams};
use joinerror::ResultExt;
use sapic_core::context::AnyAsyncContext;
use std::sync::Arc;

#[derive(Clone)]
pub struct EnvironmentEditService {
    backend: Arc<dyn EnvironmentEditBackend>,
}

impl EnvironmentEditService {
    pub fn new(backend: Arc<dyn EnvironmentEditBackend>) -> Self {
        Self { backend }
    }
    pub async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: EnvironmentEditParams,
    ) -> joinerror::Result<()> {
        self.backend
            .edit(ctx, params)
            .await
            .join_err::<()>("failed to edit environment manifest")?;

        Ok(())
    }
}
