use moss_applib::AppRuntime;
use sapic_ipc::ValidationResultExt;
use validator::Validate;

use crate::{Workspace, models::operations::*};

impl<R: AppRuntime> Workspace<R> {
    pub async fn batch_update_environment(
        &self,
        ctx: &R::AsyncContext,
        input: BatchUpdateEnvironmentInput,
    ) -> joinerror::Result<BatchUpdateEnvironmentOutput> {
        input.validate().join_err_bare()?;

        let mut ids = Vec::new();
        for item_params in input.items {
            let id = item_params.id.clone();
            self.environment_service
                .update_environment(ctx, item_params)
                .await?;

            ids.push(id);
        }

        Ok(BatchUpdateEnvironmentOutput { ids })
    }
}
