use crate::{
    collection::Collection,
    models::operations::{DeleteEntryInput, DeleteEntryOutput},
    services::worktree_service::WorktreeService,
};
use moss_common::{NanoId, api::OperationResult};
use validator::Validate;

impl Collection {
    pub async fn delete_entry(
        &self,
        input: DeleteEntryInput,
    ) -> OperationResult<DeleteEntryOutput> {
        input.validate()?;
        let id: NanoId = input.id.clone().into();
        let worktree_service = self.service::<WorktreeService>();
        worktree_service.remove_entry(&id).await?;

        Ok(DeleteEntryOutput { id: input.id })
    }
}
