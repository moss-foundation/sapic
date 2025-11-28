use moss_applib::AppRuntime;
use sapic_ipc::ValidationResultExt;
use validator::Validate;

use crate::{Workspace, models::operations::BatchUpdateEnvironmentGroupInput};

impl<R: AppRuntime> Workspace<R> {
    pub async fn batch_update_environment_group(
        &self,
        ctx: &R::AsyncContext,
        input: BatchUpdateEnvironmentGroupInput,
    ) -> joinerror::Result<()> {
        input.validate().join_err_bare()?;

        for item in input.items {
            self.environment_service
                .update_environment_group(ctx, item)
                .await?;
        }

        Ok(())
    }
}
