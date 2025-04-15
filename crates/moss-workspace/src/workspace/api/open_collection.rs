use anyhow::Context as _;
use moss_collection::collection::{Collection, CollectionCache};
use tauri::Runtime as TauriRuntime;

use crate::{
    models::operations::{OpenCollectionInput, OpenCollectionOutput},
    workspace::{OperationError, Workspace},
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn open_collection(
        &self,
        input: OpenCollectionInput,
    ) -> Result<OpenCollectionOutput, OperationError> {
        if !input.path.exists() {
            return Err(OperationError::NotFound {
                name: input
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                path: input.path.clone(),
            });
        }

        let collection = Collection::new(
            input.path.clone(),
            self.fs.clone(),
            self.indexer_handle.clone(),
        )?;
        let metadata = CollectionCache {
            name: input
                .path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(), // TODO: decode
            order: None,
        };

        let collections = self
            .collections()
            .await
            .context("Failed to get collections")?;

        let collection_key = {
            let mut collections_lock = collections.write().await;
            collections_lock.insert((collection, metadata))
        };

        Ok(OpenCollectionOutput {
            key: collection_key,
            path: input.path,
        })
    }
}
