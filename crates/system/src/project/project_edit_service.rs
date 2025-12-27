use joinerror::ResultExt;
use sapic_base::project::types::primitives::ProjectId;
use sapic_core::context::AnyAsyncContext;
use std::sync::Arc;

use crate::project::{ProjectConfigEditParams, ProjectEditBackend, ProjectEditParams};

pub struct ProjectEditService {
    backend: Arc<dyn ProjectEditBackend>,
}

impl Clone for ProjectEditService {
    fn clone(&self) -> ProjectEditService {
        Self {
            backend: self.backend.clone(),
        }
    }
}

impl ProjectEditService {
    pub fn new(backend: Arc<dyn ProjectEditBackend>) -> ProjectEditService {
        ProjectEditService { backend }
    }
    pub async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ProjectEditParams,
    ) -> joinerror::Result<()> {
        self.backend
            .edit(ctx, id, params)
            .await
            .join_err::<()>("failed to edit project manifest")?;

        Ok(())
    }

    pub async fn edit_config(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ProjectConfigEditParams,
    ) -> joinerror::Result<()> {
        self.backend
            .edit_config(ctx, id, params)
            .await
            .join_err::<()>("failed to edit project config")?;
        Ok(())
    }
}
