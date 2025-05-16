use std::{path::Path, sync::Arc};

use anyhow::Context as _;
use moss_common::api::{OperationResult, OperationResultExt};
use moss_fs::RemoveOptions;
use moss_storage::storage::operations::RemoveItem;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::operations::{DeleteCollectionInput, DeleteCollectionOutput},
    storage::segments::COLLECTION_SEGKEY,
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn delete_collection(
        &self,
        input: DeleteCollectionInput,
    ) -> OperationResult<DeleteCollectionOutput> {
        let collections = self.collections().await?;
        let collection_entry = collections
            .read()
            .await
            .get(&input.id)
            .context("Collection not found")
            .map_err_as_not_found()?
            .clone();

        let abs_path: Arc<Path> = collection_entry.abs_path().clone().into();
        if abs_path.exists() {
            self.fs
                .remove_dir(
                    &abs_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .context("Failed to delete collection from file system")?;
        }

        {
            let mut collections_lock = collections.write().await;
            collections_lock.remove(&input.id);
        }

        {
            let key = COLLECTION_SEGKEY.join(&collection_entry.name);
            RemoveItem::remove(self.workspace_storage.item_store().as_ref(), key)?;
        }

        Ok(DeleteCollectionOutput {
            id: collection_entry.id,
            abs_path,
        })
    }
}
