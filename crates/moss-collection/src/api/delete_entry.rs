use crate::{
    collection::Collection,
    models::operations::{DeleteEntryInput, DeleteEntryOutput},
    storage::segments,
};
use moss_common::api::OperationResult;
use moss_storage::storage::operations::TransactionalRemoveItem;
use validator::Validate;

impl Collection {
    pub async fn delete_entry(
        &mut self,
        input: DeleteEntryInput,
    ) -> OperationResult<DeleteEntryOutput> {
        input.validate()?;

        self.worktree().remove_entry(input.id, &input.path).await?;

        let mut txn = self.storage().begin_write()?;
        let store = self.storage().resource_store();

        {
            let segkey = segments::segkey_entry_order(&input.id.to_string());
            TransactionalRemoveItem::remove(store.as_ref(), &mut txn, segkey)?;
        }

        // {
        //     let segkey = segments::segkey_entry_expanded(&input.id.to_string());
        //     TransactionalRemoveItem::remove(store.as_ref(), &mut txn, segkey)?;
        // }

        txn.commit()?;

        Ok(DeleteEntryOutput {})
    }
}
