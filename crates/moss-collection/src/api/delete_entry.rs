use crate::{
    collection::Collection,
    models::operations::{DeleteEntryInput, DeleteEntryOutput},
};
use moss_common::api::OperationResult;
use validator::Validate;

impl Collection {
    pub async fn delete_entry(
        &mut self,
        input: DeleteEntryInput,
    ) -> OperationResult<DeleteEntryOutput> {
        input.validate()?;

        self.worktree().remove_entry(&input.path).await?;

        // TODO: db operations

        Ok(DeleteEntryOutput {})
    }
}
