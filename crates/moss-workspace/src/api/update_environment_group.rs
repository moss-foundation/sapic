use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use validator::Validate;

use crate::{Workspace, models::operations::UpdateEnvironmentGroupInput};

impl<R: AppRuntime> Workspace<R> {
    pub async fn update_environment_group(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateEnvironmentGroupInput,
    ) -> joinerror::Result<()> {
        input.validate().join_err_bare()?;

        self.environment_service
            .update_environment_group(ctx, input.inner)
            .await?;

        Ok(())
    }
}
