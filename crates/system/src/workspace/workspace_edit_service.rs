use crate::workspace::{WorkspaceEditBackend, WorkspaceEditOp, WorkspaceEditParams};
use async_trait::async_trait;
use joinerror::ResultExt;
use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_core::context::AnyAsyncContext;
use std::sync::Arc;

pub struct WorkspaceEditService {
    // INFO: It might be worth to add a model for workspace caching mechanism
    // in the future so it doesn't have to be read every time before applying patches.
    backend: Arc<dyn WorkspaceEditBackend>,
}

impl WorkspaceEditService {
    pub fn new(backend: Arc<dyn WorkspaceEditBackend>) -> Self {
        Self { backend }
    }
}

#[async_trait]
impl WorkspaceEditOp for WorkspaceEditService {
    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &WorkspaceId,
        params: WorkspaceEditParams,
    ) -> joinerror::Result<()> {
        self.backend
            .edit(ctx, id, params)
            .await
            .join_err::<()>("failed to edit workspace")?;

        Ok(())
    }
}
