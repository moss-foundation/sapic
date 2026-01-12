use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_core::context::AnyAsyncContext;
use sapic_system::environment::{
    EnvironmentCreateOp, environment_service::CreateEnvironmentItemParams,
};
use std::{path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct MainWindowEnvironmentOps {
    create_environment: Arc<dyn EnvironmentCreateOp>,
}

impl MainWindowEnvironmentOps {
    pub fn new(create_environment: Arc<dyn EnvironmentCreateOp>) -> Self {
        Self { create_environment }
    }

    pub async fn create(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<PathBuf> {
        self.create_environment
            .create(ctx, workspace_id, params)
            .await
    }
}
