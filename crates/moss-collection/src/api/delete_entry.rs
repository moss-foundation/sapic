use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    collection::Collection,
    models::operations::{DeleteEntryInput, DeleteEntryOutput},
};

impl<R: AppRuntime> Collection<R> {
    pub async fn delete_entry(
        &self,
        ctx: &R::AsyncContext,
        input: DeleteEntryInput,
    ) -> joinerror::Result<DeleteEntryOutput> {
        input.validate().join_err_bare()?;
        self.worktree.remove_entry(ctx, &input.id).await?;

        Ok(DeleteEntryOutput { id: input.id })
    }
}
