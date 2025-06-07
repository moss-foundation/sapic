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
        let worktree = self.worktree_mut().await?;

        // let changes = worktree.delete_entry_by_virtual_id(input.id).await?;

        // Ok(DeleteEntryOutput {
        //     physical_changes: changes.physical_changes,
        //     virtual_changes: changes.virtual_changes,
        // })

        todo!()
    }
}
