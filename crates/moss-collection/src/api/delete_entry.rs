use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    collection::Collection,
    models::operations::{DeleteEntryInput, DeleteEntryOutput},
    services::DynWorktreeService,
};

impl Collection {
    pub async fn delete_entry(
        &self,
        input: DeleteEntryInput,
    ) -> OperationResult<DeleteEntryOutput> {
        input.validate()?;
        let worktree_service = self.service::<DynWorktreeService>();
        worktree_service.remove_entry(&input.id).await?;

        Ok(DeleteEntryOutput { id: input.id })
    }
}
