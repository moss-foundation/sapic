use anyhow::Context as _;
use moss_common::api::OperationResult;
use moss_fs::RemoveOptions;
use tauri::Runtime as TauriRuntime;

use crate::{models::operations::DeleteCollectionInput, workspace::Workspace};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn delete_collection(&self, input: DeleteCollectionInput) -> OperationResult<()> {
        let collections = self.collections().await?;

        let mut collections_lock = collections.write().await;
        let (collection, _) = collections_lock
            .remove(input.key)
            .context("Failed to remove the collection")?;

        let collection_path = collection.path();
        let collection_relative_path = collection_path.strip_prefix(&self.abs_path).unwrap();

        // TODO: If any of the following operations fail, we should place the task
        // in the dead queue and attempt the deletion later.

        let collection_store = self.workspace_storage.collection_store();
        let mut txn = self.workspace_storage.begin_write().await?;
        collection_store.delete_collection(&mut txn, collection_relative_path.to_owned())?;

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
