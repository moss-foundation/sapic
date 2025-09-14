use moss_applib::{AppRuntime, errors::ValidationResultExt};
use validator::Validate;

use crate::{
    api::BatchUpdateCollectionOp,
    models::operations::{BatchUpdateProjectInput, BatchUpdateProjectOutput},
    workspace::Workspace,
};

impl<R: AppRuntime> BatchUpdateCollectionOp<R> for Workspace<R> {
    async fn batch_update_project(
        &self,
        ctx: &R::AsyncContext,
        input: BatchUpdateProjectInput,
    ) -> joinerror::Result<BatchUpdateProjectOutput> {
        input.validate().join_err_bare()?;

        let mut ids = Vec::new();
        for item in input.items {
            let id = item.id.clone();
            self.project_service.update_project(ctx, &id, item).await?;

            ids.push(id);
        }

        Ok(BatchUpdateProjectOutput { ids })
    }
}
