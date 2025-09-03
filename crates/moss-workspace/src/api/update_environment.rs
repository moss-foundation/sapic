use moss_applib::{AppRuntime, errors::ValidationResultExt};
use validator::Validate;

use crate::{
    models::operations::{UpdateEnvironmentInput, UpdateEnvironmentOutput},
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn update_environment(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateEnvironmentInput,
    ) -> joinerror::Result<UpdateEnvironmentOutput> {
        input.validate().join_err_bare()?;

        let id = input.inner.id.clone();
        self.environment_service
            .update_environment(ctx, input.inner)
            .await?;

        Ok(UpdateEnvironmentOutput { id })
    }
}
