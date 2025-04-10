use anyhow::Context as _;
use moss_fs::RemoveOptions;

use crate::{
    models::operations::DeleteCollectionInput,
    workspace::{OperationError, Workspace},
};

impl Workspace {
    pub async fn delete_collection(
        &self,
        input: DeleteCollectionInput,
    ) -> Result<(), OperationError> {
        let collections = self.collections().await?;

        let mut collections_lock = collections.write().await;
        let (collection, _) = collections_lock
            .remove(input.key)
            .context("Failed to remove the collection")?;

        let collection_path = collection.path();
        let collection_relative_path = collection_path.strip_prefix(&self.path).unwrap();
        let collection_store = self.state_db_manager().collection_store();

        // TODO: If any of the following operations fail, we should place the task
        // in the dead queue and attempt the deletion later.

        let (mut txn, table) = collection_store.begin_write()?;
        let table_key = collection_relative_path.to_string_lossy().to_string();
        table.remove(&mut txn, table_key)?;

        if !collection_path.exists() {
            // TODO: logging if the folder has already been removed from the filesystem
            return Ok(txn.commit()?);
        }

        self.fs
            .remove_dir(
                &collection_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        Ok(txn.commit()?)
    }
}
