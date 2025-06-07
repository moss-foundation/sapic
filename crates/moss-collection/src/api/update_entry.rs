use moss_common::api::OperationResult;

use crate::{
    collection::Collection,
    models::operations::{UpdateEntryInput, UpdateEntryOutput},
};

impl Collection {
    pub async fn update_entry(
        &mut self,
        input: UpdateEntryInput,
    ) -> OperationResult<UpdateEntryOutput> {
        // let workspace = self.worktree_mut().await?;

        // let UpdateEntryInput {
        //     id,
        //     name,
        //     classification,
        //     specification,
        //     protocol,
        //     order,
        // } = input;

        // let changes = workspace
        //     .update_entry_by_virtual_id(id, name, classification, specification, protocol, order)
        //     .await?;

        // Ok(UpdateEntryOutput {
        //     physical_changes: changes.physical_changes,
        //     virtual_changes: changes.virtual_changes,
        // })

        todo!()
    }
}
