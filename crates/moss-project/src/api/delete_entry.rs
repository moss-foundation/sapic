use moss_applib::{AppRuntime, errors::ValidationResultExt};
use validator::Validate;

use crate::{
    models::operations::{DeleteEntryInput, DeleteEntryOutput},
    project::Project,
};

impl<R: AppRuntime> Project<R> {
    pub async fn delete_entry(
        &self,
        ctx: &R::AsyncContext,
        input: DeleteEntryInput,
    ) -> joinerror::Result<DeleteEntryOutput> {
        input.validate().join_err_bare()?;
        self.worktree().await.remove_entry(ctx, &input.id).await?;

        Ok(DeleteEntryOutput { id: input.id })
    }
}
