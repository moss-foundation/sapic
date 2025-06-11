use crate::{
    collection::Collection,
    models::operations::{DeleteEntryInput, DeleteEntryOutput},
};
use moss_common::api::OperationResult;

impl Collection {
    pub async fn delete_entry(
        &mut self,
        input: DeleteEntryInput,
    ) -> OperationResult<DeleteEntryOutput> {
        let mut worktree = self.worktree().await?.write().await;
        let changes = worktree.remove_entry(input.id).await?;
        Ok(DeleteEntryOutput { changes })
    }
}
