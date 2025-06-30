use crate::{
    collection::Collection,
    models::operations::{DeleteEntryInput, DeleteEntryOutput},
    services::worktree_service::WorktreeService,
};
use moss_common::api::OperationResult;
use validator::Validate;

impl Collection {
    pub async fn delete_entry(
        &mut self,
        input: DeleteEntryInput,
    ) -> OperationResult<DeleteEntryOutput> {
        input.validate()?;

        let worktree_service = self.service::<WorktreeService>();
        worktree_service.remove_entry(input.id).await?;

        Ok(DeleteEntryOutput { id: input.id })
    }
}
