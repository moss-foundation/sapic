use std::{path::Path, sync::Arc};

use anyhow::Context as _;
use moss_common::api::{OperationResult, OperationResultExt};
use moss_fs::RemoveOptions;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::operations::{DeleteCollectionInput, DeleteCollectionOutput},
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

        let abs_path: Arc<Path> = collection_entry.path().clone().into();
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

        let collection_store = self.workspace_storage.collection_store();
        let mut txn = self.workspace_storage.begin_write().await?;
        collection_store.delete_collection(&mut txn, abs_path.to_owned().to_path_buf())?;
        txn.commit()?;

        Ok(DeleteCollectionOutput {
            id: collection_entry.id,
            abs_path,
        })
    }
}
