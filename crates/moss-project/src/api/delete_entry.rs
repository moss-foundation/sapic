use moss_applib::{AppRuntime, errors::ValidationResultExt};
use validator::Validate;

use crate::{
    models::operations::{DeleteResourceInput, DeleteResourceOutput},
    project::Project,
};

impl<R: AppRuntime> Project<R> {
    pub async fn delete_entry(
        &self,
        ctx: &R::AsyncContext,
        input: DeleteResourceInput,
    ) -> joinerror::Result<DeleteResourceOutput> {
        input.validate().join_err_bare()?;
        self.worktree().await.remove_entry(ctx, &input.id).await?;

        Ok(DeleteResourceOutput { id: input.id })
    }
}
