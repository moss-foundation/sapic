use moss_common::api::OperationResult;

use crate::{
    collection::Collection,
    models::operations::{UpdateEntryInput, UpdateEntryOutput},
};

impl Collection {
    pub async fn update_entry(
        &self,
        input: UpdateEntryInput,
    ) -> OperationResult<UpdateEntryOutput> {
        let workspace = self.worktree().await?;
        let mut worktree_lock = workspace.write().await;
        let UpdateEntryInput {
            id,
            name,
            classification,
            specification,
            protocol,
            order,
        } = input;

        let changes = worktree_lock
            .update_entry_by_virtual_id(id, name, classification, specification, protocol, order)
            .await?;

        Ok(UpdateEntryOutput {
            physical_changes: changes.physical_changes,
            virtual_changes: changes.virtual_changes,
        })
    }
}
