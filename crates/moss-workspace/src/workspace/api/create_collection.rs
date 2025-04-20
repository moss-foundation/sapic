use anyhow::Context as _;
use moss_collection::collection::{Collection, CollectionCache};
use moss_common::api::{OperationError, OperationResult};
use moss_fs::utils::encode_name;
use moss_storage::workspace_storage::entities::collection_store_entities::CollectionEntity;
use std::path::PathBuf;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::operations::{CreateCollectionInput, CreateCollectionOutput},
    workspace::{Workspace, COLLECTIONS_DIR},
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn create_collection(
        &self,
        input: CreateCollectionInput,
    ) -> OperationResult<CreateCollectionOutput> {
        input.validate()?;

        // workspace_path/encoded_collection_folder
        let relative_path = PathBuf::from(COLLECTIONS_DIR).join(encode_name(&input.name));
        let full_path = self.path().join(&relative_path);

        if full_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.name,
                path: full_path,
            });
        }

        let collections = self
            .collections()
            .await
            .context("Failed to get collections")?;

        let collection_store = self.workspace_storage.collection_store();
        let mut txn = self.workspace_storage.begin_write().await?;

        collection_store.create_collection(
            &mut txn,
            relative_path.to_owned(),
            CollectionEntity { order: None },
        )?;

        self.fs
            .create_dir(&full_path)
            .await
            .context("Failed to create the collection directory")?;

        let collection = Collection::new(
            full_path.clone(),
            self.fs.clone(),
            self.indexer_handle.clone(),
        )?;
        let metadata = CollectionCache {
            name: input.name.clone(),
            order: None,
        };

        let collection_key = {
            let mut collections_lock = collections.write().await;
            collections_lock.insert((collection, metadata))
        };

        txn.commit()?;

        Ok(CreateCollectionOutput {
            key: collection_key,
            path: full_path,
        })
    }
}
