use crate::{
    collection::Collection,
    models::operations::{DeleteEntryInput, DeleteEntryOutput},
};
use moss_common::api::OperationResult;

impl Collection {
    pub async fn delete_entry(
        &self,
        input: DeleteEntryInput,
    ) -> OperationResult<DeleteEntryOutput> {
        let worktree = self.worktree().await?;
        let mut worktree_lock = worktree.write().await;

        let changes = worktree_lock.delete_entry_by_virtual_id(input.id).await?;

        Ok(DeleteEntryOutput {
            physical_changes: changes.physical_changes,
            virtual_changes: changes.virtual_changes,
        })
    }
}
