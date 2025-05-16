use anyhow::Context as _;
use moss_collection::collection::Collection;
use moss_common::{
    api::{OperationError, OperationResult},
    models::primitives::Identifier,
};
use moss_db::primitives::AnyValue;
use moss_fs::utils::encode_name;
use moss_storage::{
    storage::operations::PutItem,
    workspace_storage::entities::collection_store_entities::CollectionEntity,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::operations::{CreateCollectionInput, CreateCollectionOutput},
    storage::segments::COLLECTION_SEGKEY,
    workspace::{COLLECTIONS_DIR, CollectionEntry, Workspace},
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn create_collection(
        &self,
        input: CreateCollectionInput,
    ) -> OperationResult<CreateCollectionOutput> {
        input.validate()?;

        let encoded_name = encode_name(&input.name);
        let path = PathBuf::from(COLLECTIONS_DIR).join(&encoded_name);
        let abs_path: Arc<Path> = self.abs_path().join(path).into();

        if abs_path.exists() {
            return Err(OperationError::AlreadyExists(
                abs_path.to_string_lossy().to_string(),
            ));
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

        {
            // NOTE:
            // This is still an open question. Here’s what I’m thinking:
            // It makes sense to add an `is_external` field to the `Input` structure,
            // it would signal that the collection being created is located outside the
            // workspace folder.

            let key = COLLECTION_SEGKEY.join(&encoded_name);
            let value = AnyValue::serialize(&CollectionEntity {
                order: None,
                external_abs_path: None,
            })?;
            PutItem::put(self.workspace_storage.item_store().as_ref(), key, value)?;
        }

        Ok(CreateCollectionOutput { id, abs_path })
    }
}
