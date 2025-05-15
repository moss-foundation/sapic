use anyhow::Context as _;
use moss_collection::collection::Collection;
use moss_common::{
    api::{OperationError, OperationResult},
    models::primitives::Identifier,
};
use moss_fs::utils::encode_name;
use moss_storage::workspace_storage::entities::collection_store_entities::CollectionEntity;
use std::{path::Path, sync::Arc};
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::operations::{CreateCollectionInput, CreateCollectionOutput},
    workspace::{COLLECTIONS_DIR, CollectionEntry, Workspace},
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn create_collection(
        &self,
        input: CreateCollectionInput,
    ) -> OperationResult<CreateCollectionOutput> {
        input.validate()?;

        let encoded_name = encode_name(&input.name);
        let abs_path: Arc<Path> = self
            .abs_path()
            .join(COLLECTIONS_DIR)
            .join(&encoded_name)
            .into();

        if abs_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.name,
                path: abs_path.to_path_buf(),
            });
        }

        let collections = self
            .collections()
            .await
            .context("Failed to get collections")?;

        self.fs
            .create_dir(&abs_path)
            .await
            .context("Failed to create the collection directory")?;

        let collection = Collection::new(
            abs_path.to_path_buf(), // FIXME: change to Arc<Path> in Collection::new
            self.fs.clone(),
            self.next_collection_entry_id.clone(),
        )?;

        {
            let collection_store = self.workspace_storage.collection_store();
            let mut txn = self.workspace_storage.begin_write().await?;
            collection_store.upsert_collection(
                &mut txn,
                // NOTE: We’re using an absolute path here to keep the option open for implementing functionality that stores collections outside the workspace folder.
                abs_path.to_path_buf(),
                CollectionEntity { order: None },
            )?;
            txn.commit()?;
        }

        let id = Identifier::new(&self.next_collection_id);
        {
            let mut collections_lock = collections.write().await;
            collections_lock.insert(
                id,
                CollectionEntry {
                    id,
                    name: encoded_name.to_owned(),
                    display_name: input.name.to_owned(),
                    order: None,
                    inner: collection,
                }
                .into(),
            );
        }

        Ok(CreateCollectionOutput { id, abs_path })
    }
}
