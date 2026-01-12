use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_core::context::AnyAsyncContext;
use sapic_system::environment::{
    EnvironmentInitializeOp, environment_service::CreateEnvironmentItemParams,
};
use std::{path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct WelcomeWindowEnvironmentOps {
    initialize_environment: Arc<dyn EnvironmentInitializeOp>,
}

impl WelcomeWindowEnvironmentOps {
    pub fn new(initialize_environment: Arc<dyn EnvironmentInitializeOp>) -> Self {
        Self {
            initialize_environment,
        }
    }

    pub async fn initialize(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<PathBuf> {
        self.initialize_environment
            .initialize(ctx, workspace_id, params)
            .await
    }
}
