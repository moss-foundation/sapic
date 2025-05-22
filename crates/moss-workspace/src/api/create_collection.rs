use anyhow::Context as _;
use moss_collection::collection::{self, Collection};
use moss_common::{
    api::{OperationError, OperationResult},
    models::primitives::Identifier,
};
use moss_db::primitives::AnyValue;
use moss_storage::{
    storage::operations::PutItem,
    workspace_storage::entities::collection_store_entities::CollectionCacheEntity,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::Runtime as TauriRuntime;
use tokio::sync::RwLock;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dirs,
    models::operations::{CreateCollectionInput, CreateCollectionOutput},
    storage::segments::COLLECTION_SEGKEY,
    workspace::{CollectionItem, Workspace},
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn create_collection(
        &self,
        input: CreateCollectionInput,
    ) -> OperationResult<CreateCollectionOutput> {
        input.validate()?;

        let id = Uuid::new_v4();
        let id_str = id.to_string();

        let path = PathBuf::from(dirs::COLLECTIONS_DIR).join(&id_str);
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

        let name = input.name.to_owned();
        let collection = Collection::create(
            &abs_path,
            self.fs.clone(),
            self.next_collection_entry_id.clone(),
            collection::CreateParams {
                name: Some(name.clone()),
            },
        )
        .await
        .map_err(|e| OperationError::Internal(e.to_string()))?;

        {
            let mut collections_lock = collections.write().await;
            collections_lock.insert(
                id,
                Arc::new(RwLock::new(CollectionItem {
                    id,
                    name,
                    order: None,
                    inner: collection,
                })),
            );
        }

        {
            let key = COLLECTION_SEGKEY.join(&id_str);
            let value = AnyValue::serialize(&CollectionCacheEntity {
                order: None,
                external_abs_path: None,
            })?;
            PutItem::put(self.workspace_storage.item_store().as_ref(), key, value)?;
        }

        Ok(CreateCollectionOutput { id, abs_path })
    }
}
