use anyhow::Context as _;
use moss_collection::collection::{Collection, CollectionCache};
use moss_fs::utils::encode_directory_name;
use std::path::PathBuf;
use validator::Validate;

use crate::{
    models::{
        entities::CollectionEntity,
        operations::{CreateCollectionInput, CreateCollectionOutput},
    },
    workspace::{OperationError, Workspace, COLLECTIONS_DIR},
};

impl Workspace {
    pub async fn create_collection(
        &self,
        input: CreateCollectionInput,
    ) -> Result<CreateCollectionOutput, OperationError> {
        input.validate()?;

        // workspace_path/encoded_collection_folder
        let relative_path = PathBuf::from(COLLECTIONS_DIR).join(encode_directory_name(&input.name));
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

        let collection_store = self.state_db_manager()?.collection_store();
        let (mut txn, table) = collection_store.begin_write()?;

        table.insert(
            &mut txn,
            relative_path.to_string_lossy().to_string(),
            &CollectionEntity { order: None },
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
