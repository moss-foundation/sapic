use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;
use moss_common::api::OperationResult;
use moss_fs::RemoveOptions;
use moss_storage::storage::operations::RemoveItem;
use tauri::Runtime as TauriRuntime;

use crate::{
    dirs,
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

        let id_str = input.id.to_string();
        let path = PathBuf::from(dirs::COLLECTIONS_DIR).join(&id_str);
        let abs_path: Arc<Path> = self.absolutize(path).into();

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

        let removed_id = {
            let mut collections_lock = collections.write().await;
            if let Some(v) = collections_lock.remove(&input.id) {
                let lock = v.read().await;
                let id = lock.id;
                Some(id)
            } else {
                None
            }
        };

        {
            let key = COLLECTION_SEGKEY.join(&id_str);
            RemoveItem::remove(self.workspace_storage.item_store().as_ref(), key)?;
        }

        Ok(DeleteCollectionOutput {
            id: removed_id.unwrap_or(uuid::Uuid::nil()),
            abs_path,
        })
    }
}
