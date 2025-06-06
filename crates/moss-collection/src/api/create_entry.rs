use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    collection::Collection,
    models::operations::{CreateEntryInput, CreateEntryOutput},
};

impl Collection {
    pub async fn create_entry(
        &mut self,
        input: CreateEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        input.validate()?;

        // TODO: validate specification

        let content = if let Some(value) = input.specification {
            Some(serde_json::to_vec(&value)?)
        } else {
            None
        };

        let worktree = self.worktree_mut().await?;
        let changes = worktree
            .create_entry(
                input.destination,
                input.order,
                input.protocol,
                content,
                input.classification,
                input.is_dir,
            )
            .await?;

        Ok(CreateEntryOutput {
            physical_changes: changes.physical_changes,
            virtual_changes: changes.virtual_changes,
        })
    }
}
