use moss_applib::AppRuntime;
use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    collection::Collection,
    models::operations::{DeleteEntryInput, DeleteEntryOutput},
    services::DynWorktreeService,
};

impl<R: AppRuntime> Collection<R> {
    pub async fn delete_entry(
        &self,
        ctx: &R::AsyncContext,
        input: DeleteEntryInput,
    ) -> OperationResult<DeleteEntryOutput> {
        input.validate()?;
        let worktree_service = self.service::<DynWorktreeService<R>>();
        worktree_service.remove_entry(ctx, &input.id).await?;

        Ok(DeleteEntryOutput { id: input.id })
    }
}
